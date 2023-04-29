use bevy::prelude::*;
use crate::common::*;
use crate::components::*;

pub fn animate_selected_circle(mut query: Query<(&mut SelectedCircleComponent, &mut Sprite, &mut Transform)>,
                               time: Res<Time>) {
    let (mut circle, mut sprite, mut transform) = match query.get_single_mut() {
        Ok(val) => val,
        Err(_) => return
    };

    circle.anim_time += time.delta_seconds() / CIRCLE_BOUNCE_TIME;
    circle.anim_time -= (circle.anim_time / 4.).floor() * 4.;

    let dh: f32;
    let dy: f32;
    if circle.anim_time <= 1. {
        dh = -circle.anim_time * CIRCLE_BOUNCE_SCALE;
        dy = dh / 2.;
    }
    else if circle.anim_time <= 2. {
        dh = -(2. - circle.anim_time) * CIRCLE_BOUNCE_SCALE;
        dy = dh / 2.;
    }
    else if circle.anim_time <= 3. {
        dh = -(circle.anim_time - 2.) * CIRCLE_BOUNCE_SCALE;
        dy = -dh / 2.;
    }
    else {
        dh = -(4. - circle.anim_time) * CIRCLE_BOUNCE_SCALE;
        dy = -dh / 2.;
    }

    sprite.custom_size = Some(Vec2::new(sprite.custom_size.unwrap().x, CIRCLE_SIZE + dh * CIRCLE_SIZE));
    transform.translation.y = get_cell_coords(circle.row, circle.col).1 + dy * CIRCLE_SIZE;
}

pub fn move_animation(mut commands: Commands,
                      mut query: Query<(Entity, &mut MoveAnimationComponent, &mut Transform)>,
                      time: Res<Time>) {
    let (entity, mut anim, mut transform) = match query.get_single_mut() {
        Ok(val) => val,
        Err(_) => return
    };

    let final_pos = anim.path[0];

    anim.anim_time += time.delta_seconds();
    while anim.anim_time > CELL_MOVE_TIME && !anim.path.is_empty() {
        anim.anim_time -= CELL_MOVE_TIME;
        anim.path.pop();
    }

    let len = anim.path.len();
    if len <= 1 {
        (transform.translation.x, transform.translation.y)
            = get_cell_coords(final_pos.0, final_pos.1);
        commands.entity(entity).remove::<MoveAnimationComponent>();
        commands.spawn(NextTurnComponent);
        return;
    }

    let prev_pos = get_cell_coords(anim.path[len - 1].0, anim.path[len - 1].1);
    let next_pos = get_cell_coords(anim.path[len - 2].0, anim.path[len - 2].1);
    let next_coef = anim.anim_time / CELL_MOVE_TIME;
    let prev_coef = 1. - next_coef;
    let cur_pos = (prev_pos.0 * prev_coef + next_pos.0 * next_coef,
                   prev_pos.1 * prev_coef + next_pos.1 * next_coef);
    (transform.translation.x, transform.translation.y) = cur_pos;
}

pub fn disappear_animation(mut commands: Commands,
                           mut query: Query<(Entity, &mut DisappearAnimationComponent, &mut Sprite)>,
                           time: Res<Time>) {
    for (entity, mut anim, mut sprite) in query.iter_mut() {
        anim.anim_time += time.delta_seconds();
        if anim.anim_time >= CIRCLE_DISAPPEAR_TIME {
            commands.entity(entity).despawn();
            continue;
        }
        let circle_size = CIRCLE_SIZE * (CIRCLE_DISAPPEAR_TIME - anim.anim_time) / CIRCLE_DISAPPEAR_TIME;
        sprite.custom_size = Some(Vec2::new(circle_size, circle_size));
    }
}
