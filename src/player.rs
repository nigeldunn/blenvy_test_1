use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use bevy_third_person_camera::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build( &self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<Action>::default());
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, player_actions);
    }
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
enum Action {
    Attack,
    Ability1,
    Ability2,
    Ultimate,
    Dash,
    Forward,
    Backward,
    Left,
    Right,
    Use,
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Speed(f32);


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

    let input_map = InputMap::new([
        (Action::Dash, KeyCode::Space),
        (Action::Ability1, KeyCode::KeyQ),
        (Action::Ability2, KeyCode::KeyE),
        (Action::Ultimate, KeyCode::KeyR),
        (Action::Forward, KeyCode::KeyW),
        (Action::Backward, KeyCode::KeyS),
        (Action::Left, KeyCode::KeyA),
        (Action::Right, KeyCode::KeyD),
        (Action::Use, KeyCode::KeyF),
    ])
    .with(Action::Attack, MouseButton::Left);

    commands
        .spawn(player)
        .insert(InputManagerBundle::with_map(input_map));
}

fn player_actions(
    time: Res<Time>,
    mut player_q: Query<(&mut Transform, &Speed), With<Player>>,
    cam_q: Query<&Transform, (With<Camera3d>, Without<Player>)>,
    action_q: Query<&ActionState<Action>, With<Player>>,
) {
    let action_state = action_q.single();

    for (mut player_transform, player_speed) in player_q.iter_mut() {
        let cam = match cam_q.get_single() {
            Ok(c) => c,
            Err(e) => Err(format!("Error retrieving camera: {}", e)).unwrap(),
        };

        let mut direction = Vec3::ZERO;

        // forward
        if action_state.pressed(&Action::Forward) {
            direction += *cam.forward();
        }

        // back
        if action_state.pressed(&Action::Backward) {
            direction += *cam.back();
        }

        // left
        if action_state.pressed(&Action::Left) {
            direction += *cam.left();
        }

        // right
        if action_state.pressed(&Action::Right) {
            direction += *cam.right();
        }

        let mut movement_speed = player_speed.0;
        if action_state.pressed(&Action::Dash) {
            movement_speed = movement_speed * 3.;
            println!("Dashing...");
        }

        direction.y = 0.0;
        let movement = direction.normalize_or_zero() * movement_speed * time.delta_seconds();
        player_transform.translation += movement;

        // rotate player to face direction he is currently moving
        if direction.length_squared() > 0.0 {
            player_transform.look_to(direction, Vec3::Y);
        }

        if action_state.just_pressed(&Action::Attack) {
            println!("I'm attacking!");
        }
        if action_state.just_pressed(&Action::Ability1) {
            println!("Ability 1!");
        }
        if action_state.just_pressed(&Action::Ability2) {
            println!("Ability 2!");
        }
        if action_state.just_pressed(&Action::Ultimate) {
            println!("GET REKT!!!");
        }
        if action_state.just_pressed(&Action::Use) {
            println!("Use!");
        }
    }
}