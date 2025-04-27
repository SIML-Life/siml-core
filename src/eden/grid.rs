use crossterm::{
    cursor::MoveTo,
    execute
};
use glam::Vec3;
use std::io::{Write, stdout};

#[derive(Clone)]
pub struct Voxel {
    pub light: u8,
}

pub struct World {
    pub size: (usize, usize, usize),
    pub voxels: Vec<Voxel>,
}

impl World {
    pub fn new(size: (usize, usize, usize)) -> Self {
        let total = size.0 * size.1 * size.2;
        Self {
            size,
            voxels: vec![Voxel { light: 0 }; total],
        }
    }

    pub fn update_lighting(&mut self, light_vec: Vec3, light_brightness: f32) {
        let normal = Vec3::new(0.0, 1.0, 0.0); // World is flat
        let incidence = light_vec.dot(normal).max(0.0);
        let brightness = (incidence * light_brightness * 255.0).min(255.0) as u8;

        for voxel in self.voxels.iter_mut() {
            voxel.light = brightness;
        }
    }

    pub fn average_light(&self) -> f32 {
        let total_light: u32 = self.voxels.iter().map(|v| v.light as u32).sum();
        let num_voxels = self.voxels.len() as u32;
        (total_light as f32) / (num_voxels as f32) / 255.0 // normalize 0.0-1.0
    }

    pub fn display_ascii(&self) {
        let mut stdout = stdout();
        
        // 1. Move to (0,0)
        execute!(stdout, MoveTo(0, 0)).unwrap();
        
        let (width, height, _depth) = self.size;
        for y in 0..height {
            for x in 0..width {
                let idx = x + y * width;
                let brightness = self.voxels[idx].light;
                let symbol = match brightness {
                    150..=255 => '█',
                    100..=149 => '▓',
                    50..=99   => '▒',
                    20..=49   => '░',
                    _         => '.',
                };
                print!("{}", symbol);
            }
            println!();
        }
    
        stdout.flush().unwrap();
    }    
}
