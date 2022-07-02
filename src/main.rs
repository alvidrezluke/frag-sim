use bevy::prelude::*;

mod physics;
mod config_menu;
fn main() {
    App::new()
        .add_state(AppState::MainMenu)
        .add_plugins(DefaultPlugins)
        .add_plugin(config_menu::config_menu::MainMenuPlugin)
        .add_plugin(physics::PhyiscsSimPlugin)
        .run();
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    LiveSim,
}