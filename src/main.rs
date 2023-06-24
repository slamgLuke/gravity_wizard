use bevy::{prelude::*, window::PrimaryWindow};

fn main() {
    println!("Running Bevy!");
    App::new()
        .insert_resource(Gravity(Direction::Down))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_systems((player_input, apply_gravity, apply_acceleration).chain())
        .run();
}

const GRAVITY: f32 = 9.8 * 1.5;

enum Direction {
    Down,
    Up,
}

#[derive(Resource)]
struct Gravity(Direction);

const WIZARD_SIZE: f32 = 20.0;
const WIZARD_COLOR: Color = Color::rgb(0.0, 1.0, 0.0);
const WIZARD_SHAPE: Vec2 = Vec2::new(WIZARD_SIZE, WIZARD_SIZE);

const ACCELERATION: f32 = 8.0;
const MAX_HORIZONTAL_VELOCITY: f32 = 250.0;
const MAX_VERTICAL_VELOCITY: f32 = 980.0;

#[derive(Component)]
struct Wizard;

// velocity: component to handle acceleration in movement physics
#[derive(Component)]
struct Velocity {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Platform {
    lowx: f32,
    highx: f32,
    lowy: f32,
    highy: f32,
}

fn setup(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.single();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: WIZARD_COLOR,
                custom_size: Some(WIZARD_SHAPE),
                ..default()
            },
            transform: Transform::from_xyz(window.width() / 4.0, window.height() / 3.0, 0.0),
            ..default()
        },
        Wizard,
        Velocity { x: 0.0, y: 0.0 },
    ));
    spawn_platform(
        commands,
        Platform {
            lowx: 50.0,
            highx: 150.0,
            lowy: 40.0,
            highy: 45.0,
        },
    );
}

fn spawn_platform(mut commands: Commands, platform: Platform) {
    let x = platform.lowx + platform.highx / 2.0;
    let y = platform.lowy + platform.highy / 2.0;
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(90.0, 3.0)),
                ..default()
            },
            transform: Transform::from_xyz(x, y, 0.0),
            ..default()
        },
        platform,
    ));
}

fn player_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_velocity_query: Query<&mut Velocity, With<Wizard>>,
    mut gravity: ResMut<Gravity>,
) {
    let mut velocity = player_velocity_query.single_mut();
    let mut direction = None;

    let left = keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A);
    let right = keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D);
    let space = keyboard_input.just_pressed(KeyCode::Space);

    if left && !right {
        direction = Some(-ACCELERATION);
    } else if right && !left {
        direction = Some(ACCELERATION);
    }

    if space {
        match gravity.0 {
            Direction::Down => gravity.0 = Direction::Up,
            Direction::Up => gravity.0 = Direction::Down,
        }
    }

    if let Some(value) = direction {
        if (velocity.x > 0.0 && value < 0.0) || (velocity.x < 0.0 && value > 0.0) {
            velocity.x += value * 2.0;
        } else {
            velocity.x += value;
        }
    } else if velocity.x > 0.0 {
        velocity.x -= ACCELERATION;
    } else if velocity.x < 0.0 {
        velocity.x += ACCELERATION;
    }

    if velocity.x > MAX_HORIZONTAL_VELOCITY {
        velocity.x = MAX_HORIZONTAL_VELOCITY;
    } else if velocity.x < -MAX_HORIZONTAL_VELOCITY {
        velocity.x = -MAX_HORIZONTAL_VELOCITY;
    }
}

fn apply_gravity(mut velocity_query: Query<&mut Velocity>, gravity: Res<Gravity>) {
    match gravity.0 {
        Direction::Down => {
            for mut velocity in velocity_query.iter_mut() {
                velocity.y -= GRAVITY;
                if velocity.y < -MAX_VERTICAL_VELOCITY {
                    velocity.y = -MAX_VERTICAL_VELOCITY;
                }
            }
        }
        Direction::Up => {
            for mut velocity in velocity_query.iter_mut() {
                velocity.y += GRAVITY;
                if velocity.y > MAX_VERTICAL_VELOCITY {
                    velocity.y = MAX_VERTICAL_VELOCITY;
                }
            }
        }
    }
}

fn apply_acceleration(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}
