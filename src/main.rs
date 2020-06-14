extern crate fluidsynth;

use fluidsynth::*;
use rand::{thread_rng, Rng};
use std::thread;
use std::time::Duration;

fn main() {
    let mut settings = settings::Settings::new();
    settings.setstr("audio.driver", "alsa");
    let mut syn = synth::Synth::new(&mut settings);
    let _adriver = audio::AudioDriver::new(&mut settings, &mut syn);
    syn.sfload("./Steinway.sf2", 1);

    let interval = Duration::from_millis(1000);

    for _x in 0..12 {
        let num: i32 = thread_rng().gen_range(0, 12);
        let key = num + 60;
        syn.noteon(0, key, 80);
        thread::sleep(interval);
        syn.noteoff(0, key);
    }
}
