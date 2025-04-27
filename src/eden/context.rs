use crate::eden::{
    clock::EdenClock, field::FieldSystem, grid::World, light_state::LightState,
    microbiome::MicroBiomeSystem, utils::compute_local_light,
};
use std::sync::{Arc, Mutex};
use std::time::Instant;

/// Central context for all Eden simulation state
pub struct EdenContext {
    pub world: World,
    pub field: FieldSystem,
    pub microbiome: MicroBiomeSystem,
    pub light_state: Arc<Mutex<LightState>>,
    pub clock: Arc<Mutex<EdenClock>>,
    pub last_tick: Instant,
    pub last_avg_light: f32,
    pub day_counter: u32,
}

impl EdenContext {
    pub fn new(world_size: (usize, usize, usize), orbital_period: f32) -> Self {
        Self {
            world: World::new(world_size),
            field: FieldSystem::new(),
            microbiome: MicroBiomeSystem::new(),
            light_state: Arc::new(Mutex::new(LightState::default())),
            clock: Arc::new(Mutex::new(EdenClock::new(orbital_period))),
            last_tick: Instant::now(),
            last_avg_light: 0.0,
            day_counter: 0,
        }
    }

    pub fn tick(&mut self) {
        let now = Instant::now();
        let delta = now.duration_since(self.last_tick);
        self.last_tick = now;
        let delta_secs = delta.as_secs_f32();

        // 1. Advance clock
        {
            let mut clock = self.clock.lock().unwrap();
            clock.tick(delta_secs);
        }

        // 2. Advance local spin (Earth-style)
        let spin_angle = {
            let mut light = self.light_state.lock().unwrap();
            const SIMULATED_DAY_LENGTH_SECS: f32 = 10.0; // One full day = 10 seconds
            let spin_radians_per_second = std::f32::consts::TAU / SIMULATED_DAY_LENGTH_SECS;

            light.local_spin_angle += delta_secs * spin_radians_per_second;
            light.local_spin_angle %= std::f32::consts::TAU;

            light.local_spin_angle
        };

        // 3. Compute local light vector based on spin (new way!)
        let rotated_light_vec = compute_local_light(spin_angle);

        // 4. Update world lighting
        {
            let light = self.light_state.lock().unwrap();
            self.world
                .update_lighting(rotated_light_vec, light.brightness);
        }

        // 5. Detect sunrise
        let avg_light = self.world.average_light();

        let light_was_dark = self.last_avg_light <= 0.01;
        let light_now_bright = avg_light > 0.01;

        if light_was_dark && light_now_bright {
            self.day_counter += 1;
            println!("🌞 Sunrise detected! → Total Days: {}", self.day_counter);
        }
        self.last_avg_light = avg_light;

        // 6. Advance biology
        self.microbiome.update(&mut self.world);
        self.field.update(&mut self.world);

        // 7. Display world
        {
            let clock = self.clock.lock().unwrap();
            // let day_of_year = clock.day_of_year();
            let season = clock.season();
            let solar_time = clock.solar_time(spin_angle);

            self.world.display_ascii();
            println!(
                "🌅 Current Day: {} | Season: {} | Solar Time: {:.1}h | ",
                self.day_counter, season, solar_time, 
            );
        }
    }
}
