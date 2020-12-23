use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, PrintDiagnosticsPlugin},
    prelude::*,
};
use rand::{rngs::StdRng, Rng, SeedableRng};
use bevy_rapier3d::physics::RapierPhysicsPlugin;

fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_plugin(RapierPhysicsPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(PrintDiagnosticsPlugin::default())
        .add_startup_system(setup.system())
        .add_system(move_cubes.system())
        .run();
}

fn setup(
    commands: &mut Commands,
    // asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        // light
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, -4.0, 5.0)),
            ..Default::default()
        })
        // camera
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 15.0, 150.0))
                .looking_at(Vec3::default(), Vec3::unit_y()),
            ..Default::default()
        });

    let mut rng = StdRng::from_entropy();
    let cube_handle = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
    for _ in 0..10000 {
        commands.spawn(PbrBundle {
            mesh: cube_handle.clone(),
            material: materials.add(StandardMaterial {
                albedo: Color::rgb(
                    rng.gen_range(0.0, 1.0),
                    rng.gen_range(0.0, 1.0),
                    rng.gen_range(0.0, 1.0),
                ),
                ..Default::default()
            }),
            transform: Transform::from_translation(Vec3::new(
                rng.gen_range(-50.0, 50.0),
                rng.gen_range(-50.0, 50.0),
                0.0,
            )),
            ..Default::default()
        });
    }
}

fn move_cubes(
    time: Res<Time>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(&mut Transform, &Handle<StandardMaterial>)>,
) {
    for (mut transform, material_handle) in query.iter_mut() {
        let material = materials.get_mut(material_handle).unwrap();
        transform.translation += Vec3::new(1.0, 0.0, 0.0) * time.delta_seconds();
        material.albedo =
            Color::BLUE * Vec3::splat((3.0 * time.seconds_since_startup() as f32).sin());
    }
}
