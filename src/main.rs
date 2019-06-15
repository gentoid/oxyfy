extern crate libpulse_binding as pulse;
extern crate libpulse_simple_binding as psimple;
extern crate minimp3;

use minimp3::{Decoder, Error, Frame};
use std::env;
use std::fs::File;

use psimple::Simple;
use pulse::stream::Direction;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let spec = pulse::sample::Spec {
        format: pulse::sample::SAMPLE_S16NE,
        channels: 2,
        rate: 44100,
    };
    assert!(spec.is_valid());

    let stream = Simple::new(
        None,                 // Use the default server
        "Oxyfy music player", // Our application’s name
        Direction::Playback,  // We want a playback stream
        None,                 // Use the default device
        "Music",              // Description of our stream
        &spec,                // Our sample format
        None,                 // Use default channel map
        None,                 // Use default buffering attributes
    )
    .unwrap();

    let mut decoder = Decoder::new(File::open(filename).unwrap());

    loop {
        match decoder.next_frame() {
            Ok(Frame { data, .. }) => {
                let splitten: Vec<[u8; 2]> = data.into_iter().map(|el| el.to_ne_bytes()).collect();
                let buffer: Vec<u8> = splitten.iter().flatten().map(|el| *el).collect();
                stream.write(&buffer).unwrap();
            }
            Err(Error::Eof) => break,
            Err(err) => panic!("{:?}", err),
        }
    }
}
