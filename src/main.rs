use bevy::{
    color::palettes::basic::WHITE,
    ecs::query::{self, WorldQuery},
    math::bounding::{Aabb2d, IntersectsVolume},
    mesh,
    prelude::*,
};

#[derive(Component)]
struct Player;
#[derive(Component)]
struct ScoreBoard;
#[derive(Component)]
struct Oponent;
#[derive(Component)]
struct Paddle;

#[derive(Component)]
enum Side {
    PLAYER,
    OPONENT,
}

#[derive(Event)]
struct BallCollided {
    side: Side,
}
#[derive(Resource)]
struct Score {
    player: u8,
    oponent: u8,
}

#[derive(Component)]
struct Ball;

#[derive(Resource)]
struct PlayerScore(u8);
#[derive(Resource)]
struct OponentScore(u8);

#[derive(Component)]
struct PlayerScoreTag;

#[derive(Component)]
struct OponentScoreTag;
#[derive(Resource)]
struct Velocity(f32);

#[derive(Resource)]
struct Direction(Vec3);

#[derive(Component)]
struct Collider;
fn move_rectangle(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Ball>>,
    velocity: Res<Velocity>,
    direction: Res<Direction>,
) {
    for mut transform in &mut query {
        transform.translation.x += (direction.0[0] * velocity.0) * time.delta_secs();
        transform.translation.y += (direction.0[1] * velocity.0) * time.delta_secs();

        //transform.translation.x += (300. * velocity.0) * time.delta_secs();
    }
}

fn keyboard_control(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    window: Single<&Window>,
) {
    if keyboard_input.pressed(KeyCode::KeyW) {
        for mut transform in &mut player_query {
            if transform.translation.y <= window.height() / 2. - 52. {
                transform.translation.y += 5.
            }
        }
    };
    if keyboard_input.pressed(KeyCode::KeyS) {
        for mut transform in &mut player_query {
            if transform.translation.y > -window.height() / 2. + 52. {
                transform.translation.y -= 5.
            }
        }
    };
}
fn oponent_keyboard_control(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Oponent>>,
    window: Single<&Window>,
) {
    if keyboard_input.pressed(KeyCode::ArrowUp) {
        for mut transform in &mut player_query {
            if transform.translation.y <= window.height() / 2. - 52. {
                transform.translation.y += 5.
            }
        }
    };
    if keyboard_input.pressed(KeyCode::ArrowDown) {
        for mut transform in &mut player_query {
            if transform.translation.y > -window.height() / 2. + 52. {
                transform.translation.y -= 5.
            }
        }
    };
}

fn detect_collision(
    ball_query: Query<&Transform, With<Ball>>,
    collider_query: Query<&Transform, With<Collider>>,
    mut velocity: ResMut<Velocity>,
    mut direction: ResMut<Direction>,
) {
    for transform in ball_query {
        let ball_bounding_box = Aabb2d::new(
            Vec2 {
                x: transform.translation.x,
                y: transform.translation.y,
            },
            Vec2 { x: 16.0, y: 16.0 },
        );
        for collider_transform in collider_query {
            let collider_bounding_box = Aabb2d::new(
                Vec2 {
                    x: collider_transform.translation.x,
                    y: collider_transform.translation.y,
                },
                Vec2 { x: 8., y: 50.0 },
            );
            if ball_bounding_box.intersects(&collider_bounding_box) {
                velocity.0 *= -1.1;
                direction.0[1] *= -1.;
            }
        }
    }
}
fn bounce_off_walls(
    ball_query: Single<&Transform, With<Ball>>,
    window: Single<&Window>,
    mut direction: ResMut<Direction>,
) {
    let ball_bounding_box = Aabb2d::new(
        Vec2 {
            x: ball_query.translation.x,
            y: ball_query.translation.y,
        },
        Vec2 { x: 16.0, y: 16.0 },
    );

    let top_bounding = Aabb2d::new(
        Vec2 {
            x: 0.,
            y: (window.height() / 2.0) + 2.,
        },
        Vec2 {
            x: window.width() / 2.,
            y: 1.5,
        },
    );

    let bottom_bounding = Aabb2d::new(
        Vec2 {
            x: 0.,
            y: -((window.height() / 2.0) + 2.0),
        },
        Vec2 {
            x: window.width() / 2.,
            y: 1.5,
        },
    );
    if ball_bounding_box.intersects(&top_bounding) || ball_bounding_box.intersects(&bottom_bounding)
    {
        //direction.0[0] *= -1.;
        direction.0[1] *= -1.;
    }
}
fn update_score(
    _collided: On<BallCollided>,
    mut score: ResMut<Score>,
    mut score_board: Query<(&Side, &mut Text2d), With<ScoreBoard>>,
) {
    match _collided.side {
        Side::OPONENT => {
            score.player += 1;
        }
        Side::PLAYER => {
            score.oponent += 1;
        }
    }
    for (side, mut text) in &mut score_board {
        match side {
            Side::OPONENT => text.0 = score.oponent.to_string(),
            Side::PLAYER => text.0 = score.player.to_string(),
        }
    }
}

fn score_goal(
    mut commands: Commands,
    mut ball_query: Single<&mut Transform, With<Ball>>,
    window: Single<&Window>,
) {
    let ball_bounding_box = Aabb2d::new(
        Vec2 {
            x: ball_query.translation.x,
            y: ball_query.translation.y,
        },
        Vec2 { x: 16.0, y: 16.0 },
    );
    let right_bounding = Aabb2d::new(
        Vec2 {
            x: window.width() / 2. + 2.,
            y: 0.,
        },
        Vec2 {
            x: 1.5,
            y: window.height() / 2.,
        },
    );
    let left_bounding = Aabb2d::new(
        Vec2 {
            x: -(window.width() / 2. + 2.),
            y: 0.,
        },
        Vec2 {
            x: 1.5,
            y: window.height() / 2.,
        },
    );
    if ball_bounding_box.intersects(&left_bounding) {
        commands.trigger(BallCollided { side: Side::PLAYER });
        //player_score.0 += 1; //direction.0[0] *= -1.;
        //player_score_text.0 = player_score.0.to_string();
        ball_query.translation.x = 0.;
        ball_query.translation.y = 0.;
    } else if ball_bounding_box.intersects(&right_bounding) {
        commands.trigger(BallCollided {
            side: Side::OPONENT,
        });
        ball_query.translation.x = 0.;
        ball_query.translation.y = 0.;
    }
}
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Single<&Window>,
    score: Res<Score>,
) {
    commands.spawn(Camera2d);
    commands.spawn((
        Ball,
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(Color::from(WHITE))),
        Transform::default().with_scale(Vec3::splat(32.0)),
    ));
    commands.spawn((
        Oponent,
        Paddle,
        Collider,
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(Color::from(WHITE))),
        Transform::default()
            .with_scale(Vec3::new(16.0, 100.0, 1.0))
            .with_translation(Vec3::new(window.width() / 2. - 32., 0., 0.)),
    ));
    commands.spawn((
        Player,
        Paddle,
        Collider,
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(Color::from(WHITE))),
        Transform::default()
            .with_scale(Vec3::new(16.0, 100.0, 1.0))
            .with_translation(Vec3::new(-window.width() / 2. + 32., 0., 0.)),
    ));

    commands.spawn((
        ScoreBoard,
        Side::OPONENT,
        Text2d::new(score.oponent.to_string()),
        TextLayout::new_with_justify(Justify::Center),
        Transform::from_translation(Vec3::new(10., window.height() / 2. - 20., 0.0)),
    ));
    commands.spawn((
        ScoreBoard,
        Side::PLAYER,
        Text2d::new(score.player.to_string()),
        TextLayout::new_with_justify(Justify::Center),
        Transform::from_translation(Vec3::new(-10., window.height() / 2. - 20., 0.0)),
    ));
}

fn main() {
    App::new()
        .insert_resource(Velocity(100.0))
        .insert_resource(Score {
            player: 0,
            oponent: 0,
        })
        .insert_resource(Direction(Vec3::new(1., -1., 0.)))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                keyboard_control,
                oponent_keyboard_control,
                move_rectangle,
                detect_collision,
                bounce_off_walls,
                score_goal,
            ),
        )
        .add_observer(update_score)
        .run();
}
