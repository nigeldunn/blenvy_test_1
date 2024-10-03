use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_world);
    }
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