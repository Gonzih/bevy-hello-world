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

impl Default for Player {
    fn default() -> Self {
        Self {
            yaw: 90.0,
            pitch: 0.0,
            sensitivity: 30.0,
        }
    }
}

struct Options {
    pub key_backward: KeyCode,
    pub key_forward: KeyCode,
    pub key_left: KeyCode,
    pub key_right: KeyCode,
    pub key_up: KeyCode,
    pub key_down: KeyCode,
    pub speed: f32,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            key_backward: KeyCode::Down,
            key_forward: KeyCode::Up,
            key_left: KeyCode::Left,
            key_right: KeyCode::Right,
            key_up: KeyCode::Space,
            key_down: KeyCode::LShift,
            speed: 10.0,
        }
    }
}

fn main() {
    App::build()
        // resources
        .init_resource::<State>()
        .init_resource::<Options>()
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
            transform: Transform::from_translation(Vec3::new(50.0, 0.0, 0.0)),
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
        // player
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
        .with(Player::default())
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
    cursor_hidden: bool,
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

fn movement_axis(input: &Res<Input<KeyCode>>, plus: KeyCode, minus: KeyCode) -> f32 {
    let mut axis = 0.0;

    if input.pressed(plus) {
        axis += 1.0;
    }

    if input.pressed(minus) {
        axis -= 1.0;
    }

    axis
}

fn movement_offset(input: &Res<Input<KeyCode>>, options: &Res<Options>) -> (f32, f32, f32) {
    (
        movement_axis(input, options.key_right, options.key_left),
        movement_axis(input, options.key_backward, options.key_forward),
        movement_axis(input, options.key_up, options.key_down),
    )
}

fn forward_vec(rotation: &Quat) -> Vec3 {
    rotation.mul_vec3(Vec3::unit_z()).normalize()
}

fn forward_walk_vec(rotation: &Quat) -> Vec3 {
    let f = forward_vec(rotation);
    // flatten vector by removing y axis info
    Vec3::new(f.x, 0.0, f.z).normalize()
}

fn strafe_vec(rotation: &Quat) -> Vec3 {
    Quat::from_rotation_y(90.0f32.to_radians())
        .mul_vec3(forward_walk_vec(rotation))
        .normalize()
}

fn move_player(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    options: Res<Options>,
    // mut player_query: Query<(&Player, &mut Transform)>,
    mut camera_query: Query<(&Camera, &mut Transform)>,
) {
    let (axis_h, axis_v, axis_float) = movement_offset(&keyboard_input, &options);

    for (_, mut transform) in camera_query.iter_mut() {
        let rotation = transform.rotation;
        let accel = (strafe_vec(&rotation) * axis_h)
            + (forward_walk_vec(&rotation) * axis_v)
            + (Vec3::unit_y() * axis_float);
        let accel = accel * options.speed;
        transform.translation += accel * time.delta_seconds();
    }
}

fn mouse_capture_system(mut state: ResMut<State>, windows: Res<WinitWindows>) {
    if !state.cursor_hidden {
        if let Some(window) = windows.get_window(WindowId::primary()) {
            if window.set_cursor_grab(true).is_ok() {
                println!("Snatching users cursor!");
                window.set_cursor_visible(false);
                state.cursor_hidden = true;
            }
        }
    }
}
