//
// physics.rs
//
use crate::layout::*;
use bevy::prelude::*;

pub enum GravityDirection {
    Down,
    Up,
}

pub const GRAVITY: f32 = 9.8 * 1.5;

#[derive(Resource)]
pub struct Gravity(pub GravityDirection);

pub const HORIZONTAL_ACCELERATION: f32 = 4.0;
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

            // collision from above
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

            // collision from the sides
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

            // rare case: collision from the corners
            if (above && left)
                && lowy + velocity.y * time.delta_seconds() < platform.highy
                && highx + velocity.x * time.delta_seconds() > platform.lowx
            {
                velocity.y = 0.0;
                velocity.x = 0.0;
            } else if (above && right)
                && lowy + velocity.y * time.delta_seconds() < platform.highy
                && lowx + velocity.x * time.delta_seconds() < platform.highx
            {
                velocity.y = 0.0;
                velocity.x = 0.0;
            } else if (below && left)
                && highy + velocity.y * time.delta_seconds() > platform.lowy
                && highx + velocity.x * time.delta_seconds() > platform.lowx
            {
                velocity.y = 0.0;
                velocity.x = 0.0;
            } else if (below && right)
                && highy + velocity.y * time.delta_seconds() > platform.lowy
                && lowx + velocity.x * time.delta_seconds() < platform.highx
            {
                velocity.y = 0.0;
                velocity.x = 0.0;
            }
        }
    }
}

pub fn object_collision(
    mut velocity_query: Query<(&mut Velocity, &Transform, &EntitySize)>,
    time: Res<Time>,
) {
    for (mut velocity, transform, size) in velocity_query.iter() {
        for (object_velocity, object_transform, object_entitysize) in velocity_query.iter() {
            // skip if the object is the same
            if transform.translation == object_transform.translation {
                continue;
            }
            let highx = transform.translation.x + size.width / 2.0;
            let lowx = transform.translation.x - size.width / 2.0;
            let highy = transform.translation.y + size.height / 2.0;
            let lowy = transform.translation.y - size.height / 2.0;

            let object_highx = object_transform.translation.x + object_entitysize.width / 2.0;
            let object_lowx = object_transform.translation.x - object_entitysize.width / 2.0;
            let object_highy = object_transform.translation.y + object_entitysize.height / 2.0;
            let object_lowy = object_transform.translation.y - object_entitysize.height / 2.0;

            let above = lowy >= object_highy;
            let below = highy <= object_lowy;
            let left = highx <= object_lowx;
            let right = lowx >= object_highx;

            // difference: 2 objects can be moving at the same time and collide
            // collision from above
            if above
                && !right
                && !left
                && lowy + velocity.y * time.delta_seconds()
                    < object_highy + object_velocity.y * time.delta_seconds()
            {
                velocity.y = 0.0;
            } else if below
                && !right
                && !left
                && highy + velocity.y * time.delta_seconds()
                    > object_lowy + object_velocity.y * time.delta_seconds()
            {
                velocity.y = 0.0;
            }

            // collision from the sides
            if left
                && !above
                && !below
                && highx + velocity.x * time.delta_seconds()
                    > object_lowx + object_velocity.x * time.delta_seconds()
            {
                velocity.x = 0.0;
            } else if right
                && !above
                && !below
                && lowx + velocity.x * time.delta_seconds()
                    < object_highx + object_velocity.x * time.delta_seconds()
            {
                velocity.x = 0.0;
            }

            // rare case: collision from the corners
            if (above && left)
                && lowy + velocity.y * time.delta_seconds()
                    < object_highy + object_velocity.y * time.delta_seconds()
                && highx + velocity.x * time.delta_seconds()
                    > object_lowx + object_velocity.x * time.delta_seconds()
            {
                velocity.y = 0.0;
                velocity.x = 0.0;
            } else if (above && right)
                && lowy + velocity.y * time.delta_seconds()
                    < object_highy + object_velocity.y * time.delta_seconds()
                && lowx + velocity.x * time.delta_seconds()
                    < object_highx + object_velocity.x * time.delta_seconds()
            {
                velocity.y = 0.0;
                velocity.x = 0.0;
            } else if (below && left)
                && highy + velocity.y * time.delta_seconds()
                    > object_lowy + object_velocity.y * time.delta_seconds()
                && highx + velocity.x * time.delta_seconds()
                    > object_lowx + object_velocity.x * time.delta_seconds()
            {
                velocity.y = 0.0;
                velocity.x = 0.0;
            } else if (below && right)
                && highy + velocity.y * time.delta_seconds()
                    > object_lowy + object_velocity.y * time.delta_seconds()
                && lowx + velocity.x * time.delta_seconds()
                    < object_highx + object_velocity.x * time.delta_seconds()
            {
                velocity.y = 0.0;
                velocity.x = 0.0;
            }
        }
    }
}

// todo: handle collision between objects with velocity and other objects with velocity
