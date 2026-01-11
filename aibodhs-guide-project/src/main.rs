use bevy::{prelude::*,window::{Window, WindowPlugin, WindowResolution}};

use crate::player::PlayerPlugin;
use crate::map::generate::{map_pixel_dimensions, setup_generator};

use bevy_procedural_tilemaps::prelude::*;

mod player;
mod map;

fn main() {
    let map_size = map_pixel_dimensions();

    App::new()
    .insert_resource(ClearColor(Color::WHITE))
    .add_plugins(DefaultPlugins
        .set(AssetPlugin {
            file_path: "assets".into(),
            ..default()
        })
        .set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(map_size.x as u32, map_size.y as u32), 
                title: "Game title".to_string(), 
                ..default()}),
            ..default()
        })
        .set(ImagePlugin::default_nearest())
    )
    .add_plugins(ProcGenSimplePlugin::<Cartesian3D, Sprite>::default())
    .add_plugins(PlayerPlugin)
    .add_systems(Startup, (setup_camera, setup_generator))
    .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);

//    commands.spawn((
//        Text2d::new("@"),
//        TextFont {
//            font_size: 24.0,
//            font: default(),
//            ..default()
//        },
//        TextColor(Color::WHITE),
//        Transform::from_translation(Vec3::ZERO),
//        Player
//    ));
}