// #![allow(unused)]
use std::error::Error;
use bevy::{prelude::*, render::camera::{WindowOrigin, ScalingMode}};

mod billards;
use billards::BillardsPlugin;


// Window Constants
const ASPECT_RATIO: f32 = 16.0 / 9.0;
const WIDTH: f32 = 1280.0;
const HEIGHT: f32 = WIDTH / ASPECT_RATIO; // 1280x720
// Simulation Constants
const SIM_MIN_WIDTH: f32 = 2.0;
const SIM_SCALE: f32 = HEIGHT / SIM_MIN_WIDTH;
const SIM_WIDTH: f32 = WIDTH / SIM_SCALE;
const SIM_HEIGHT: f32 = HEIGHT / SIM_SCALE;

const _DT: f32 = 1.0 / 60.0;


pub fn simulate() -> Result<(), Box<dyn Error>> {
    App::new()
        .add_startup_system(setup)
        .add_startup_system(spawn_camera)
        .add_plugins(DefaultPlugins)
        .add_plugin(BillardsPlugin)
        .run();
    Ok(())
}


fn setup(
    mut commands: Commands,
) {
    commands.insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)));
    commands.insert_resource(WindowDescriptor {
        title: "Billard Balls".to_string(),
        width: WIDTH,
        height: HEIGHT,
        vsync: true,
        resizable: false,
        ..Default::default()
    });
}

fn spawn_camera(
    mut commands: Commands,
) {
    let mut camera = OrthographicCameraBundle::new_2d();
    
    camera.orthographic_projection.window_origin = WindowOrigin::BottomLeft;
    camera.orthographic_projection.top = SIM_HEIGHT;
    camera.orthographic_projection.bottom = 0.0;
    camera.orthographic_projection.left = 0.0;
    camera.orthographic_projection.right = SIM_WIDTH;
    camera.orthographic_projection.scaling_mode = ScalingMode::None;

    commands.spawn_bundle(camera);
}
