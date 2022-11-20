use std::io::Cursor;
use winit::window::Icon;

use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
// use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
// use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::window::WindowId;
use bevy::winit::WinitWindows;

pub const ASPECT_RATIO: f32 = 8. / 6.;
pub const WIDTH: f32 = 90.;
pub const HEIGHT: f32 = WIDTH * ASPECT_RATIO;

pub struct ConfigPlugin;

#[derive(Component)]
pub struct CameraFlag;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Msaa { samples: 1 })
            .insert_resource(bevy::render::texture::ImageSettings::default_nearest())
            .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
            .insert_resource(WindowDescriptor {
                width: 600.,
                height: 800.,
                title: "beverage".to_string(),
                canvas: Some("#bevy".to_owned()),
                fit_canvas_to_parent: true,
                ..Default::default()
            })
            .add_startup_system(camera_setup)
            .add_startup_system(window_icon_setup)
            .add_system(cursor_grab_system);

        #[cfg(debug_assertions)]
        {
            // app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            //     .add_plugin(LogDiagnosticsPlugin::default());
        }
    }
}

fn camera_setup(mut commands: Commands) {
    commands
        .spawn_bundle(Camera2dBundle {
            projection: OrthographicProjection {
                scaling_mode: ScalingMode::FixedHorizontal(WIDTH),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(CameraFlag);
}

fn window_icon_setup(windows: NonSend<WinitWindows>) {
    let primary = windows.get_window(WindowId::primary()).unwrap();
    let icon_buf = Cursor::new(include_bytes!("../assets/textures/app_icon.png"));
    if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        let icon = Icon::from_rgba(rgba, width, height).unwrap();
        primary.set_window_icon(Some(icon));
    };
}

fn cursor_grab_system(
    mut windows: ResMut<Windows>,
    btn: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
) {
    let window = windows.get_primary_mut().unwrap();

    if btn.just_pressed(MouseButton::Left) {
        window.set_cursor_lock_mode(true);
        window.set_cursor_visibility(false);
    }

    if key.just_pressed(KeyCode::Escape) {
        window.set_cursor_lock_mode(false);
        window.set_cursor_visibility(true);
    }
}

pub fn get_world_position(
    raw_position: Vec2,
    window: &Window,
    camera_transform: &GlobalTransform,
) -> Vec3 {
    let adjusted_position = Vec3::new(
        raw_position.x / window.width() * WIDTH - WIDTH / 2.,
        raw_position.y / window.height() * HEIGHT - HEIGHT / 2.,
        0.,
    );

    *camera_transform * adjusted_position
}
