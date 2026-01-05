use bevy::prelude::*;

use crate::player::PlayerPlugin;

mod player;


fn main() {
    App::new()
    .insert_resource(ClearColor(Color::WHITE))
    .add_plugins(DefaultPlugins.set(AssetPlugin {
        file_path: "assets".into(),
        ..default()
        }))
    .add_plugins(PlayerPlugin)
    .add_systems(Startup, setup_camera)
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