use bevy::{
    color::palettes::basic::WHITE,
    ecs::query,
    math::bounding::{Aabb2d, IntersectsVolume},
    mesh,
    prelude::*,
};

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Oponent;
#[derive(Component)]
enum Role {
    Player,
    Oponent,
}
#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Ball;

#[derive(Resource)]
struct Score(u8);

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
            println!("window size is {}", window.height());
            println!("y position is {}", transform.translation.y);

            if transform.translation.y <= window.height() / 2. - 52. {
                transform.translation.y += 5.
            }
        }
    };
    if keyboard_input.pressed(KeyCode::KeyS) {
        for mut transform in &mut player_query {
            println!("window size is {}", window.height());
            println!("y position is {}", transform.translation.y);
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
            println!("window size is {}", window.height());
            println!("y position is {}", transform.translation.y);

            if transform.translation.y <= window.height() / 2. - 52. {
                transform.translation.y += 5.
            }
        }
    };
    if keyboard_input.pressed(KeyCode::ArrowDown) {
        for mut transform in &mut player_query {
            println!("window size is {}", window.height());
            println!("y position is {}", transform.translation.y);
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
                velocity.0 *= -1.
            }
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Single<&Window>,
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
}

fn main() {
    App::new()
        .insert_resource(Velocity(100.0))
        .insert_resource(Direction(Vec3::new(1., 1., 0.)))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                keyboard_control,
                oponent_keyboard_control,
                move_rectangle,
                detect_collision,
            )
                .chain(),
        )
        .run();
}
