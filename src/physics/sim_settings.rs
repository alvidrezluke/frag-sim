use bevy::math::Vec3;

pub struct SimSettings {
    pub fuse_time: f32,
    // pub density: f32,
    pub lin_vel: Vec3,
    pub ang_vel: Vec3,
    pub friction: f32,
    pub restitution: f32,
    pub initial_height: f32,
    pub fragment_count: u32,
    pub explosion_vel: f32,
    pub csv_location: String,
}