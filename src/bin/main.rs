use siml_core::eden::EdenContext;
use std::sync::Arc;
use std::time::Duration;
use std::thread;
use siml_core::eden::star::OrbitalStar;
use siml_core::eden::star_thread;

fn main() {
    let mut eden = EdenContext::new((64, 16, 1), 6000.0); // world size, 600s per year

    let light_state = eden.light_state.clone();
    let star = Arc::new(OrbitalStar::default());
    let clock = eden.clock.clone(); // <-- fix: clone clock

    star_thread::start_star_thread(light_state, star, clock); // <-- no need to spawn another thread here!

    loop {
        // Clear the screen before drawing new frame
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

        eden.tick(); // one heartbeat

        thread::sleep(Duration::from_millis(100)); // slow down for visibility (10 FPS)
    }
}
