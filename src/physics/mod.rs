use bevy::prelude::*;
use bevy_flycam::NoCameraPlayerPlugin;
use bevy_rapier3d::prelude::*;
use crate::AppState;

use self::{grenade::GrenadeData, sim_settings::SimSettings, sim_setup::Wall};

pub mod grenade;
pub mod sim_setup;
pub mod sim_settings;
pub mod fragment;

// Create a plugin
pub struct PhyiscsSimPlugin;

impl Plugin for PhyiscsSimPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add variable to keep track if cursor is locked
            .insert_resource(CursorLocked(true))

            // Add default SimSettings
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

            // Initialize to starting simulation as a grenade
            .add_state(GrenadeState::Grenade)

            // When simulation is started try to fetch new settings from config file
            .add_system_set(SystemSet::on_enter(AppState::LiveSim).with_system(sim_setup::update_sim_settings))

            // When simulation is started start cursor locking logic
            .add_system_set(SystemSet::on_enter(AppState::LiveSim).with_system(lock_cursor))

            // When simulation is started spawn the walls and floor
            .add_system_set(SystemSet::on_enter(AppState::LiveSim).with_system(sim_setup::spawn_solid_surfaces))

            // When simulation is started spawn the ambient light
            .add_system_set(SystemSet::on_enter(AppState::LiveSim).with_system(sim_setup::spawn_light))

            // When simulation is started spawn a grenade with the settings from SimSettings
            .add_system_set(SystemSet::on_enter(AppState::LiveSim).with_system(grenade::spawn_grenade))

            // While simulation is running check to see if it is time to explode grenade
            .add_system_set(SystemSet::on_update(AppState::LiveSim).with_system(grenade::explode_grenade))

            // While simulation is running check to see if "Q" is pressed to return to main menu
            .add_system_set(SystemSet::on_update(AppState::LiveSim).with_system(back_to_main_menu_controls))

            // When simulation is ending despawn grenade and camera
            .add_system_set(SystemSet::on_exit(AppState::LiveSim).with_system(cleanup))

            // When entering the fragment part of the simulation generate all fragmentss 
            .add_system_set(SystemSet::on_enter(GrenadeState::Fragment).with_system(fragment::generate_fragments))

            // While in the fragment part of simulation, export path of frag
            .add_system_set(SystemSet::on_update(GrenadeState::Fragment).with_system(fragment::write_fragment_data))

            // When fragmentation simulation is completed clean the fragments
            .add_system_set(SystemSet::on_exit(GrenadeState::Fragment).with_system(fragment::clean_fragments))

            // Add physics engine
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())

            // Add camera handling
            .add_plugin(NoCameraPlayerPlugin);
    }
}

// Manage simulation stages
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GrenadeState {
    Grenade,
    Fragment,
}

// Manage if cursor is locked
#[derive(Component)]
struct CursorLocked(bool);

// Handle cursor locking
fn lock_cursor(
    mut windows: ResMut<Windows>,
    btn: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
) {
    // Get the simulation window
    let window = windows.get_primary_mut().unwrap();

    // If window is entered lock cursor
    if btn.just_pressed(MouseButton::Left) {
        window.set_cursor_lock_mode(true);
        window.set_cursor_visibility(false);
    }

    // Unlock cursor when Escape pressed
    if key.just_pressed(KeyCode::Escape) {
        window.set_cursor_lock_mode(false);
        window.set_cursor_visibility(true);
    }
}

// Check for "Q" press to return to main menu
fn back_to_main_menu_controls(
    mut keys: ResMut<Input<KeyCode>>,
    mut app_state: ResMut<State<AppState>>,
    mut grenade_state: ResMut<State<GrenadeState>>
 ) {
    // If in simulation
    if *app_state.current() == AppState::LiveSim {
        
        // If "Q" pressed
        if keys.just_pressed(KeyCode::Q) {
            
            // Set state back to main menu
            app_state.set(AppState::MainMenu).unwrap();

            // Reset simulation to grenade
            let _grenade_res = grenade_state.set(GrenadeState::Grenade);

            // Reset the "Q" stroke instantly
            keys.reset(KeyCode::Q);
        }
    }
}

// Remove grenade and camers
fn cleanup(mut commands: Commands, sim_data: Res<GrenadeData>, walls: Query<Entity, With<Wall>>) {

    // If grenade exists then despawn it
    if sim_data.grenade_spawned {
        commands.entity(sim_data.grenade).despawn_recursive();
    }

    for wall in walls.iter() {
        commands.entity(wall).despawn_recursive();
    }

    // Despawn camera
    commands.entity(sim_data.camera).despawn_recursive();
}
