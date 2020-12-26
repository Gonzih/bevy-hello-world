use bevy::{
    // diagnostic::{FrameTimeDiagnosticsPlugin, PrintDiagnosticsPlugin},
    input::mouse::{MouseButtonInput, MouseMotion, MouseWheel},
    prelude::*,
    window::CursorMoved,
    window::WindowId,
    winit::WinitWindows,
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
            yaw: 34.0,
            pitch: 12.0,
            sensitivity: 30.0,
        };
    }
}

fn main() {
    App::build()
        // resources
        .init_resource::<State>()
        .add_resource(Msaa { samples: 4 })
        // plugins
        .add_plugin(RapierRenderPlugin)
        .add_plugin(RapierPhysicsPlugin)
        .add_plugins(DefaultPlugins)
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // .add_plugin(PrintDiagnosticsPlugin::default())
        // setup
        .add_startup_system(setup.system())
        // loop
        .add_system(rotate_player.system())
        .add_system(move_player.system())
        .add_system(mouse_capture_system.system())
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
            transform: Transform::from_translation(Vec3::new(0.0, 5.0, 0.0)),
            ..Default::default()
        })
        // camera
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(35.0, -1.0, 35.0)),
            ..Default::default()
        })
        .with(Camera);

    let mut rng = StdRng::from_entropy();
    let player_handle = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
    let player_body = RigidBodyBuilder::new_dynamic().translation(0.0, 100.0, 0.0);
    let player_collider = ColliderBuilder::cuboid(1.0, 1.0, 1.0);

    let platform_handle = meshes.add(Mesh::from(shape::Plane { size: 100.0 }));
    let platform_body = RigidBodyBuilder::new_static().translation(-5.0, -5.0, -5.0);
    let platform_collider = ColliderBuilder::cuboid(100.0, 0.0, 100.0);

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
    mut player_query: Query<(&mut Player, &mut Transform)>,
    mut camera_query: Query<(&Camera, &mut Transform)>,
) {
    let mut delta = Vec2::zero();
    for event in state.mouse_motion_event_reader.iter(&mouse_motion_events) {
        delta += event.delta;
    }

    let mut yaw_rad = 0.0;
    let mut pitch_rad = 0.0;

    for (mut player, mut transform) in player_query.iter_mut() {
        player.yaw -= delta.x * player.sensitivity * time.delta_seconds();
        player.pitch += delta.y * player.sensitivity * time.delta_seconds();

        if player.pitch < -90.0 {
            player.pitch = -90.0;
        } else if player.pitch > 90.0 {
            player.pitch = 90.0
        }

        yaw_rad = player.yaw.to_radians();
        pitch_rad = player.pitch.to_radians();
        transform.rotation = Quat::from_axis_angle(Vec3::unit_y(), yaw_rad);

        println!("Player yaw = {}, pitch = {}", player.yaw, player.pitch);
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
    mut query: Query<(&Camera, &mut Transform)>,
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
    if keyboard_input.pressed(KeyCode::LShift) {
        offset.y -= 10.0;
    }

    for (_, mut transform) in query.iter_mut() {
        transform.translation += offset * time.delta_seconds();
    }
}

fn mouse_capture_system(
    mut state: ResMut<State>,
    mouse_button_events: Res<Events<MouseButtonInput>>,
    windows: Res<WinitWindows>,
) {
    if let Some(event) = state.mouse_button_event_reader.latest(&mouse_button_events) {
        let window = windows.get_window(WindowId::primary()).unwrap();
        match event.button {
            MouseButton::Left => {
                window.set_cursor_grab(true).unwrap();
                window.set_cursor_visible(false);
            }
            MouseButton::Right => {
                window.set_cursor_grab(false).unwrap();
                window.set_cursor_visible(true);
            }
            _ => (),
        }
    }
}
