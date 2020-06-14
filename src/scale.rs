
use fluidsynth::*;
use std::time::Duration;
//use phf::phf_map;
use std::thread;

pub const MAJOR: [i32; 7] = [0, 2, 4, 5, 7, 9, 11];
pub const MINOR: [i32; 7] = [0, 2, 3, 5, 7, 8, 10];
pub const HARMONIC_MINOR: [i32; 7] = [2, 3, 5, 7, 8, 11, 12];

pub const MIXOLYDIAN: [i32; 7] = [0, 2, 4, 5, 7, 9, 10];

// Test function for scales.
// Automatically plays in channel 0 with velocity 80
pub fn play_scale(syn: &synth::Synth, scale: &[i32], key: i32, duration: Option<Duration>) {
    for inc in scale {
        syn.noteon(0, key + inc, 80);
        thread::sleep(duration.unwrap_or(Duration::from_millis(500)));
        syn.noteoff(0, key + inc);
    }
}
