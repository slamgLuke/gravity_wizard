// main.rs
// Project: gravity_wizard
// Author: slamgLuke
// A Bevy game.

mod physics;
mod platform;
mod wizard;

use crate::physics::*;
use crate::platform::*;
use crate::wizard::*;
use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowMode, WindowResolution},
};

fn main() {
    println!("Running Bevy!");
    App::new()
        .insert_resource(Gravity(GravityDirection::Down))
        .add_plugins(DefaultPlugins)
        .add_startup_systems((setup, spawn_wizard))
        .add_systems(
            (
                wizard_input,
                gravitational_pull,
                platform_collision,
                apply_movement,
            )
                .chain(),
        )
        .add_system(flip_gravity.run_if(not(in_air)))
        .add_systems(
            (despawn_wizard, spawn_wizard)
                .chain()
                .distributive_run_if(out_of_screen),
        )
        .run();
}

fn setup(mut commands: Commands, mut window_query: Query<&mut Window, With<PrimaryWindow>>) {
    let mut window = window_query.single_mut();
    window.resizable = false;
    window.resolution = WindowResolution::new(1280.0, 720.0);
    window.mode = WindowMode::Windowed;
    window.title = "Gravity Wizard".to_string();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });

    // draw a grid on the background
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

    let platforms = vec![
        Platform {
            lowx: 0.0,
            highx: 300.0,
            lowy: 0.0,
            highy: 100.0,
        },
        Platform {
            lowx: 400.0,
            highx: 600.0,
            lowy: 500.0,
            highy: 550.0,
        },
        Platform {
            lowx: 690.0,
            highx: 1100.0,
            lowy: 590.0,
            highy: 599.0,
        },
        Platform {
            lowx: 1000.0,
            highx: window.width(),
            lowy: window.height() - 100.0,
            highy: window.height() - 50.0,
        },
    ];
    spawn_platforms(commands, platforms);
}
