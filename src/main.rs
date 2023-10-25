use bevy::math::{vec2, vec3};
use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};

//padle const
const PADDLE_START_Y                  : f32   = WALL_BOTTOM + 60.0;
const PADDLE_SIZE                     : Vec2  = Vec2::new(120.0, 20.0);
const PADDLE_COLOR                    : Color = Color::rgb(0.3, 0.3, 0.7);
const PADDLE_SPEED                    : f32   = 500.0;

  //ball const
const BALL_COLOR                      : Color = Color::rgb(1.0, 0.5, 0.5);
const BALL_START_POS                  : Vec3  = Vec3::new(0.0, -50.0, 1.0);
const BALL_SIZE                       : Vec2  = Vec2::new(30.0, 30.0);
const BALL_SPEED                      : f32   = 400.0;
const BALL_INITIAL_DIRECTION          : Vec2  = Vec2::new(0.5, -0.5);

  //wall
const WALL_LEFT                       : f32 = -450.0;
const WALL_RIGHT                      : f32 = 450.0;
const WALL_UP                         : f32 = 300.0;
const WALL_BOTTOM                     : f32 = -300.0;

const WALL_THICKNESS                  : f32 = 10.0;
const WALL_BLOCK_WIDTH                : f32 = WALL_RIGHT - WALL_LEFT;
const WALL_BLOCK_HEIGHT               : f32 = WALL_UP - WALL_BOTTOM;

  //Bricks
const BRICK_SIZE                      : Vec2  = Vec2::new(100., 30.);
const BRICK_COLOR                     : Color = Color::rgb(0.5, 0.5, 1.0);
const GAP_BETWEEN_BRICKS              : f32   = 5.0;
const GAP_BETWEEN_PADDLE_AND_BRICKS   : f32   = 270.0;
const GAP_BETWEEN_BRICKS_AND_CEILING  : f32   = 20.0;
const GAP_BETWEEN_BRICKS_AND_SIDES    : f32   = 20.0;

  //scoreboard
const SCOREBOARD_FONT_SIZE            : f32   = 40.0;
const SCOREBOARD_TEXT_PADDING         : Val   = Val::Px(5.0);
const TEXT_COLOR                      : Color = Color::rgb(0.5, 0.5, 1.0);
const SCORE_COLOR                     : Color = Color::rgb(1.0, 0.5, 0.5);

fn main() {
    App::new()
        .insert_resource(Scoreboard { score: 0 })
        .add_systems(Update, update_scoreboard)
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (
                paddle_mov,
                apply_velocity,
                check_ball_collisions.after(apply_velocity),
            ),
        )
        .run();
}

#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Ball {
    size: Vec2,
}

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct Collider {
    size: Vec2,
}

#[derive(Bundle)]
struct WallBundle {
    sprite_bundle: SpriteBundle,
    collider     : Collider,
}

#[derive(Component)]
struct Brick;

#[derive(Resource, Clone, Copy)]
struct Scoreboard {
    score: usize,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let paddle_transform = Transform {
        translation: vec3(0., PADDLE_START_Y, 0.),
        ..default()
    };

    let paddle_sprite = Sprite {
        color      : PADDLE_COLOR,
        custom_size: Some(PADDLE_SIZE),
        ..default()
    };

    commands.spawn((
        SpriteBundle {
            transform: paddle_transform,
            sprite   : paddle_sprite,
            ..default()
        },
        Paddle,
        Collider { size: PADDLE_SIZE },
    ));

    let ball_tex = asset_server.load("sprites/test.png");

    let ball_transform = Transform {
        translation: BALL_START_POS,
        ..default()
    };

    let ball_sprite = Sprite {
        color      : BALL_COLOR,
        custom_size: Some(BALL_SIZE),
        ..default()
    };

    commands.spawn((
        SpriteBundle {
            transform: ball_transform,
            sprite   : ball_sprite,
            texture  : ball_tex,
            ..default()
        },
        Ball { size: BALL_SIZE },
        Velocity(BALL_INITIAL_DIRECTION * BALL_SPEED),
    ));

    let vertical_wall_size   = vec2(WALL_THICKNESS, WALL_BLOCK_HEIGHT + WALL_THICKNESS);
    let horizontal_wall_size = vec2(WALL_BLOCK_WIDTH + WALL_THICKNESS, WALL_THICKNESS);

    let left_wall_transform = Transform {
        translation: vec3(WALL_LEFT, 0.0, 0.0),
        ..default()
    };

    let left_wall_sprite = Sprite {
        color      : BALL_COLOR,
        custom_size: Some(vertical_wall_size),
        ..default()
    };

    let left_wall_colider = Collider {
        size: vertical_wall_size,
    };

    commands.spawn(WallBundle {
        sprite_bundle: SpriteBundle {
            transform: left_wall_transform,
            sprite   : left_wall_sprite,
            ..default()
        },
        collider: left_wall_colider,
    });

    let right_wall_transform = Transform {
        translation: vec3(WALL_RIGHT, 0.0, 0.0),
        ..default()
    };

    let right_wall_sprite = Sprite {
        color      : BALL_COLOR,
        custom_size: Some(vertical_wall_size),
        ..default()
    };

    commands.spawn(WallBundle {
        sprite_bundle: SpriteBundle {
            transform: right_wall_transform,
            sprite   : right_wall_sprite,
            ..default()
        },
        collider: Collider {
            size: vertical_wall_size,
        },
    });

    let top_wall_transform = Transform {
        translation: vec3(0.0, WALL_UP, 0.0),
        ..default()
    };

    let top_wall_sprite = Sprite {
        color      : BALL_COLOR,
        custom_size: Some(horizontal_wall_size),
        ..default()
    };

    commands.spawn(WallBundle {
        sprite_bundle: SpriteBundle {
            transform: top_wall_transform,
            sprite   : top_wall_sprite,
            ..default()
        },
        collider: Collider {
            size: horizontal_wall_size,
        },
    });

    let bot_wall_transform = Transform {
        translation: vec3(0.0, WALL_BOTTOM, 0.0),
        ..default()
    };

    let bot_wall_sprite = Sprite {
        color      : BALL_COLOR,
        custom_size: Some(horizontal_wall_size),
        ..default()
    };

    commands.spawn(WallBundle {
        sprite_bundle: SpriteBundle {
            transform: bot_wall_transform,
            sprite   : bot_wall_sprite,
            ..default()
        },
        collider: Collider {
            size: horizontal_wall_size,
        },
    });

    commands.spawn(
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    font_size: SCOREBOARD_FONT_SIZE,
                    color    : TEXT_COLOR,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: SCOREBOARD_FONT_SIZE,
                color    : SCORE_COLOR,
                ..default()
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top          : SCOREBOARD_TEXT_PADDING,
            left         : SCOREBOARD_TEXT_PADDING,
            ..default()
        }),
    );

    let offset_x = WALL_LEFT + GAP_BETWEEN_BRICKS_AND_SIDES + BRICK_SIZE.x * 0.5;
    let offset_y = WALL_BOTTOM + GAP_BETWEEN_PADDLE_AND_BRICKS + BRICK_SIZE.y * 0.5;

    let bricks_total_width  = (WALL_RIGHT - WALL_LEFT) - 2. * GAP_BETWEEN_BRICKS_AND_SIDES;
    let bricks_total_height = 
        (WALL_UP - WALL_BOTTOM) - GAP_BETWEEN_BRICKS_AND_CEILING - GAP_BETWEEN_PADDLE_AND_BRICKS;

    let rows = (bricks_total_height / (BRICK_SIZE.y + GAP_BETWEEN_BRICKS)).floor() as i32;
    let cols = (bricks_total_width / (BRICK_SIZE.x + GAP_BETWEEN_BRICKS)).floor() as i32;

    for row in 0..rows {
        for col in 0..cols {
            let brick_pos = vec2(
                offset_x + col as f32 * (BRICK_SIZE.x + GAP_BETWEEN_BRICKS),
                offset_y + row as f32 * (BRICK_SIZE.y + GAP_BETWEEN_BRICKS),
            );

            commands.spawn((
                SpriteBundle {
                    transform: Transform {
                        translation: brick_pos.extend(0.0),
                        ..default()
                    },
                    sprite: Sprite {
                        color      : BRICK_COLOR,
                        custom_size: Some(BRICK_SIZE),
                        ..default()
                    },
                    ..default()
                },
                Brick,
                Collider { size: BRICK_SIZE },
            ));
        }
    }
}

fn paddle_mov(
        input    : Res<Input<KeyCode>>,
        time_step: Res<FixedTime>,
    mut query    : Query<&mut Transform, With<Paddle>>,
) {
    let mut paddle_transform = query.single_mut();

    let mut direction = 0.0;

    if input.pressed(KeyCode::A) {
        direction -= 1.0;
        println!("Left")
    }
    if input.pressed(KeyCode::D) {
        direction += 1.0;
        println!("Right")
    }

    let mut new_x =
        paddle_transform.translation.x + direction * PADDLE_SPEED * time_step.period.as_secs_f32();

    new_x = new_x.min(WALL_RIGHT - (WALL_THICKNESS + PADDLE_SIZE.x) * 0.5);
    new_x = new_x.max(WALL_LEFT + (WALL_THICKNESS + PADDLE_SIZE.x) * 0.5);

    paddle_transform.translation.x = new_x;
}

fn apply_velocity(
    mut query    : Query<(&mut Transform, &Velocity)>,
        time_step: Res<FixedTime>) {
    let dt = time_step.period.as_secs_f32();
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * dt;
        transform.translation.y += velocity.y * dt;
    }
}

fn check_ball_collisions(
    mut commands      : Commands,
    mut score         : ResMut<Scoreboard>,
    mut ball_query    : Query<(&mut Velocity, &Transform, &Ball)>,
    mut collider_query: Query<(Entity, &Transform, &Collider, Option<&Brick>)>,
) {
    for (mut ball_velocity, ball_transform, ball) in &mut ball_query {
        for (other_entity, transform, other, opt_brick) in &mut collider_query {
            let collision = collide(
                ball_transform.translation,
                ball.size,
                transform.translation,
                other.size,
            );

            let mut reflect_x = false;
            let mut reflect_y = false;

            if let Some(collision) = collision {
                match collision {
                    Collision::Left   => reflect_x = ball_velocity.x > 0.0,
                    Collision::Right  => reflect_x = ball_velocity.x < 0.0,
                    Collision::Top    => reflect_y = ball_velocity.y < 0.0,
                    Collision::Bottom => reflect_y = ball_velocity.y > 0.0,
                    Collision::Inside => {}
                }

                if reflect_x {
                    ball_velocity.x *= -1.0;
                }

                if reflect_y {
                    ball_velocity.y *= -1.0;
                }

                if opt_brick.is_some() {
                    commands.entity(other_entity).despawn();
                    score.score += 1;
                }
            }
        }
    }
}

fn update_scoreboard(
        score: Res<Scoreboard>,
    mut query: Query<&mut Text>
) {
    let mut text = query.single_mut();
    text.sections[1].value = score.score.to_string();
}
