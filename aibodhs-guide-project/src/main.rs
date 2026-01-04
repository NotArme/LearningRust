use bevy::prelude::*;

mod player;


fn main() {
    App::new()
    .insert_resource(ClearColor(Color::WHITE))
    .add_plugins(DefaultPlugins.set(AssetPlugin {
        file_path: "assets".into(),
        ..default()
        }))
    .add_systems(Startup, setup_camera)
    .add_systems(Update, move_player)
    .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Text2d::new("@"),
        TextFont {
            font_size: 24.0,
            font: default(),
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_translation(Vec3::ZERO),
        PLayer
    ));
}

fn move_player(input: Res<ButtonInput<KeyCode>>, time: Res<Time>, mut player_transform: Single<&mut Transform, With<PLayer>>) {
    let mut direction = Vec2::ZERO;
    if input.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }
    if input.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }
    if input.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }
    if input.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }

    if direction != Vec2::ZERO {
        let speed = 300.0;
        let velocity = direction.normalize() * speed * time.delta_secs();
        player_transform.translation.x += velocity.x;
        player_transform.translation.y += velocity.y;
    }
}