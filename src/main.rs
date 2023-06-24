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
use bevy::{prelude::*, window::PrimaryWindow};

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

fn setup(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.single();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });

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
            lowy: 598.0,
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
