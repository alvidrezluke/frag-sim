use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_flycam::FlyCam;

use crate::AppState;

use super::{sim_settings::SimSettings, GrenadeState};

#[derive(Component)]
pub struct Grenade;

pub fn spawn_grenade(
    mut commands:Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    sim_settings: Res<SimSettings>
) {
    let grenade = commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Capsule {radius: 0.2, depth: 0.2, ..default()})),
        material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
        ..default()
    })
    .insert(Grenade)
    .insert(RigidBody::Dynamic)
    .insert(Velocity {
        linvel: sim_settings.lin_vel,
        angvel: sim_settings.ang_vel,
    })
    .insert(Collider::capsule(Vec3::new(0.0, -0.1, 0.0), Vec3::new(0.0, 0.1, 0.0), 0.2))
    .insert(Friction::coefficient(sim_settings.friction))
    .insert(Restitution::coefficient(sim_settings.restitution))
    .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, sim_settings.initial_height, 0.0))).id();

    let camera = PerspectiveCameraBundle {
        transform: Transform::from_xyz(-2.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    };

    // add plugin
    let camera_entity = commands.spawn_bundle(camera).insert(FlyCam).id();

    commands.insert_resource(GrenadeTimer(Timer::from_seconds(5.0, true)));

    commands.insert_resource(GrenadeData {
        grenade,
        camera: camera_entity,
        grenade_spawned: true,
        last_location: Transform {..default()},
        last_vel: Velocity {..default()}
    });
}

pub struct GrenadeData {
    pub grenade: Entity,
    pub camera: Entity,
    pub grenade_spawned: bool,
    pub last_location: Transform,
    pub last_vel: Velocity
}

pub struct GrenadeTimer(Timer);

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
    if *app_state.current() == AppState::LiveSim {
        if timer.0.tick(time.delta()).just_finished() && !query.is_empty(){
            let (entity, _grenade) = query.single_mut();
            if grenade_data.grenade_spawned {
                let (pos, vel) = grenade_cur.single();
                grenade_data.last_location = *pos;
                grenade_data.last_vel = *vel;
                commands.entity(entity).despawn();
                grenade_data.grenade_spawned = false;
                play_explosion(asset_server, audio);
                grenade_state.set(GrenadeState::Fragment).expect("Could not set grenade state.");
            }
        }
    }
}

fn play_explosion(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio.play(asset_server.load("grenade.ogg"));
}