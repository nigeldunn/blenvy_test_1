use bevy::prelude::*;
use bevy_third_person_camera::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ThirdPersonCameraPlugin);
        app.add_systems(Startup, spawn_camera);
    }
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
