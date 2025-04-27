use glam::Vec3;

// Rotates a Vec3 around Y axis (Earth-style spin)
pub fn rotate_vector_y(vec: Vec3, angle_rad: f32) -> Vec3 {
    let cos_a = angle_rad.cos();
    let sin_a = angle_rad.sin();
    Vec3::new(
        vec.x * cos_a - vec.z * sin_a,
        vec.y,
        vec.x * sin_a + vec.z * cos_a,
    )
}

pub fn compute_local_light(spin_angle: f32) -> Vec3 {
    let elevation = spin_angle.sin();

    Vec3::new(
        spin_angle.cos(),
        elevation,
        0.0,
    ).normalize()
}
