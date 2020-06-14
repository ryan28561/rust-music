use fluidsynth::*;
use std::time::Duration;
use std::thread;
use std::vec::Vec;

pub const TRIAD: [i32; 2] = [2, 4];
pub const SEVEN: [i32; 3] = [2, 4, 6];
pub const NINE: [i32; 4] = [2, 4, 6, 8];
pub const ELEVEN: [i32; 5] = [2, 4, 6, 8, 10];
pub const THIRTEEN: [i32; 6] = [2, 4, 6, 8, 10, 12];


pub fn build_chord(chord: &[i32], key: i32, scale: &[i32]) -> Vec<i32> {
    let mut vec = vec![];
    vec.push(key + scale[0]);
    for i in chord {
        vec.push(key + scale[(*i) as usize]);
    }
    return vec;
}

pub fn play_chord(syn: &synth::Synth, chord: &Vec<i32>, duration: Option<Duration>) {
    for &key in chord {
        syn.noteon(0, key, 80);
    }
    thread::sleep(duration.unwrap_or(Duration::from_millis(500)));
    for &key in chord {
        syn.noteoff(0, key);
    }
}
