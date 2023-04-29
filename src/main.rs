use bevy::prelude::*;

mod constants;
use constants::*;
mod textures;
use textures::*;
mod game_state;
use game_state::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin{
            window: WindowDescriptor {
                width: WINDOW_WIDTH,
                height: WINDOW_HEIGHT,
                title: "Lines".to_string(),
                resizable: false,
                ..Default::default()
            },
            ..Default::default()
        }))
        .init_resource::<Textures>()
        .init_resource::<GameState>()
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_field)
        .add_startup_system(next_turn)
        .add_startup_system(next_turn)
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
struct CircleComponent;

#[derive(Component)]
struct FutureCircleComponent;

fn next_turn(mut commands: Commands,
             query: Query<(Entity, &FutureCircleComponent)>,
             mut game_state: ResMut<GameState>,
             textures: Res<Textures>) {
    for (entity, _) in query.iter() {
        commands.entity(entity).despawn();
    }

    let cell_size = get_cell_size();
    let game_state = &mut *game_state;
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
        }, CircleComponent));
    }

    game_state.random_future_circles(CIRCLES_PER_TURN).unwrap();

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
