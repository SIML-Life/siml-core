use std::sync::{Arc, Mutex};
use std::time::Duration;
use crate::eden::light_state::LightState;
use crate::eden::star::OrbitalStar;
use crate::eden::clock::EdenClock;

pub fn start_star_thread(
    light_state: Arc<Mutex<LightState>>,
    star: Arc<OrbitalStar>,
    clock: Arc<Mutex<EdenClock>>,
) {
    std::thread::spawn(move || {
        loop {
            // Read sim_time from the global clock
            let sim_time = {
                let clock = clock.lock().unwrap();
                clock.sim_time
            };

            // Compute new light vector based on orbital motion
            let (light_vec, distance) = star.as_ref().compute_light_vector(sim_time);

            {
                let mut light = light_state.lock().unwrap();
                light.light_vector = light_vec;
                light.star_position = light_vec * distance;
                light.brightness = star.luminosity / (distance * distance);
            }

            std::thread::sleep(Duration::from_millis(16)); // 60Hz refresh
        }
    });
}
