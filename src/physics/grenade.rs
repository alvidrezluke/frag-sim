use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_flycam::FlyCam;

use crate::AppState;

use super::{sim_settings::SimSettings, GrenadeState};

#[derive(Component)]
pub struct Grenade;

// Spawn grenade
pub fn spawn_grenade(
    mut commands:Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    sim_settings: Res<SimSettings>
) {
    let grenade = commands.spawn_bundle(PbrBundle {
        // Create a mesh of a capsule
        mesh: meshes.add(Mesh::from(shape::Capsule {radius: 0.2, depth: 0.2, ..default()})),

        // Set the color to red
        material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),

        // All other values as default
        ..default()
    })
    
    // Insert Grenade identifier
    .insert(Grenade)

    // Add component for physics engine
    .insert(RigidBody::Dynamic)

    // Add in initial velocities from SimSettings
    .insert(Velocity {
        linvel: sim_settings.lin_vel,
        angvel: sim_settings.ang_vel,
    })

    // Add a collider the same size as the mesh
    .insert(Collider::capsule(Vec3::new(0.0, -0.1, 0.0), Vec3::new(0.0, 0.1, 0.0), 0.2))

    // Set friction to SimSettings value
    .insert(Friction::coefficient(sim_settings.friction))

    // Set resitution to SimSettings value
    .insert(Restitution::coefficient(sim_settings.restitution))

    // Move to initial location
    .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, sim_settings.initial_height, 0.0)))

    // Return the id to be stored in the "grenade" variable
    .id();


    // Create a moveable camera
    let camera = PerspectiveCameraBundle {
        
        // Move camera to initial position and point at the center
        transform: Transform::from_xyz(0.0, 5.0, -5.0).looking_at(Vec3::ZERO, Vec3::Y),

        // Set everything else to default
        ..default()
    };

    // Add camera to world
    let camera_entity = commands.spawn_bundle(camera).insert(FlyCam).id();

    // Insert timer with duration from SimSettings
    commands.insert_resource(GrenadeTimer(Timer::from_seconds(sim_settings.fuse_time, false)));

    // Save grenade id, camera id, that a grenade has been spawned, and the initial velocity and position for later use 
    commands.insert_resource(GrenadeData {
        grenade,
        camera: camera_entity,
        grenade_spawned: true,
        last_location: Transform {..default()},
        last_vel: Velocity {..default()}
    });
}

// Data structure to keep track of grenade data
pub struct GrenadeData {
    pub grenade: Entity,
    pub camera: Entity,
    pub grenade_spawned: bool,
    pub last_location: Transform,
    pub last_vel: Velocity
}

// Timer to explode grenade
pub struct GrenadeTimer(Timer);

// System to handle grenade explosion
pub fn explode_grenade(
    grenade_cur: Query<(&Transform, &Velocity), With<Grenade>>,
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<GrenadeTimer>,
    mut query: Query<(Entity, With<Grenade>)>,
    app_state: ResMut<State<AppState>>,
    mut grenade_state: ResMut<State<GrenadeState>>,
    mut grenade_data: ResMut<GrenadeData>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>
) {
    // Ensure that still in simulation
    if *app_state.current() == AppState::LiveSim {
        
        // Check if timer is finished and grenade can be found
        if timer.0.tick(time.delta()).just_finished() && !query.is_empty(){

            // Extract grenade from world
            let (entity, _grenade) = query.single_mut();

            // If grenade is currently spawned
            if grenade_data.grenade_spawned {

                // Play explosion audio effect
                play_explosion(asset_server, audio);

                // Get the position and velocity of the grenade
                let (pos, vel) = grenade_cur.single();

                // Set last_location and last_vel from current pos and vel
                grenade_data.last_location = *pos;
                grenade_data.last_vel = *vel;

                // Despawn grenade
                commands.entity(entity).despawn();

                // Set grenade_spawned to false
                grenade_data.grenade_spawned = false;

                // Move simulation state to fragment
                grenade_state.set(GrenadeState::Fragment).expect("Could not set grenade state.");
            }
        }
    }
}

// Play explosion noise from ./assets/grenade.ogg
fn play_explosion(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio.play(asset_server.load("grenade.ogg"));
}