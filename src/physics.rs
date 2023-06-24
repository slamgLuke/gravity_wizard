// physics.rs

use crate::platform::Platform;
use bevy::prelude::*;

pub enum GravityDirection {
    Down,
    Up,
}

pub const GRAVITY: f32 = 9.8 * 1.5;

#[derive(Resource)]
pub struct Gravity(pub GravityDirection);

pub const HORIZONTAL_ACCELERATION: f32 = 8.0;
pub const MAX_HORIZONTAL_VELOCITY: f32 = 250.0;
pub const MAX_VERTICAL_VELOCITY: f32 = 980.0;

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct EntitySize {
    pub width: f32,
    pub height: f32,
}

pub fn flip_gravity(keyboard_input: Res<Input<KeyCode>>, mut gravity: ResMut<Gravity>) {
    let space = keyboard_input.just_pressed(KeyCode::Space);
    if space {
        match gravity.0 {
            GravityDirection::Down => gravity.0 = GravityDirection::Up,
            GravityDirection::Up => gravity.0 = GravityDirection::Down,
        }
    }
}

pub fn apply_movement(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}
// apply gravity to objects with velocity
pub fn gravitational_pull(mut velocity_query: Query<&mut Velocity>, gravity: Res<Gravity>) {
    match gravity.0 {
        GravityDirection::Down => {
            for mut velocity in velocity_query.iter_mut() {
                velocity.y -= GRAVITY;
                if velocity.y < -MAX_VERTICAL_VELOCITY {
                    velocity.y = -MAX_VERTICAL_VELOCITY;
                }
            }
        }
        GravityDirection::Up => {
            for mut velocity in velocity_query.iter_mut() {
                velocity.y += GRAVITY;
                if velocity.y > MAX_VERTICAL_VELOCITY {
                    velocity.y = MAX_VERTICAL_VELOCITY;
                }
            }
        }
    }
}
// handle collision between objects with velocity and platforms
pub fn platform_collision(
    mut velocity_query: Query<(&mut Velocity, &Transform, &EntitySize)>,
    platform_query: Query<&Platform>,
    time: Res<Time>,
) {
    for (mut velocity, transform, size) in velocity_query.iter_mut() {
        for platform in platform_query.iter() {
            let highx = transform.translation.x + size.width / 2.0;
            let lowx = transform.translation.x - size.width / 2.0;
            let highy = transform.translation.y + size.height / 2.0;
            let lowy = transform.translation.y - size.height / 2.0;

            let above = lowy >= platform.highy;
            let below = highy <= platform.lowy;
            let left = highx <= platform.lowx;
            let right = lowx >= platform.highx;

            if above && !right && !left && lowy + velocity.y * time.delta_seconds() < platform.highy
            {
                velocity.y = 0.0;
            } else if below
                && !right
                && !left
                && highy + velocity.y * time.delta_seconds() > platform.lowy
            {
                velocity.y = 0.0;
            }

            if left && !above && !below && highx + velocity.x * time.delta_seconds() > platform.lowx
            {
                velocity.x = 0.0;
            } else if right
                && !above
                && !below
                && lowx + velocity.x * time.delta_seconds() < platform.highx
            {
                velocity.x = 0.0;
            }
        }
    }
}

// todo: handle collision between objects with velocity and other objects with velocity
