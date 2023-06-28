#[allow(unused_imports)]
//
// platform.rs
//
use crate::physics::*;
use crate::wizard::Wizard;
use bevy::prelude::*;
use std::fs;

#[derive(Component)]
pub struct Platform {
    pub lowx: f32,
    pub highx: f32,
    pub lowy: f32,
    pub highy: f32,
}

// exit: a square that the player must squeeze into to win
#[derive(Component)]
pub struct Exit {
    pub x: f32,
    pub y: f32,
}

// the amount of pixels between the player and the exit that is still considered a win
const EXIT_MARGIN: f32 = 5.0;
const EXIT_SIZE: f32 = 18.0;

pub fn is_in_exit(
    exit_query: Query<&Exit>,
    wizard_transform_query: Query<&Transform, With<Wizard>>,
) -> bool {
    let translation = wizard_transform_query.single().translation;
    let exit = exit_query.single();
    (exit.x - translation.x).abs() < EXIT_MARGIN && (exit.y - translation.y).abs() < EXIT_MARGIN
}

#[derive(Component)]
pub struct Object;

#[derive(Component)]
pub struct DeathZone;

// level creation/deletion
pub fn level_data_reader(path: String, mut commands: Commands) {
    let data = fs::read_to_string(path).expect("Failed to read level data");
    let level_data: Vec<&str> = data.split(";").collect();
    println!("{:#?}", level_data);

    // let header = level_data[0].split("\n").collect::<Vec<&str>>();
    // let title = header[0];
    // let n_platforms = header[1].split(" ").collect::<Vec<&str>>()[1]
    //     .parse::<usize>()
    //     .unwrap();
    // let n_objects = header[2].split(" ").collect::<Vec<&str>>()[1]
    //     .parse::<usize>()
    //     .unwrap();

    let platforms = level_data[1].split(",").collect::<Vec<&str>>();
    let mut platforms_vec = Vec::new();
    for line in platforms.iter() {
        let line = line.trim().split(" ").collect::<Vec<&str>>();
        if line == vec!["NULL"] {
            break;
        }
        platforms_vec.push(Platform {
            lowx: line[0].parse::<f32>().unwrap(),
            highx: line[1].parse::<f32>().unwrap(),
            lowy: line[2].parse::<f32>().unwrap(),
            highy: line[3].parse::<f32>().unwrap(),
        });
    }

    let objects = level_data[2].split(",").collect::<Vec<&str>>();
    let mut objects_vec = Vec::new();
    for line in objects.iter() {
        let line = line.trim().split(" ").collect::<Vec<&str>>();
        if line == vec!["NULL"] {
            break;
        }
        objects_vec.push(Platform {
            lowx: line[0].parse::<f32>().unwrap(),
            highx: line[1].parse::<f32>().unwrap(),
            lowy: line[2].parse::<f32>().unwrap(),
            highy: line[3].parse::<f32>().unwrap(),
        });
    }
    let exit_vec = level_data[3].trim().split(" ").collect::<Vec<&str>>();

    // spawning
    for platform in platforms_vec {
        let x = (platform.lowx + platform.highx) / 2.0;
        let y = (platform.lowy + platform.highy) / 2.0;
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.0, 0.0, 0.0),
                    custom_size: Some(Vec2::new(
                        platform.highx - platform.lowx,
                        platform.highy - platform.lowy,
                    )),
                    ..default()
                },
                transform: Transform::from_xyz(x, y, 0.0),
                ..default()
            },
            platform,
        ));
    }
    for object in objects_vec {
        let x = (object.lowx + object.highx) / 2.0;
        let y = (object.lowy + object.highy) / 2.0;
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.0, 0.0, 0.0),
                    custom_size: Some(Vec2::new(
                        object.highx - object.lowx,
                        object.highy - object.lowy,
                    )),
                    ..default()
                },
                transform: Transform::from_xyz(x, y, 0.0),
                ..default()
            },
            Velocity { x: 0.0, y: 0.0 },
            EntitySize {
                width: object.highx - object.lowx,
                height: object.highy - object.lowy,
            },
            object,
        ));
    }
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.0, 1.0, 0.0),
                custom_size: Some(Vec2::new(EXIT_SIZE, EXIT_SIZE)),
                ..default()
            },
            transform: Transform::from_xyz(
                exit_vec[0].parse::<f32>().unwrap(),
                exit_vec[1].parse::<f32>().unwrap(),
                0.0,
            ),
            ..default()
        },
        Exit {
            x: exit_vec[0].parse::<f32>().unwrap(),
            y: exit_vec[1].parse::<f32>().unwrap(),
        },
    ));
}

pub fn clear_level(
    mut commands: Commands,
    platform_query: Query<Entity, With<Platform>>,
    exit_query: Query<Entity, With<Exit>>,
) {
    for platform in platform_query.iter() {
        commands.entity(platform).despawn();
    }
    let exit = exit_query.single();
    commands.entity(exit).despawn();
}
