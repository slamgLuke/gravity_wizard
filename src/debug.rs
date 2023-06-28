use crate::physics::*;
use crate::wizard::*;
use bevy::input::keyboard;
use bevy::{
    prelude::*,
    window::{PrimaryWindow, Window},
};

pub fn debug_grid(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.single();

    let grid_size = 10.0;
    let grid_width = window.width() / grid_size;
    let grid_height = window.height() / grid_size;
    println!("Grid size: {}x{}", grid_width, grid_height);

    for i in 0..=(grid_width as u32) {
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(1.0, window.height())),
                ..default()
            },
            transform: Transform::from_xyz(i as f32 * grid_size, window.height() / 2.0, 0.0),
            ..Default::default()
        });
    }
    for i in 0..=(grid_height as u32) {
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(window.width(), 1.0)),
                ..default()
            },
            transform: Transform::from_xyz(window.width() / 2.0, i as f32 * grid_size, 0.0),
            ..Default::default()
        });
    }
}

pub fn debug_wizard(
    wizard_query: Query<(&Transform, &Velocity), With<Wizard>>,
    keyboard_input: Res<Input<keyboard::KeyCode>>,
) {
    let r = keyboard_input.just_pressed(keyboard::KeyCode::R);
    if r {
        for (transform, velocity) in wizard_query.iter() {
            println!(
                "Wizard position: ({}, {}), velocity: ({}, {})",
                transform.translation.x, transform.translation.y, velocity.x, velocity.y
            );
        }
    }
}
