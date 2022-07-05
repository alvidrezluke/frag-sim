use bevy::prelude::*;
use bevy_flycam::NoCameraPlayerPlugin;
use bevy_rapier3d::prelude::*;
use crate::AppState;

use self::grenade::GrenadeData;

pub mod grenade;
pub mod floor;
pub mod sim_settings;

pub struct PhyiscsSimPlugin;

impl Plugin for PhyiscsSimPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(CursorLocked(true))
            .add_system_set(SystemSet::on_enter(AppState::LiveSim).with_system(lock_cursor))
            .add_system_set(SystemSet::on_enter(AppState::LiveSim).with_system(floor::spawn_floor))
            .add_system_set(SystemSet::on_enter(AppState::LiveSim).with_system(grenade::spawn_grenade))
            .add_system_set(SystemSet::on_update(AppState::LiveSim).with_system(grenade::explode_grenade))
            .add_system_set(SystemSet::on_exit(AppState::LiveSim).with_system(cleanup))
            .add_system_set(SystemSet::on_update(AppState::LiveSim).with_system(back_to_main_menu_controls))
            .add_startup_system(setup)
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugin(NoCameraPlayerPlugin);
    }
}

fn setup(mut commands: Commands) {
    let lin_vel: Vec3 = Vec3::new(7.0, 2.0, 7.0);
    let ang_vel: Vec3 = Vec3::new(1.0, 2.0, 1.0);
    commands.insert_resource(sim_settings::SimSettings {
        fuse_time: 5.0,
        density: 1.0,
        lin_vel,
        ang_vel,
        friction: 10.0,
        restitution: 0.9,
        initial_height: 5.0,
        fragment_count: 10.0
    });
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

fn back_to_main_menu_controls(mut keys: ResMut<Input<KeyCode>>, mut app_state: ResMut<State<AppState>>) {
    if *app_state.current() == AppState::LiveSim {
        if keys.just_pressed(KeyCode::Q) {
            app_state.set(AppState::MainMenu).unwrap();
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
