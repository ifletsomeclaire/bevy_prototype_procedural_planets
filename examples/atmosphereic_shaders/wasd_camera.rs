// use super::*;
use bevy::{
    input::mouse::MouseMotion,
    input::system::exit_on_esc_system,
    math::{vec3, Quat, Vec3},
    prelude::*,
    render::camera::{PerspectiveProjection, Camera},
    window::Window,
    winit::WinitWindows,
};
pub struct WasdCamera;

pub struct CameraConfig {
    pub pan: f32,
    pub margin: f32,
    pub rotation: f32,
    pub w: f32,
    pub a: f32,
    pub s: f32,
    pub d: f32,
    pub z: f32,
    pub x: f32,
    pub q: f32,
    pub e: f32,
    pub camera_start: Translation,
}
impl Default for CameraConfig {
    fn default() -> Self {
        Self {
            pan: 100.,
            margin: 0.55,
            rotation: 8000.,
            w: 10.,
            a: 10.,
            s: 10.,
            d: 10.,
            z: 5.1,
            x: 5.1,
            q: 0.01,
            e: 0.01,
            camera_start: Translation::from(vec3(0.0, 0.0, 100000.)),
        }
    }
}

impl Plugin for WasdCamera {
    fn build(&self, app: &mut AppBuilder) {
        app
            // .init_resource::<MouseListener>()
            .init_resource::<CursorListener>()
            .init_resource::<Momentum>()
            .add_startup_system(Self::setup.system())
            .add_system(Self::camera.system())
            .add_system(Self::cursor.system())
            .add_system(Self::wasdcamera.system())
            .add_system(Self::momentum.system())
            .add_system(exit_on_esc_system.system());
    }
}

// #[derive(Default)]
// pub struct MouseListener {
//     cursor_event: EventReader<MouseMotion>,
// }
#[derive(Default)]
pub struct CursorListener {
    cursor_event: EventReader<CursorMoved>,
    pos: Vec2,
}
#[derive(Default)]
struct Momentum {
    moment: Vec3,
}

pub struct CameraMarker;

impl WasdCamera {
    fn setup(mut commands: Commands, config: Res<CameraConfig>) {
        commands
            .spawn(Camera3dComponents {
                translation: config.camera_start,
                perspective_projection: PerspectiveProjection {
                    far: 200000.0,
                    ..Default::default()
                },
                ..Default::default()
            })
            .with(CameraMarker);
    }
    fn momentum(
        config: Res<CameraConfig>,
        mut moment: ResMut<Momentum>,
        mut query: Query<(&CameraMarker, &Camera, &mut Translation, &mut Rotation)>,
    ) {
        for (_, _, mut trans, _) in &mut query.iter() {
            trans.0 += moment.moment
        }
    }

    fn wasdcamera(
        config: Res<CameraConfig>,
        mut moment: ResMut<Momentum>,
        time: Res<Time>,
        keys: Res<Input<KeyCode>>,
        mut query: Query<(&CameraMarker, &Camera, &mut Translation, &mut Rotation)>,
    ) {
        for (_, _, mut translation, mut rot) in &mut query.iter() {
            if keys.pressed(KeyCode::W) {
                translation.0 += rot.mul_vec3(vec3(0.0, time.delta_seconds * config.w, 0.0));
            }
            if keys.pressed(KeyCode::A) {
                translation.0 -= rot.mul_vec3(vec3(time.delta_seconds * config.a, 0.0, 0.0));
            }
            if keys.pressed(KeyCode::S) {
                translation.0 -= rot.mul_vec3(vec3(0.0, time.delta_seconds * config.s, 0.0));
            }
            if keys.pressed(KeyCode::D) {
                translation.0 += rot.mul_vec3(vec3(time.delta_seconds * config.d, 0.0, 0.0));
            }
            if keys.pressed(KeyCode::Z) {
                moment.moment -= rot.mul_vec3(vec3(0.0, 0.0, time.delta_seconds * config.z));

                // translation.0 -= rot.mul_vec3(vec3(0.0, 0.0, time.delta_seconds * 10.0));
            }
            if keys.pressed(KeyCode::X) {
                moment.moment += rot.mul_vec3(vec3(0.0, 0.0, time.delta_seconds * config.x));

                // translation.0 += rot.mul_vec3(vec3(0.0, 0.0, time.delta_seconds * 10.0));
            }
            if keys.pressed(KeyCode::Q) {
                rot.0 = rot
                    .normalize()
                    .mul_quat(Quat::from_rotation_z(config.q).normalize())
                    .normalize();
            }
            if keys.pressed(KeyCode::E) {
                rot.0 = rot
                    .normalize()
                    .mul_quat(Quat::from_rotation_z(-config.e).normalize())
                    .normalize();
            }
        }
    }
    fn cursor(
        config: Res<CameraConfig>,
        windows: Res<Windows>,
        mut cursor: ResMut<CursorListener>,
        cursor_moved_events: Res<Events<CursorMoved>>,
    ) {
        if let Some(event) = cursor.cursor_event.latest(&cursor_moved_events) {
            if let Some(window) = windows.get(event.id) {
                cursor.pos.set_x(event.position.x() / (window.width as f32));
                cursor
                    .pos
                    .set_y(event.position.y() / (window.height as f32));
            }
        }
    }

    fn camera(
        config: Res<CameraConfig>,

        time: Res<Time>,
        state: Res<CursorListener>,
        mut query: Query<(&CameraMarker, &Camera, &mut Translation, &mut Rotation)>,
    ) {
        let mut x_pan = 0.0;
        let mut y_pan = 0.0;
        let pos = (state.pos.x(), state.pos.y());

        let neg_marg = 1.0 - config.margin;
        match pos {
            (x, y) if x > config.margin && y > config.margin => {
                x_pan -= config.pan * (x - config.margin);
                y_pan -= config.pan * (y - config.margin);
            }
            (x, y) if x < neg_marg && y < neg_marg => {
                x_pan -= config.pan * (x - neg_marg);
                y_pan -= config.pan * (y - neg_marg);
            }
            (x, y) if x > config.margin && y < neg_marg => {
                x_pan -= config.pan * (x - config.margin);
                y_pan -= config.pan * (y - neg_marg);
            }
            (x, y) if x < neg_marg && y > config.margin => {
                x_pan -= config.pan * (x - neg_marg);
                y_pan -= config.pan * (y - config.margin);
            }
            (x, _) if x > config.margin => x_pan -= config.pan * (x - config.margin),
            (x, _) if x < neg_marg => x_pan -= config.pan * (x - neg_marg),
            (_, y) if y > config.margin => y_pan -= config.pan * (y - config.margin),
            (_, y) if y < neg_marg => y_pan -= config.pan * (y - neg_marg),
            _ => return,
        }
        for (_, _, mut translation, mut rot) in &mut query.iter() {
            // *translation.x_mut() -= x_pan * time.delta_seconds;
            // *translation.y_mut() -= y_pan * time.delta_seconds;
            rot.0 = rot
                .normalize()
                .mul_quat(Quat::from_rotation_y(x_pan / config.rotation).normalize())
                .normalize();
            rot.0 = rot
                .normalize()
                .mul_quat(Quat::from_rotation_x(-y_pan / config.rotation).normalize())
                .normalize();
        }
    }
    // fn cursor(
    //     mut winit_windows: ResMut<WinitWindows>,
    //     bevy_win: Res<Windows>,
    //     mut cursor: ResMut<MouseListener>,
    //     cursor_moved_events: Res<Events<MouseMotion>>,
    //     keys: Res<Input<KeyCode>>,
    //     mut query: Query<(&Camera, &mut Translation, &mut Rotation)>,
    // ) {
    //     if let Some(event) = cursor.cursor_event.latest(&cursor_moved_events) {
    //         //so this is a TERRIBLE solution
    //         for (_, mut translation, mut rot) in &mut query.iter() {
    //             if keys.pressed(KeyCode::LShift) {
    //                 for (_, window) in winit_windows.windows.iter() {
    //                     if let Some(w) =
    //                         bevy_win.get(winit_windows.get_window_id(window.id()).unwrap())
    //                     {
    //                         let mut pos = window.inner_position().unwrap();
    //                         pos.x = w.width as i32 / 2;
    //                         pos.y = w.height as i32 / 2;
    //                         window.set_cursor_position(pos);
    //                         window.set_cursor_visible(false);
    //                     }
    //                 }
    //                 rot.0 = rot
    //                     .normalize()
    //                     .mul_quat(Quat::from_rotation_y(-event.delta.x() / 80.).normalize())
    //                     .normalize();
    //                 rot.0 = rot
    //                     .normalize()
    //                     .mul_quat(Quat::from_rotation_x(-event.delta.y() / 80.).normalize())
    //                     .normalize();
    //             }
    //         }
    //     }
    // }
}
