use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    log::{Level, LogPlugin},
    prelude::*,
};

mod map;

use map::MapPlugin;

fn main() {
    std::env::set_var(
        "RUST_LOG",
        "info,wgpu_core=error,wgpu_hal=error,bevy=warn,grimmfog=info",
    );
    std::env::set_var("LOGE_FORMAT", "target");
    // loge::init_with_file("grimmfog.log");

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(LogPlugin {
                    filter: "info,wgpu_core=error,wgpu_hal=error,bevy=warn".into(),
                    level: Level::INFO,
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_startup_system(spawn_camera)
        .add_plugin(MapPlugin)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    let projection = OrthographicProjection {
        scale: 0.065,
        ..default()
    };

    let mut camera_bundle = Camera2dBundle {
        projection,
        ..Default::default()
    };

    camera_bundle.transform.translation.x = 50.0;
    camera_bundle.transform.translation.y = 30.0;

    camera_bundle.camera_2d.clear_color = ClearColorConfig::Custom(Color::hex("1a1a1a").unwrap());

    commands.spawn(camera_bundle);
}
