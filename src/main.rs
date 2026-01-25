use bevy::{color::palettes::basic::WHITE, ecs::query, prelude::*};

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

fn move_rectangle(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Ball>>,
    velocity: Res<Velocity>,
) {
    for mut transform in &mut query {
        transform.translation.x += 300. * velocity.0 * time.delta_secs();
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
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        for mut transform in &mut player_query {
            println!("window size is {}", window.height());
            println!("y position is {}", transform.translation.y);
            if transform.translation.y > -window.height() / 2. + 52. {
                transform.translation.y -= 5.
            }
        }
    }
}
fn collision(
    ball_query: Query<&Transform, With<Ball>>,
    paddle_query: Query<&Transform, With<Paddle>>,
    mut velocity: ResMut<Velocity>,
) {
    for transform in ball_query {
        let paddle_translate = paddle_query.single().unwrap().translation.x;
        println!(
            "transform of ball is {} and of paddle {}",
            transform.translation.x, paddle_translate
        );
        if transform.translation.x == paddle_translate {
            velocity.0 *= -1.
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
        Paddle,
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(Color::from(WHITE))),
        Transform::default().with_scale(Vec3::splat(32.0)),
    ));
    commands.spawn((
        Oponent,
        Paddle,
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(Color::from(WHITE))),
        Transform::default()
            .with_scale(Vec3::new(16.0, 100.0, 1.0))
            .with_translation(Vec3::new(window.width() / 2. - 32., 0., 0.)),
    ));
    commands.spawn((
        Player,
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(Color::from(WHITE))),
        Transform::default()
            .with_scale(Vec3::new(16.0, 100.0, 1.0))
            .with_translation(Vec3::new(-window.width() / 2. + 32., 0., 0.)),
    ));
}
fn main() {
    App::new()
        .insert_resource(Velocity(1.0))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (keyboard_control, move_rectangle, collision).chain(),
        )
        .run();
}
