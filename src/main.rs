use bevy::{prelude::*, window::PrimaryWindow};

fn main() {
    println!("Running Bevy!");
    App::new()
        .insert_resource(Gravity(Direction::Down))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_systems((movement_input, apply_gravity, collide, apply_acceleration).chain())
        .add_system(passive_color.run_if(in_air))
        .add_system(active_color.run_if(not(in_air)))
        .add_system(gravity_flip_input.run_if(not(in_air)))
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
const WIZARD_COLOR_ACTIVE: Color = Color::rgb(0.0, 1.0, 0.0);
const WIZARD_COLOR_PASSIVE: Color = Color::rgb(0.0, 0.0, 1.0);
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

#[derive(Component)]
struct Size {
    width: f32,
    height: f32,
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
                color: WIZARD_COLOR_PASSIVE,
                custom_size: Some(WIZARD_SHAPE),
                ..default()
            },
            transform: Transform::from_xyz(window.width() / 5.0, window.height() / 3.0, 0.0),
            ..default()
        },
        Wizard,
        Velocity { x: 0.0, y: 0.0 },
        Size {
            width: WIZARD_SIZE,
            height: WIZARD_SIZE,
        },
    ));

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

fn spawn_platforms(mut commands: Commands, platforms: Vec<Platform>) {
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

fn movement_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_velocity_query: Query<&mut Velocity, With<Wizard>>,
) {
    let mut velocity = player_velocity_query.single_mut();
    let mut direction = None;

    let left = keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A);
    let right = keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D);

    if left && !right {
        direction = Some(-ACCELERATION);
    } else if right && !left {
        direction = Some(ACCELERATION);
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

fn in_air(player_velocity_query: Query<&Velocity, With<Wizard>>) -> bool {
    let velocity = player_velocity_query.single();
    velocity.y != 0.0
}

fn active_color(mut player_color_query: Query<&mut Sprite, With<Wizard>>) {
    let mut sprite = player_color_query.single_mut();
    sprite.color = WIZARD_COLOR_ACTIVE;
}
fn passive_color(mut player_color_query: Query<&mut Sprite, With<Wizard>>) {
    let mut sprite = player_color_query.single_mut();
    sprite.color = WIZARD_COLOR_PASSIVE;
}

fn gravity_flip_input(keyboard_input: Res<Input<KeyCode>>, mut gravity: ResMut<Gravity>) {
    let space = keyboard_input.just_pressed(KeyCode::Space);

    if space {
        match gravity.0 {
            Direction::Down => gravity.0 = Direction::Up,
            Direction::Up => gravity.0 = Direction::Down,
        }
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

// handle collision between objects with velocity and platforms
fn collide(
    mut velocity_query: Query<(&mut Velocity, &Transform, &Size)>,
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

fn apply_acceleration(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}

fn out_of_screen(
    player_query: Query<(&Transform, &Size), With<Wizard>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) -> bool {
    let (player_transform, player_size) = player_query.single();
    let window = window_query.single();

    let highx = player_transform.translation.x + player_size.width / 2.0;
    let lowx = player_transform.translation.x - player_size.width / 2.0;
    let highy = player_transform.translation.y + player_size.height / 2.0;
    let lowy = player_transform.translation.y - player_size.height / 2.0;

    highx > window.width() || lowx < 0.0 || highy > window.height() || lowy < 0.0
}
