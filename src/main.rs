use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, PrintDiagnosticsPlugin},
    prelude::*,
};
use bevy_rapier3d::physics::RapierPhysicsPlugin;
use bevy_rapier3d::rapier::dynamics::RigidBodyBuilder;
use bevy_rapier3d::rapier::geometry::ColliderBuilder;
use rand::{rngs::StdRng, Rng, SeedableRng};

struct Player {}

impl Player {
    fn new() -> Player {
        return Player {};
    }
}

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
            transform: Transform::from_translation(Vec3::new(35.0, 35.0, 35.0))
                .looking_at(Vec3::default(), Vec3::unit_y()),
            ..Default::default()
        });

    let mut rng = StdRng::from_entropy();
    let player_handle = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
    let player_body = RigidBodyBuilder::new_dynamic();
    let player_collider = ColliderBuilder::cuboid(1.0, 1.0, 1.0);

    let platform_handle = meshes.add(Mesh::from(shape::Cube { size: 100.0 }));
    let platform_body = RigidBodyBuilder::new_static();
    let platform_collider = ColliderBuilder::cuboid(100.0, 100.0, 100.0);

    commands
        .spawn(PbrBundle {
            mesh: player_handle.clone(),
            material: materials.add(StandardMaterial {
                albedo: Color::rgb(
                    rng.gen_range(0.0..1.0),
                    rng.gen_range(0.0..1.0),
                    rng.gen_range(0.0..1.0),
                ),
                ..Default::default()
            }),
            transform: Transform::from_translation(Vec3::new(0.0, 10.0, 0.0)),
            ..Default::default()
        })
        .with(Player::new())
        .with(player_body)
        .with(player_collider)
        // platform
        .spawn(PbrBundle {
            mesh: platform_handle.clone(),
            material: materials.add(StandardMaterial {
                albedo: Color::rgb(
                    rng.gen_range(0.0..1.0),
                    rng.gen_range(0.0..1.0),
                    rng.gen_range(0.0..1.0),
                ),
                ..Default::default()
            }),
            transform: Transform::from_translation(Vec3::new(-50.0, -60.0, -50.0)),
            ..Default::default()
        })
        .with(platform_body)
        .with(platform_collider);
}

fn move_cubes(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut _materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(&Player, &mut Transform, &Handle<StandardMaterial>)>,
) {
    for (_player, mut transform, _material_handle) in query.iter_mut() {
        // let material = materials.get_mut(material_handle).unwrap();

        let mut offset = Vec3::new(0.0, 0.0, 0.0);
        if keyboard_input.pressed(KeyCode::Left) {
            offset.x -= 10.0;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            offset.x += 10.0;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            offset.z -= 10.0;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            offset.z += 10.0;
        }
        if keyboard_input.pressed(KeyCode::Space) {
            offset.y += 10.0;
        }
        if keyboard_input.pressed(KeyCode::C) {
            offset.z -= 10.0;
        }

        transform.translation += offset * time.delta_seconds();
        // material.albedo =
        //     Color::BLUE * Vec3::splat((3.0 * time.seconds_since_startup() as f32).sin());
    }
}
