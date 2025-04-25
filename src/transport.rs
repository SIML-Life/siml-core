use crate::io::socket;
use crate::sensors::chemosense::Chemosense;
use crate::cell::Cell;
pub use crate::types::{Bitfield, Nutrient};

/// Worm position
pub fn send_agent_position(pos: (usize, usize, usize)) {
    let (x, y, z) = pos;
    socket::send_i32_triplet(x as i32, y as i32, z as i32);
}

/// Receive food collision
pub fn try_receive_food_collision() -> Option<(usize, usize, usize)> {
    if let Some((fx, fy, fz)) = socket::read_i32_triplet() {
        println!("[SIML] 🍎 Food Collision at ({}, {}, {})", fx, fy, fz);
        Some((fx as usize, fy as usize, fz as usize))
    } else {
        None
    }
}

/// Initial worm spawn
pub fn read_agent_position_from_godot() -> Option<(usize, usize, usize)> {
    socket::read_i32_triplet().map(|(x, y, z)| (x as usize, y as usize, z as usize))
}

/// Death marker
pub fn send_agent_death(position: (usize, usize, usize)) {
    let (x, y, z) = position;
    let marker_code: i32 = 9999;

    let mut buffer = vec![];
    buffer.extend_from_slice(&marker_code.to_le_bytes());
    buffer.extend_from_slice(&(x as i32).to_le_bytes());
    buffer.extend_from_slice(&(y as i32).to_le_bytes());
    buffer.extend_from_slice(&(z as i32).to_le_bytes());

    if let Err(e) = socket::send_bytes(&buffer) {
        eprintln!("❌ Failed to send death message: {}", e);
    } else {
        println!("💀 Sent death event to Godot: {:?}", position);
    }
}

/// ✅ Receives chemosensory packet from Godot: 20 bytes
/// Prefix (4) + x (4) + y (4) + z (4) + chemo_val (4)
pub fn try_receive_chemosense() -> Option<((usize, usize, usize), Chemosense)> {
    use crate::io::socket::SHARED_STREAM;
    use std::io::Read;

    let mut stream_lock = SHARED_STREAM.lock().unwrap();
    if let Some(stream) = stream_lock.as_mut() {
        let mut peek_buf = [0u8; 1];
        if stream.peek(&mut peek_buf).is_ok() {
            let mut buffer = [0u8; 20];
            if let Ok(_) = stream.read_exact(&mut buffer) {
                let prefix = i32::from_le_bytes(buffer[0..4].try_into().unwrap());
                if prefix != 8888 {
                    return None;
                }
                let x = i32::from_le_bytes(buffer[4..8].try_into().unwrap()) as usize;
                let y = i32::from_le_bytes(buffer[8..12].try_into().unwrap()) as usize;
                let z = i32::from_le_bytes(buffer[12..16].try_into().unwrap()) as usize;
                let chemo_val = u32::from_le_bytes(buffer[16..20].try_into().unwrap());
                return Some(((x, y, z), Chemosense(chemo_val)));
            }
        }
    }
    None
}

/// Generate an artificial hard cell for out-of-bounds or unwalkable areas
pub fn solid_wall_cell() -> Cell {
    Cell {
        flags: Bitfield(0),
        nutrients: Nutrient(0),
        density: 255,
        hardness: 255,                    // 🧱 Max hardness
        chemosense: Chemosense(0),
    }
}
