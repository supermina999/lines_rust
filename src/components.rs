use bevy::prelude::*;

#[derive(Component)]
pub struct CircleComponent {
    pub row: usize,
    pub col: usize
}

#[derive(Component)]
pub struct FutureCircleComponent;

#[derive(Component)]
pub struct NextTurnComponent;

#[derive(Component)]
pub struct SelectedCircleComponent {
    pub row: usize,
    pub col: usize,
    pub anim_time: f32
}

#[derive(Component)]
pub struct MoveAnimationComponent {
    pub path: Vec<(usize, usize)>,
    pub anim_time: f32
}

#[derive(Component)]
pub struct DisappearComponent;

#[derive(Component)]
pub struct DisappearAnimationComponent {
    pub anim_time: f32
}
