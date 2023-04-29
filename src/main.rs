use bevy::prelude::*;
use bevy::window::WindowResolution;

mod constants;
use constants::*;
mod textures;
use textures::*;
mod game_state;
use game_state::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin{
            primary_window: Some(Window {
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                title: "Lines".to_string(),
                resizable: false,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .init_resource::<Textures>()
        .init_resource::<GameState>()
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_field)
        .add_startup_system(make_initial_turns)
        .add_system(select_circle)
        .add_system(animate_selected_circle.before(select_circle))
        .run();
}

fn get_cell_size() -> f32 {
    WINDOW_WIDTH / FIELD_SIZE as f32
}

fn get_cell_coords(row: usize, col: usize) -> (f32, f32) {
    let cell_size = WINDOW_WIDTH / FIELD_SIZE as f32;
    let x = -WINDOW_WIDTH / 2. + cell_size * (col as f32 + 0.5);
    let y = -WINDOW_HEIGHT / 2. + cell_size * (row as f32 + 0.5);
    (x, y)
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_field(mut commands: Commands, textures: Res<Textures>) {
    let cell_size = get_cell_size();
    for row in 0..FIELD_SIZE {
        for col in 0..FIELD_SIZE {
            let (x, y) = get_cell_coords(row, col);
            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(cell_size, cell_size)),
                    ..Default::default()
                },
                transform: Transform::from_translation(Vec3::new(x, y, 0.)),
                texture: textures.cell.clone(),
                ..Default::default()
            });
        }
    }
}

#[derive(Component)]
struct CircleComponent {
    row: usize,
    col: usize
}

#[derive(Component)]
struct FutureCircleComponent;

fn make_initial_turns(mut commands: Commands,
             query: Query<(Entity, &FutureCircleComponent)>,
             mut game_state: ResMut<GameState>,
             textures: Res<Textures>) {
    next_turn_impl(&mut commands, &query, &mut game_state, &textures, false);
    next_turn_impl(&mut commands, &query, &mut game_state, &textures, true);
}

fn next_turn(commands: &mut Commands,
             query: &Query<(Entity, &FutureCircleComponent)>,
             game_state: &mut ResMut<GameState>,
             textures: &Res<Textures>) {
    next_turn_impl(commands, query, game_state, textures, true);
}

fn next_turn_impl(commands: &mut Commands,
                  query: &Query<(Entity, &FutureCircleComponent)>,
                  game_state: &mut ResMut<GameState>,
                  textures: &Res<Textures>,
                  spawn_future: bool) {
    for (entity, _) in query.iter() {
        commands.entity(entity).despawn();
    }

    let cell_size = get_cell_size();
    let game_state = &mut **game_state;
    for circle in &game_state.future_circles {
        game_state.cells[circle.row][circle.col].0 = circle.kind;
        let (x, y) = get_cell_coords(circle.row, circle.col);
        commands.spawn((SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(cell_size * 0.8, cell_size * 0.8)),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(x, y, 2.)),
            texture: textures.circles[circle.kind as usize].clone(),
            ..Default::default()
        }, CircleComponent {
            row: circle.row,
            col: circle.col
        }));
    }

    game_state.random_future_circles(CIRCLES_PER_TURN).unwrap();

    if !spawn_future {
        return;
    }

    for circle in &game_state.future_circles {
        let (x, y) = get_cell_coords(circle.row, circle.col);
        commands.spawn((SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(cell_size / 4., cell_size / 4.)),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(x, y, 1.)),
            texture: textures.circles[circle.kind as usize].clone(),
            ..Default::default()
        }, FutureCircleComponent));
    }
}

#[derive(Component)]
struct SelectedCircleComponent {
    row: usize,
    col: usize,
    anim_time: f32
}

fn select_circle(mut commands: Commands,
                 query: Query<(Entity, &CircleComponent)>,
                 mut query_selected: Query<(Entity, &mut Sprite, &mut Transform, &CircleComponent), With<SelectedCircleComponent>>,
                 query_future: Query<(Entity, &FutureCircleComponent)>,
                 mut game_state: ResMut<GameState>,
                 textures: Res<Textures>,
                 window_query: Query<&Window>,
                 mouse_button: Res<Input<MouseButton>>) {
    if !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }

    let mut pos = match window_query.single().cursor_position() {
        Some(pos) => pos,
        None => return
    };
    pos.x -= WINDOW_WIDTH / 2.;
    pos.y -= WINDOW_HEIGHT / 2.;

    let cell_size = get_cell_size();
    for (entity, circle) in query.iter() {
        let (x, y) = get_cell_coords(circle.row, circle.col);
        if pos.x >= x - cell_size / 2. && pos.x <= x + cell_size / 2.
        && pos.y >= y - cell_size / 2. && pos.y <= y + cell_size / 2. {
            for (old_entity, mut old_sprite, mut old_transform, old_circle) in query_selected.iter_mut() {
                commands.entity(old_entity).remove::<SelectedCircleComponent>();
                old_sprite.custom_size = Some(Vec2::new(cell_size * 0.8, cell_size * 0.8));
                let (old_x, old_y) = get_cell_coords(old_circle.row, old_circle.col);
                old_transform.translation.x = old_x;
                old_transform.translation.y = old_y;
            }
            commands.entity(entity).insert(SelectedCircleComponent {
                row: circle.row,
                col: circle.col,
                anim_time: 0.
            });
            return;
        }
    }

    let (sel_entity, mut sel_sprite, mut sel_transform, sel_circle) = match query_selected.get_single_mut() {
        Ok(val) => val,
        Err(_) => return
    };

    let new_col = ((pos.x + WINDOW_WIDTH / 2.) / cell_size) as usize;
    let new_row = ((pos.y + WINDOW_HEIGHT / 2.) / cell_size) as usize;
    if new_col >= FIELD_SIZE || new_row >= FIELD_SIZE {
        return;
    }

    game_state.cells[new_row][new_col] = game_state.cells[sel_circle.row][sel_circle.col];
    game_state.cells[sel_circle.row][sel_circle.col] = CellState(-1);
    sel_sprite.custom_size = Some(Vec2::new(cell_size * 0.8, cell_size * 0.8));
    (sel_transform.translation.x, sel_transform.translation.y) = get_cell_coords(new_row, new_col);
    commands.entity(sel_entity).remove::<SelectedCircleComponent>();

    next_turn(&mut commands, &query_future, &mut game_state, &textures);
}

fn animate_selected_circle(mut query: Query<(&mut SelectedCircleComponent, &mut Sprite, &mut Transform)>,
                           time: Res<Time>) {
    let (mut circle, mut sprite, mut transform) = match query.get_single_mut() {
        Ok(val) => val,
        Err(_) => return
    };

    circle.anim_time += time.delta_seconds() * 5.;
    circle.anim_time -= (circle.anim_time / 4.).floor() * 4.;

    let dh: f32;
    let dy: f32;
    if circle.anim_time <= 1. {
        dh = -circle.anim_time * 0.3;
        dy = dh / 2.;
    }
    else if circle.anim_time <= 2. {
        dh = -(2. - circle.anim_time) * 0.3;
        dy = dh / 2.;
    }
    else if circle.anim_time <= 3. {
        dh = -(circle.anim_time - 2.) * 0.3;
        dy = -dh / 2.;
    }
    else {
        dh = -(4. - circle.anim_time) * 0.3;
        dy = -dh / 2.;
    }

    let cell_size = get_cell_size();
    sprite.custom_size = Some(Vec2::new(sprite.custom_size.unwrap().x, cell_size * 0.8 + dh * cell_size * 0.8));
    transform.translation.y = get_cell_coords(circle.row, circle.col).1 + dy * cell_size * 0.8;
}
