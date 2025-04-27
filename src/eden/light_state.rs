use glam::Vec3;

#[derive(Default)]
pub struct LightState {
    pub light_vector: Vec3,
    pub star_position: Vec3,
    pub brightness: f32,
    pub local_spin_angle: f32
}
