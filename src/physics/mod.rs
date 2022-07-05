use bevy::prelude::*;
use bevy_flycam::NoCameraPlayerPlugin;
use bevy_rapier3d::prelude::*;
use crate::AppState;

use self::{grenade::GrenadeData, sim_settings::SimSettings};

pub mod grenade;
pub mod sim_setup;
pub mod sim_settings;
pub mod fragment;

pub struct PhyiscsSimPlugin;

impl Plugin for PhyiscsSimPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(CursorLocked(true))
            .insert_resource(SimSettings {
                fuse_time: 5.0,
                lin_vel: Vec3::new(7.0, 2.0, 7.0),
                ang_vel: Vec3::new(1.0, 2.0, 1.0),
                friction: 10.0,
                restitution: 0.9,
                initial_height: 5.0,
                fragment_count: 10,
                explosion_vel: 480.0,
                csv_location: "data.csv".to_string(),
            })
            .add_state(GrenadeState::Grenade)
            .add_system_set(SystemSet::on_enter(AppState::LiveSim).with_system(sim_setup::update_sim_settings))
            .add_system_set(SystemSet::on_enter(AppState::LiveSim).with_system(lock_cursor))
            .add_system_set(SystemSet::on_enter(AppState::LiveSim).with_system(sim_setup::spawn_solid_surfaces))
            .add_system_set(SystemSet::on_enter(AppState::LiveSim).with_system(sim_setup::spawn_light))
            .add_system_set(SystemSet::on_enter(AppState::LiveSim).with_system(grenade::spawn_grenade))
            .add_system_set(SystemSet::on_update(AppState::LiveSim).with_system(grenade::explode_grenade))
            .add_system_set(SystemSet::on_update(AppState::LiveSim).with_system(back_to_main_menu_controls))
            .add_system_set(SystemSet::on_exit(AppState::LiveSim).with_system(cleanup))
            // Handle code just for fragment part of sim
            .add_system_set(SystemSet::on_enter(GrenadeState::Fragment).with_system(fragment::generate_fragments))
            .add_system_set(SystemSet::on_update(GrenadeState::Fragment).with_system(fragment::write_fragment_data))
            .add_system_set(SystemSet::on_exit(GrenadeState::Fragment).with_system(fragment::clean_fragments))
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugin(NoCameraPlayerPlugin);
    }
}

// Manage which stage of simulation
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GrenadeState {
    Grenade,
    Fragment,
}

#[derive(Component)]
struct CursorLocked(bool);

fn lock_cursor(
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

fn back_to_main_menu_controls(mut keys: ResMut<Input<KeyCode>>, mut app_state: ResMut<State<AppState>>, mut grenade_state: ResMut<State<GrenadeState>>) {
    if *app_state.current() == AppState::LiveSim {
        if keys.just_pressed(KeyCode::Q) {
            app_state.set(AppState::MainMenu).unwrap();
            grenade_state.set(GrenadeState::Grenade);
            keys.reset(KeyCode::Q);
        }
    }
}

fn cleanup(mut commands: Commands, sim_data: Res<GrenadeData>) {
    if sim_data.grenade_spawned {
        commands.entity(sim_data.grenade).despawn_recursive();
    }
    commands.entity(sim_data.camera).despawn_recursive();
}
