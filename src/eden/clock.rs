pub struct EdenClock {
    pub sim_time: f32,          // total seconds elapsed
    pub orbital_period: f32,    // seconds per year (60s, 3600s, etc.)
}

impl EdenClock {
    pub fn new(orbital_period: f32) -> Self {
        Self {
            sim_time: 0.0,
            orbital_period,
        }
    }

    pub fn tick(&mut self, delta_time: f32) {
        self.sim_time += delta_time;
    }

    pub fn day_of_year(&self) -> f32 {
        (self.sim_time / self.orbital_period) * 365.0
    }

    pub fn season(&self) -> &'static str {
        match self.day_of_year() {
            d if d < 80.0 => "Winter",
            d if d < 172.0 => "Spring",
            d if d < 264.0 => "Summer",
            d if d < 355.0 => "Fall",
            _ => "Winter",
        }
    }

    pub fn solar_time(&self, spin_angle: f32) -> f32 {
        let normalized_spin = (spin_angle / std::f32::consts::TAU) % 1.0;
        let mut solar_hour = normalized_spin * 24.0;
        solar_hour = (solar_hour + 6.0) % 24.0; // Shift back by 6h (or +18 mod 24)
        solar_hour
    }
    
    
    
}
