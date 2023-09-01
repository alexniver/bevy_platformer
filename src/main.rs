use bevy::{
    input::common_conditions::input_toggle_active,
    prelude::*,
    sprite::MaterialMesh2dBundle,
    window::{close_on_esc, WindowResolution},
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_platformer::platform::PlatformBundle;
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(COLOR_BACKGROUND))
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Bevy Platformer".to_string(),
                    resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                    resizable: false,
                    ..default()
                }),
                ..default()
            }),
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(200.0),
            RapierDebugRenderPlugin::default(),
        ))
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Tab)),
        )
        .add_systems(Startup, setup)
        .add_systems(Update, close_on_esc)
        .add_systems(Update, player_move)
        .add_systems(Update, player_jump)
        .add_systems(Update, velocity_y)
        .add_systems(Update, check_grounded)
        .run();
}

const WINDOW_WIDTH: f32 = 1920.0;
const WINDOW_HEIGHT: f32 = 1080.0;

const WINDOW_BOTTOM_Y: f32 = -WINDOW_HEIGHT / 2.0;
const WINDOW_LEFT_X: f32 = -WINDOW_WIDTH / 2.0;

const COLOR_BACKGROUND: Color = Color::rgb(0.2, 0.5, 0.2);
const COLOR_PLAYER: Color = Color::rgb(0.9, 0.4, 0.2);

const GROUND_HEIGHT: f32 = 20.0;

const PLAYER_SPEED: f32 = 200.0;

const JUMP_VELOCITY: f32 = 500.0;
const FALL_VELOCITY: f32 = 1000.0;

#[derive(Debug, Component)]
struct Jump;
#[derive(Debug, Component)]
struct VelocityY(f32);

fn setup(
    mut commands: Commands,
    mut meshs: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(PlatformBundle::new(
        Vec3::new(0.0, WINDOW_BOTTOM_Y + GROUND_HEIGHT / 2.0, 0.0),
        Vec3::new(WINDOW_WIDTH, GROUND_HEIGHT, 1.0),
    ));

    commands.spawn(PlatformBundle::new(
        Vec3::new(
            WINDOW_LEFT_X + 500.0,
            WINDOW_BOTTOM_Y + GROUND_HEIGHT + 100.0,
            1.0,
        ),
        Vec3::new(100.0, 20.0, 1.0),
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshs.add(shape::Circle::default().into()).into(),
            material: materials.add(ColorMaterial::from(COLOR_PLAYER)),
            transform: Transform {
                translation: Vec3::new(WINDOW_LEFT_X + 100.0, WINDOW_BOTTOM_Y + 100.0, 0.0),
                scale: Vec3::new(30.0, 30.0, 1.0),
                ..default()
            },
            ..default()
        },
        RigidBody::KinematicPositionBased,
        Collider::ball(0.5),
        KinematicCharacterController::default(),
        VelocityY(0.0),
    ));
}

fn player_move(
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<&mut KinematicCharacterController>,
) {
    if player_query.is_empty() {
        return;
    }

    let mut player = player_query.single_mut();
    let mut transform = Vec2::ZERO;
    if keyboard.pressed(KeyCode::A) {
        transform.x -= time.delta_seconds() * PLAYER_SPEED;
    }
    if keyboard.pressed(KeyCode::D) {
        transform.x += time.delta_seconds() * PLAYER_SPEED;
    }

    match player.translation {
        Some(t) => player.translation = Some(t + transform),
        None => player.translation = Some(transform),
    }
}

fn player_jump(
    mut commands: Commands,
    keyboard: Res<Input<KeyCode>>,
    mut player_query: Query<
        (Entity, &mut VelocityY),
        (With<KinematicCharacterController>, Without<Jump>),
    >,
) {
    if player_query.is_empty() {
        return;
    }

    if keyboard.just_pressed(KeyCode::Space) {
        let (entity, mut velocity_y) = player_query.single_mut();
        velocity_y.0 = JUMP_VELOCITY;
        commands.entity(entity).insert(Jump);
    }
}

fn velocity_y(
    mut player_query: Query<(&mut KinematicCharacterController, &mut VelocityY)>,
    time: Res<Time>,
) {
    if player_query.is_empty() {
        return;
    }

    let (mut player, mut velocity_y) = player_query.single_mut();

    let mut transform = Vec2::ZERO;
    transform.y += time.delta_seconds() * velocity_y.0;

    match player.translation {
        Some(t) => player.translation = Some(transform + t),
        None => player.translation = Some(transform),
    }

    velocity_y.0 -= FALL_VELOCITY * time.delta_seconds();
    velocity_y.0 = (-FALL_VELOCITY).max(velocity_y.0);
}

fn check_grounded(
    mut commands: Commands,
    mut player_query: Query<
        (Entity, &mut KinematicCharacterControllerOutput, &VelocityY),
        With<Jump>,
    >,
) {
    if player_query.is_empty() {
        return;
    }
    let (entity, output, velocity_y) = player_query.single_mut();
    if output.grounded && velocity_y.0 < 0.0 {
        commands.entity(entity).remove::<Jump>();
    }
}
