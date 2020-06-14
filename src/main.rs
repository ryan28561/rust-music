extern crate fluidsynth;

mod scale;
mod chord;

use fluidsynth::*;
//use rand::{thread_rng, Rng};
//use std::thread;
use std::time::Duration;
use scale::*;
use chord::*;

fn main() {
    let mut settings = settings::Settings::new();
    settings.setstr("audio.driver", "alsa");
    let mut syn = synth::Synth::new(&mut settings);
    let _adriver = audio::AudioDriver::new(&mut settings, &mut syn);
    syn.sfload("./Steinway.sf2", 1);
    
    let interval = Duration::from_millis(1000);
    
    let chord: Vec<i32> = build_chord(&TRIAD, 52, &MAJOR);
    let chord1: Vec<i32> = build_chord(&TRIAD, 57, &MAJOR);
    let chord2: Vec<i32> = build_chord(&SEVEN, 47, &MIXOLYDIAN);
    play_scale(&syn, &MAJOR, 40, Some(interval));
    play_chord(&syn, &chord, Some(interval));
    play_chord(&syn, &chord1, Some(interval));
    play_chord(&syn, &chord, Some(interval));
    play_chord(&syn, &chord2, Some(interval));
    play_chord(&syn, &chord, Some(interval));

}
