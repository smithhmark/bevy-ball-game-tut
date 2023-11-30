use bevy::{prelude::*, window::PrimaryWindow};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        //.add_systems(Startup, setup)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_player)
        .run();
}

#[derive(Component)]
struct Player {}
/*
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: asset_server.load("sprites/ball_blue_large.png"),
        ..default()
    });
}
*/

fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    println!("spawning player");
    println!(
        "{}, {}, {}",
        window.width() / 2.0,
        window.height() / 2.0,
        0.0
    );
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/ball_blue_large.png"),
            ..default()
        },
        Player {},
    ));
}

fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    println!("spawning camera");
    //commands.spawn(Camera2dBundle::default());
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
    /*
     */
}
