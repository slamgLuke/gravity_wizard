// main.rs
// Project: gravity_wizard
// Author: slamgLuke
// A Bevy game.

mod debug;
mod layout;
mod physics;
mod wizard;

use crate::debug::*;
use crate::layout::*;
use crate::physics::*;
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
        .add_startup_systems((window_setup, debug_grid, load_level, spawn_wizard))
        .add_systems(
            (
                wizard_input,
                gravitational_pull,
                platform_collision,
                apply_movement,
                debug_wizard,
            )
                .chain(),
        )
        .add_system(set_active_color.run_if(in_air))
        .add_system(set_passive_color.run_if(not(in_air)))
        .add_system(flip_gravity.run_if(not(in_air)))
        .add_systems(
            (despawn_wizard, spawn_wizard)
                .chain()
                .distributive_run_if(out_of_screen),
        )
        .add_system(clear_level.run_if(is_in_exit))
        .run();
}

pub fn window_setup(
    mut commands: Commands,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut window = window_query.single_mut();
    window.resizable = false;
    window.resolution = WindowResolution::new(1280.0, 720.0);
    window.mode = WindowMode::Windowed;
    window.title = "Gravity Wizard".to_string();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

pub fn load_level(commands: Commands) {
    level_data_reader("levels/1.txt".into(), commands);
}
