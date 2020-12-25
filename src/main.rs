use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, PrintDiagnosticsPlugin},
    input::mouse::{MouseButtonInput, MouseMotion, MouseWheel},
    prelude::*,
    window::CursorMoved,
};
use bevy_rapier3d::physics::RapierPhysicsPlugin;
use bevy_rapier3d::rapier::dynamics::RigidBodyBuilder;
use bevy_rapier3d::rapier::geometry::ColliderBuilder;
use bevy_rapier3d::render::RapierRenderPlugin;
use rand::{rngs::StdRng, Rng, SeedableRng};

struct Camera;

struct Player {
    pub yaw: f32,
    pub pitch: f32,
    pub sensitivity: f32,
}

impl Player {
    fn new() -> Self {
        return Self {
            yaw: 0.0,
            pitch: 0.0,
            sensitivity: 30.0,
        };
    }
}

fn main() {
    App::build()
        .init_resource::<State>()
        .add_resource(Msaa { samples: 4 })
        .add_plugin(RapierRenderPlugin)
        .add_plugin(RapierPhysicsPlugin)
        .add_plugins(DefaultPlugins)
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // .add_plugin(PrintDiagnosticsPlugin::default())
        .add_startup_system(setup.system())
        .add_system(rotate_player.system())
        .add_system(move_player.system())
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
            transform: Transform::from_translation(Vec3::new(35.0, 35.0, 35.0)),
            ..Default::default()
        })
        .with(Camera);

    let mut rng = StdRng::from_entropy();
    let player_handle = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
    let player_body = RigidBodyBuilder::new_dynamic();
    let player_collider = ColliderBuilder::cuboid(1.0, 1.0, 1.0);

    let platform_translation = Vec3::new(-5.0, -60.0, -5.0);
    let platform_handle = meshes.add(Mesh::from(shape::Cube { size: 100.0 }));
    let platform_body = RigidBodyBuilder::new_static().translation(-5.0, -60.0, -5.0);
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
            transform: Transform::from_translation(platform_translation),
            ..Default::default()
        })
        .with(platform_body)
        .with(platform_collider);
}

#[derive(Default)]
struct State {
    mouse_button_event_reader: EventReader<MouseButtonInput>,
    mouse_motion_event_reader: EventReader<MouseMotion>,
    cursor_moved_event_reader: EventReader<CursorMoved>,
    mouse_wheel_event_reader: EventReader<MouseWheel>,
}

fn rotate_player(
    mut state: ResMut<State>,
    time: Res<Time>,
    mouse_motion_events: Res<Events<MouseMotion>>,
    // mut player_query: Query<(&mut Player, &mut Transform)>,
    mut player_query: Query<(&mut Player)>,
    mut camera_query: Query<(&Camera, &mut Transform)>,
) {
    let mut delta = Vec2::zero();
    for event in state.mouse_motion_event_reader.iter(&mouse_motion_events) {
        println!("{:?}", event);
        delta += event.delta;
    }

    let mut yaw_rad = 0.0;
    let mut pitch_rad = 0.0;

    // for (mut player, mut transform) in player_query.iter_mut() {
    for (mut player) in player_query.iter_mut() {
        player.yaw -= delta.x * player.sensitivity * time.delta_seconds();
        player.pitch += delta.y * player.sensitivity * time.delta_seconds();
        yaw_rad = player.yaw.to_radians();
        pitch_rad = player.pitch.to_radians();
        // transform.rotation = Quat::from_axis_angle(Vec3::unit_y(), yaw_rad)
        //     * Quat::from_axis_angle(-Vec3::unit_x(), pitch_rad);
    }

    for (_, mut transform) in camera_query.iter_mut() {
        transform.rotation = Quat::from_axis_angle(Vec3::unit_y(), yaw_rad)
            * Quat::from_axis_angle(-Vec3::unit_x(), pitch_rad);
    }
}

fn move_player(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut _materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(&Player, &mut Transform, &Handle<StandardMaterial>)>,
) {
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

    for (_player, mut transform, _material_handle) in query.iter_mut() {
        // transform.rotate
        transform.translation += offset * time.delta_seconds();
    }
}
