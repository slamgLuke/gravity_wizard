// wizard.rs

use crate::physics::*;
use bevy::{prelude::*, window::PrimaryWindow};

pub const WIZARD_SIZE: f32 = 20.0;
pub const WIZARD_COLOR_ACTIVE: Color = Color::rgb(0.0, 0.0, 1.0);
pub const WIZARD_COLOR_PASSIVE: Color = Color::rgb(0.0, 0.0, 0.0);
pub const WIZARD_SHAPE: Vec2 = Vec2::new(WIZARD_SIZE, WIZARD_SIZE);

#[derive(Component)]
pub struct Wizard;

pub fn wizard_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut wizard_velocity_query: Query<&mut Velocity, With<Wizard>>,
) {
    let mut velocity = wizard_velocity_query.single_mut();
    let mut direction = None;

    let left = keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A);
    let right = keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D);

    if left && !right {
        direction = Some(-HORIZONTAL_ACCELERATION);
    } else if right && !left {
        direction = Some(HORIZONTAL_ACCELERATION);
    }

    if let Some(value) = direction {
        if (velocity.x > 0.0 && value < 0.0) || (velocity.x < 0.0 && value > 0.0) {
            velocity.x += value * 2.0;
        } else {
            velocity.x += value;
        }
    } else if velocity.x > 0.0 {
        velocity.x -= HORIZONTAL_ACCELERATION;
    } else if velocity.x < 0.0 {
        velocity.x += HORIZONTAL_ACCELERATION;
    }

    if velocity.x > MAX_HORIZONTAL_VELOCITY {
        velocity.x = MAX_HORIZONTAL_VELOCITY;
    } else if velocity.x < -MAX_HORIZONTAL_VELOCITY {
        velocity.x = -MAX_HORIZONTAL_VELOCITY;
    }
}

// conditionals
pub fn in_air(wizard_velocity_query: Query<&Velocity, With<Wizard>>) -> bool {
    let wizard_velocity = wizard_velocity_query.single();
    wizard_velocity.y != 0.0
}
pub fn out_of_screen(
    wizard_query: Query<(&Transform, &EntitySize), With<Wizard>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) -> bool {
    let (wizard_transform, player_size) = wizard_query.single();
    let window = window_query.single();

    let highx = wizard_transform.translation.x + player_size.width / 2.0;
    let lowx = wizard_transform.translation.x - player_size.width / 2.0;
    let highy = wizard_transform.translation.y + player_size.height / 2.0;
    let lowy = wizard_transform.translation.y - player_size.height / 2.0;

    lowx > window.width() || highx < 0.0 || lowy > window.height() || highy < 0.0
}

pub fn spawn_wizard(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut gravity: ResMut<Gravity>,
) {
    let window = window_query.single();
    gravity.0 = GravityDirection::Down;
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: WIZARD_COLOR_PASSIVE,
                custom_size: Some(WIZARD_SHAPE),
                ..default()
            },
            transform: Transform::from_xyz(window.width() / 6.0, window.height() / 5.0, 0.0),
            ..default()
        },
        Wizard,
        Velocity { x: 0.0, y: 0.0 },
        EntitySize {
            width: WIZARD_SIZE,
            height: WIZARD_SIZE,
        },
    ));
}

pub fn despawn_wizard(mut commands: Commands, wizard_query: Query<Entity, With<Wizard>>) {
    let wizard = wizard_query.single();
    commands.entity(wizard).despawn();
}
