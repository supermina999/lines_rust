use std::process::exit;
use bevy::prelude::*;
use bevy::window::WindowResolution;

mod common;
use common::*;
mod textures;
use textures::*;
mod components;
use components::*;
mod game_state;
use game_state::*;
mod animations;
use animations::*;

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
        .add_startup_system(spawn_score)
        .add_startup_system(make_initial_turns)
        .add_system(next_turn)
        .add_system(select_circle)
        .add_system(animate_selected_circle.before(select_circle))
        .add_system(move_animation)
        .add_system(disappear_circles)
        .add_system(disappear_animation)
        .add_system(score_update)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_field(mut commands: Commands, textures: Res<Textures>) {
    for row in 0..FIELD_SIZE {
        for col in 0..FIELD_SIZE {
            let (x, y) = get_cell_coords(row, col);
            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(CELL_SIZE, CELL_SIZE)),
                    ..Default::default()
                },
                transform: Transform::from_translation(Vec3::new(x, y, 0.)),
                texture: textures.cell.clone(),
                ..Default::default()
            });
        }
    }
}

fn spawn_score(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.), Val::Px(WINDOW_HEIGHT - CELL_SIZE * FIELD_SIZE as f32)),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        ..Default::default()
    }).with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            "",
            TextStyle {
                font: asset_server.load("FiraSans-Bold.ttf"),
                font_size: 100.0,
                color: Color::rgb(154. / 255., 193. / 255., 250. / 255.)
            }));
    });
}

fn make_initial_turns(mut commands: Commands,
             query: Query<(Entity, &FutureCircleComponent)>,
             mut game_state: ResMut<GameState>,
             textures: Res<Textures>) {
    next_turn_impl(&mut commands, &query, &mut game_state, &textures, false);
    next_turn_impl(&mut commands, &query, &mut game_state, &textures, true);
}

fn next_turn(mut commands: Commands,
             next_turn_query: Query<(Entity, &NextTurnComponent)>,
             query: Query<(Entity, &FutureCircleComponent)>,
             mut game_state: ResMut<GameState>,
             textures: Res<Textures>) {
    let next_turn_entity = match next_turn_query.get_single() {
        Ok((entity, _)) => entity,
        Err(_) => return
    };
    next_turn_impl(&mut commands, &query, &mut game_state, &textures, true);
    commands.entity(next_turn_entity).despawn();
}

fn next_turn_impl(commands: &mut Commands,
                  query: &Query<(Entity, &FutureCircleComponent)>,
                  game_state: &mut ResMut<GameState>,
                  textures: &Res<Textures>,
                  spawn_future: bool) {
    for (entity, _) in query.iter() {
        commands.entity(entity).despawn();
    }

    let game_state = &mut **game_state;
    for circle in &game_state.future_circles {
        let cell = &mut game_state.cells[circle.row][circle.col];
        if cell.0 != -1 {
            continue;
        }
        cell.0 = circle.kind;
        let (x, y) = get_cell_coords(circle.row, circle.col);
        commands.spawn((SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(CIRCLE_SIZE, CIRCLE_SIZE)),
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

    if game_state.random_future_circles(CIRCLES_PER_TURN).is_err() {
        println!("Game Over");
        exit(0);
    }

    if !spawn_future {
        return;
    }

    for circle in &game_state.future_circles {
        let (x, y) = get_cell_coords(circle.row, circle.col);
        commands.spawn((SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(CELL_SIZE / 4., CELL_SIZE / 4.)),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(x, y, 1.)),
            texture: textures.circles[circle.kind as usize].clone(),
            ..Default::default()
        }, FutureCircleComponent));
    }

    commands.spawn(DisappearComponent);
}

fn select_circle(mut commands: Commands,
                 query: Query<(Entity, &CircleComponent), Without<SelectedCircleComponent>>,
                 mut query_selected: Query<(Entity, &mut Sprite, &mut Transform, &mut CircleComponent), With<SelectedCircleComponent>>,
                 mut game_state: ResMut<GameState>,
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

    for (entity, circle) in query.iter() {
        let (x, y) = get_cell_coords(circle.row, circle.col);
        if pos.x >= x - CELL_SIZE / 2. && pos.x <= x + CELL_SIZE / 2.
        && pos.y >= y - CELL_SIZE / 2. && pos.y <= y + CELL_SIZE / 2. {
            for (old_entity, mut old_sprite, mut old_transform, old_circle) in query_selected.iter_mut() {
                commands.entity(old_entity).remove::<SelectedCircleComponent>();
                old_sprite.custom_size = Some(Vec2::new(CIRCLE_SIZE, CIRCLE_SIZE));
                let (old_x, old_y) = get_cell_coords(old_circle.row, old_circle.col);
                (old_transform.translation.x, old_transform.translation.y) = (old_x, old_y);
            }
            commands.entity(entity).insert(SelectedCircleComponent {
                row: circle.row,
                col: circle.col,
                anim_time: 0.
            });
            return;
        }
    }

    let (sel_entity, mut sel_sprite, mut sel_transform, mut sel_circle) =
        match query_selected.get_single_mut() {
        Ok(val) => val,
        Err(_) => return
    };

    let new_col = ((pos.x + WINDOW_WIDTH / 2.) / CELL_SIZE) as usize;
    let new_row = ((pos.y + WINDOW_HEIGHT / 2.) / CELL_SIZE) as usize;
    if new_col >= FIELD_SIZE || new_row >= FIELD_SIZE {
        return;
    }

    let path = game_state.find_path((sel_circle.row, sel_circle.col), (new_row, new_col));
    if path.is_none() {
        return;
    }

    game_state.cells[new_row][new_col] = game_state.cells[sel_circle.row][sel_circle.col];
    game_state.cells[sel_circle.row][sel_circle.col] = CellState(-1);
    sel_sprite.custom_size = Some(Vec2::new(CIRCLE_SIZE, CIRCLE_SIZE));
    (sel_transform.translation.x, sel_transform.translation.y) = get_cell_coords(sel_circle.row, sel_circle.col);
    (sel_circle.row, sel_circle.col) = (new_row, new_col);
    commands.entity(sel_entity).remove::<SelectedCircleComponent>();
    commands.entity(sel_entity).insert(MoveAnimationComponent {
        path: path.unwrap(),
        anim_time: 0.
    });
}

fn disappear_circles(mut commands: Commands,
                     query: Query<(Entity, &CircleComponent)>,
                     disappear_query: Query<(Entity, &DisappearComponent)>,
                     mut game_state: ResMut<GameState>) {
    let entity = match disappear_query.get_single() {
        Ok((entity, _)) => entity,
        Err(_) => return
    };
    commands.entity(entity).despawn();

    let disappear_set = game_state.process_disappearing_circles();
    for (entity, circle) in query.iter() {
        if disappear_set.contains(&(circle.row, circle.col)) {
            commands.entity(entity).insert(DisappearAnimationComponent {
                anim_time: 0.
            });
        }
    }
}

fn score_update(mut query: Query<&mut Text>, game_state: Res<GameState>) {
    let mut text = query.single_mut();
    text.sections[0].value = format!("Score: {}", game_state.score);
}
