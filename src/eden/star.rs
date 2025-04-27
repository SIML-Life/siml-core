use glam::{Mat3, Vec3};

pub struct OrbitalStar {
    pub semi_major_axis: f32,  // average distance (AU or normalized units)
    pub eccentricity: f32,     // orbital eccentricity
    pub orbital_period: f32,   // seconds for full orbit
    pub axial_tilt_deg: f32,   // obliquity angle in degrees (Earth: 23.4)
    pub precession_rate: f32,  // radians per second (slow wobble)
    pub luminosity: f32,       // base brightness
}

impl Default for OrbitalStar {
    fn default() -> Self {
        Self {
            semi_major_axis: 1.0, // normalized distance
            eccentricity: 0.017, // Earth's orbital eccentricity
            orbital_period: 60.0, // let's say 60s for 1 year for now
            axial_tilt_deg: 23.4,
            precession_rate: 2.0 * std::f32::consts::PI / (26_000.0 * 60.0), // ~26,000 "years" for full precession
            luminosity: 1.0,
        }
    }
}

pub fn rotate_with_tilt_and_precession(pos: Vec3, tilt: f32, precession: f32) -> Vec3 {
    // First, rotate around Z axis for precession
    let precession_rot = Mat3::from_axis_angle(Vec3::Y, precession);
    let precessed = precession_rot * pos;

    // Then, tilt around X axis for obliquity
    let tilt_rot = Mat3::from_axis_angle(Vec3::X, tilt);
    let tilted = tilt_rot * precessed;

    tilted
}

impl OrbitalStar {
    pub fn compute_light_vector(&self, sim_time: f32) -> (Vec3, f32) {
        let angle = (sim_time / self.orbital_period) * std::f32::consts::TAU;
    
        let r = self.semi_major_axis * (1.0 - self.eccentricity * self.eccentricity)
            / (1.0 + self.eccentricity * angle.cos());
    
        let orbital_pos = Vec3::new(angle.cos(), 0.0, angle.sin()) * r;
    
        let obliquity = self.axial_tilt_deg.to_radians();
        let precession = sim_time * self.precession_rate;
    
        let tilted = rotate_with_tilt_and_precession(orbital_pos, obliquity, precession);
    
        (tilted.normalize(), r)
    }
    
}

