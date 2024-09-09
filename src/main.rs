use std::f32::consts::PI;

use bevy::{
    prelude::*, 
    pbr::CascadeShadowConfigBuilder,
};
use leafwing_input_manager::prelude::*;
use bevy_third_person_camera::*;

fn main() {
    App::new()
    .add_plugins((DefaultPlugins, ThirdPersonCameraPlugin))
    .add_plugins(InputManagerPlugin::<Action>::default())
    .add_systems(Startup, (spawn_player, spawn_world, spawn_camera))
    .add_systems(Update, player_movement)
    .run();
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
enum Action {
    Rotate,
    Attack,
    Ability1,
    Ability2,
    Dash,
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Speed(f32);


// fn spawn_player (mut commands: Commands) {
//     let input_map = InputMap::new([
//         (Action::Dash, KeyCode::Space),
//         (Action::Ability1, KeyCode::KeyQ),
//         (Action::Ability2, KeyCode::KeyE),
//     ])
//     .with(Action::Attack, MouseButton::Left)
//     .with(Action::Rotate, MouseButton::Right);
//
//     commands
//         .spawn(InputManagerBundle::with_map(input_map))
//         .insert(Player);
// }
fn spawn_player(mut commands: Commands, assets: Res<AssetServer>) {
    let player = (
        SceneBundle {
            scene: assets.load("Player.gltf#Scene0"),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        Player,
        ThirdPersonCameraTarget, // ADD THIS
        Speed(2.5),
    );

    commands.spawn(player);
}

fn spawn_camera(mut commands: Commands) {
    let camera = (
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        ThirdPersonCamera {
            cursor_lock_toggle_enabled: false,
            mouse_orbit_button_enabled: true,
            mouse_orbit_button: MouseButton::Right,
            gamepad_settings: CustomGamepadSettings { ..default() },
            zoom_enabled: true,        // default
            zoom: Zoom::new(1.5, 10.0), // default
            ..default()
        },
    );
    commands.spawn(camera);
}

fn spawn_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,

    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor = PbrBundle {
        mesh: meshes.add(Mesh::from(Plane3d::default().mesh().size(15.0, 15.0))),
        material: materials.add(Color::srgb(0.11, 0.27, 0.16)),
        ..default()
    };

    let light = PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            intensity: 1500.0 * 1000.0,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 7.5, 0.0),
        ..default()
    };

    commands.spawn(floor);
    commands.spawn(light);


}


// fn player_actions (query: Query<&ActionState<Action>, With<Player>>) {
//     let action_state = query.single();
//     if action_state.just_pressed(&Action::Attack) {
//         println!("I'm attacking!");
//     }
//     if action_state.just_pressed(&Action::Rotate) {
//         println!("Rotating...");
//     }
//     if action_state.just_pressed(&Action::Ability1) {
//         println!("Ability 1!");
//     }
//     if action_state.just_pressed(&Action::Ability2) {
//         println!("Ability 2!");
//     }
//     if action_state.just_pressed(&Action::Dash) {
//         println!("Dashing...");
//     }
// }

fn player_movement(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut player_q: Query<(&mut Transform, &Speed), With<Player>>,
    cam_q: Query<&Transform, (With<Camera3d>, Without<Player>)>,
) {
    for (mut player_transform, player_speed) in player_q.iter_mut() {
        let cam = match cam_q.get_single() {
            Ok(c) => c,
            Err(e) => Err(format!("Error retrieving camera: {}", e)).unwrap(),
        };

        let mut direction = Vec3::ZERO;

        // forward
        if keys.pressed(KeyCode::KeyW) {
            direction += *cam.forward();
        }

        // back
        if keys.pressed(KeyCode::KeyS) {
            direction += *cam.back();
        }

        // left
        if keys.pressed(KeyCode::KeyA) {
            direction += *cam.left();
        }

        // right
        if keys.pressed(KeyCode::KeyD) {
            direction += *cam.right();
        }

        direction.y = 0.0;
        let movement = direction.normalize_or_zero() * player_speed.0 * time.delta_seconds();
        player_transform.translation += movement;

        // rotate player to face direction he is currently moving
        if direction.length_squared() > 0.0 {
            player_transform.look_to(direction, Vec3::Y);
        }
    }
}