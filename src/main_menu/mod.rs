use bevy::prelude::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, main_menu_placeholder);
    }
}

fn main_menu_placeholder() {
    println!("a placeholder");
}
