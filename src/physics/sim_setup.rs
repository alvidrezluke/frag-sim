use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::fs;

use super::sim_settings::SimSettings;

pub fn spawn_solid_surfaces(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    // Spawn floor
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box {min_x: -50.0, max_x: 50.0, min_y: 0.0, max_y: 10.0, min_z: -50.0, max_z: 50.0 })),
            material: materials.add(Color::rgb(0.9, 0.9, 0.9).into()),
            ..default()
        })
        .insert(Collider::cuboid(100.0, 10.0, 100.0))
        .insert(Friction {
            coefficient: 100000000.0,
            combine_rule: CoefficientCombineRule::Multiply
        })
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, -10.0, 0.0)));
    
    // Spawn wall 1
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box { min_x: -50.0, max_x: 50.0, min_y: 0.0, max_y: 75.0, min_z: 0.0, max_z: 1.0})),
            material: materials.add(Color::rgb(0.9, 0.9, 0.9).into()),
            ..default()
        })
        .insert(Collider::cuboid(100.0, 75.0, 1.0))
        .insert(Friction {
            coefficient: 100000000.0,
            combine_rule: CoefficientCombineRule::Multiply
        })
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 50.0)));
    
    // Spawn wall 2
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box { min_x: -50.0, max_x: 50.0, min_y: 0.0, max_y: 75.0, min_z: 0.0, max_z: 1.0})),
            material: materials.add(Color::rgb(0.9, 0.9, 0.9).into()),
            ..default()
        })
        .insert(Collider::cuboid(100.0, 75.0, 1.0))
        .insert(Friction {
            coefficient: 100000000.0,
            combine_rule: CoefficientCombineRule::Multiply
        })
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 0.0, -50.0)));

    // Spawn wall 3
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box { min_x: 0.0, max_x: 1.0, min_y: 0.0, max_y: 75.0, min_z: -50.0, max_z: 50.0})),
            material: materials.add(Color::rgb(0.9, 0.9, 0.9).into()),
            ..default()
        })
        .insert(Collider::cuboid(100.0, 75.0, 1.0))
        .insert(Friction {
            coefficient: 100000000.0,
            combine_rule: CoefficientCombineRule::Multiply
        })
        .insert_bundle(TransformBundle::from(Transform::from_xyz(50.0, 0.0, 0.0)));

    // Spawn wall 4
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box { min_x: 0.0, max_x: 1.0, min_y: 0.0, max_y: 75.0, min_z: -50.0, max_z: 50.0})),
            material: materials.add(Color::rgb(0.9, 0.9, 0.9).into()),
            ..default()
        })
        .insert(Collider::cuboid(100.0, 75.0, 1.0))
        .insert(Friction {
            coefficient: 100000000.0,
            combine_rule: CoefficientCombineRule::Multiply
        })
        .insert_bundle(TransformBundle::from(Transform::from_xyz(-50.0, 0.0, 0.0)));
}


// Add light
pub fn spawn_light(
    mut commands: Commands
) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1.0
    })
}

// Check for file and read it in if present
pub fn update_sim_settings(mut commands: Commands) {
    let args: Vec<String> = std::env::args().collect();
    let mut path = "config.txt".to_string();
    if args.len() >= 3 {
        if args[1] == "-c" {
            path = args[2].clone();
        } else {
            println!("Only valid arg is -c and then the filepath to the config file.")
        }
    }

    let res = fs::read_to_string(path);
    if res.is_err() {
        println!("Error");
        let lin_vel: Vec3 = Vec3::new(7.0, 2.0, 7.0);
        let ang_vel: Vec3 = Vec3::new(1.0, 2.0, 1.0);
        commands.insert_resource(SimSettings {
            fuse_time: 5.0,
            lin_vel,
            ang_vel,
            friction: 10.0,
            restitution: 0.9,
            initial_height: 5.0,
            fragment_count: 10,
            explosion_vel: 480.0,
            csv_location: "data.csv".to_string(),
        });
    } else {
        let mut fuse_time: f32 = 5.0;
        let mut friction: f32 = 10.0;
        let mut restitution: f32 = 0.9;
        let mut initial_height: f32 = 5.0;
        let mut fragment_count: u32 = 100;
        let mut explosion_vel: f32 = 480.0;
        let mut csv_location: String = "data.csv".to_string();
        let mut lin_vel: Vec3 = Vec3::new(7.0, 2.0, 7.0);
        let mut ang_vel: Vec3 = Vec3::new(1.0, 2.0, 1.0);
        let valid_file = res.expect("Could not open.");
        let lines = valid_file.lines();
        if lines.clone().count() == 0 {
            commands.insert_resource(SimSettings {
                fuse_time,
                lin_vel,
                ang_vel,
                friction,
                restitution,
                initial_height,
                fragment_count,
                explosion_vel,
                csv_location: csv_location.clone()
            });
        }
        for line in lines {
            let delimeter = ":";
            let config_vals = line.split(delimeter).collect::<Vec<&str>>();
            if config_vals.len() >= 2 {
                match config_vals[0].to_lowercase().as_str() {
                    "fuse_time" => {
                        let parsed: Result<f32, _> = config_vals[1].trim().parse();
                        if parsed.is_ok() {
                            fuse_time = parsed.expect("Not a f32");
                        }
                    },
                    "friction" => {
                        let parsed: Result<f32, _> = config_vals[1].trim().parse();
                        if parsed.is_ok() {
                            friction = parsed.expect("Not a f32");
                        }
                    },
                    "restitution" => {
                        let parsed: Result<f32, _> = config_vals[1].trim().parse();
                        if parsed.is_ok() {
                            restitution = parsed.expect("Not a f32");
                        }
                    },
                    "initial_height" => {
                        let parsed: Result<f32, _> = config_vals[1].trim().parse();
                        if parsed.is_ok() {
                            initial_height = parsed.expect("Not a f32");
                        }
                    },
                    "fragment_count" => {
                        let parsed: Result<u32, _> = config_vals[1].trim().parse();
                        if parsed.is_ok() {
                            fragment_count = parsed.expect("Not a u32");
                        }
                    },
                    "explosion_vel" => {
                        let parsed: Result<f32, _> = config_vals[1].trim().parse();
                        if parsed.is_ok() {
                            explosion_vel = parsed.expect("Not a f32");
                        }
                    },
                    "csv_location" => {
                        csv_location = config_vals[1].trim().to_string();
                    },
                    "lin_vel" => {
                        let cleaned_string = config_vals[1].trim().replace("<", "").replace(">", "");
                        let cleaned_arr = cleaned_string.split(",");
                        let vec: Vec<&str> = cleaned_arr.collect();
                        let mut x: f32 = 7.0;
                        let mut y: f32 = 2.0;
                        let mut z: f32 = 7.0;
                        if vec.len() == 3 {
                            let parsed_x: Result<f32, _> = vec[0].trim().parse();
                            if parsed_x.is_ok() {
                                x = parsed_x.expect("Not a f32");
                            }
                            let parsed_y: Result<f32, _> = vec[1].trim().parse();
                            if parsed_y.is_ok() {
                                y = parsed_y.expect("Not a f32");
                            }
                            let parsed_z: Result<f32, _> = vec[2].trim().parse();
                            if parsed_z.is_ok() {
                                z = parsed_z.expect("Not a f32");
                            }
                            lin_vel = Vec3::new(x, y, z);
                        } else {
                            println!("Invalid vector for lin_vel.")
                        }
                    },
                    "ang_vel" => {
                        let cleaned_string = config_vals[1].trim().replace("<", "").replace(">", "");
                        let cleaned_arr = cleaned_string.split(",");
                        let vec: Vec<&str> = cleaned_arr.collect();
                        let mut x: f32 = 7.0;
                        let mut y: f32 = 2.0;
                        let mut z: f32 = 7.0;
                        if vec.len() == 3 {
                            let parsed_x: Result<f32, _> = vec[0].trim().parse();
                            if parsed_x.is_ok() {
                                x = parsed_x.expect("Not a f32");
                            }
                            let parsed_y: Result<f32, _> = vec[1].trim().parse();
                            if parsed_y.is_ok() {
                                y = parsed_y.expect("Not a f32");
                            }
                            let parsed_z: Result<f32, _> = vec[2].trim().parse();
                            if parsed_z.is_ok() {
                                z = parsed_z.expect("Not a f32");
                            }
                            ang_vel = Vec3::new(x, y, z);
                        } else {
                            println!("Invalid vector for ang_vel.")
                        }
                    },
                    _ => {}
                }
                commands.insert_resource(SimSettings {
                    fuse_time,
                    lin_vel,
                    ang_vel,
                    friction,
                    restitution,
                    initial_height,
                    fragment_count,
                    explosion_vel,
                    csv_location: csv_location.clone()
                });
            } else {
                commands.insert_resource(SimSettings {
                    fuse_time,
                    lin_vel,
                    ang_vel,
                    friction,
                    restitution,
                    initial_height,
                    fragment_count,
                    explosion_vel,
                    csv_location: csv_location.clone()
                });
            }
        }
    }
}