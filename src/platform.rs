// platform.rs

use bevy::prelude::*;

#[derive(Component)]
pub struct Platform {
    pub lowx: f32,
    pub highx: f32,
    pub lowy: f32,
    pub highy: f32,
}

pub fn spawn_platforms(mut commands: Commands, platforms: Vec<Platform>) {
    for platform in platforms {
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
}
