#![allow(warnings)]
mod config;
mod error;

use config::Config;
use error::Result;

use rodio::{Decoder, OutputStream, Sink};
use sunk::Streamable;

pub fn main() -> std::result::Result<(), error::Error> {
    env_logger::init();

    let Config {
        server,
        username,
        password,
    } = Config::try_load()?;

    let client = sunk::Client::new(&server, &username, &password)
        .unwrap()
        .with_target("1.16.0".into());

    // Update the library.
    client.scan_library().unwrap();

    // Fetch some songs and play them.
    let random = sunk::song::Song::random(&client, 1).unwrap();
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    for mut song in random {
        match &song.artist {
            Some(artist) => println!("Playing {} by {:?}", song.title, artist),
            None => println!("Playing {}", song.title),
        };
        song.set_max_bit_rate(128);
        let bytes: Vec<u8> = song.stream(&client).unwrap();
        // Use cursor around the bytes vector since Vec<u8> doesn't impl Read + Seek
        let cursor = std::io::Cursor::new(bytes);

        let source = Decoder::new(cursor).unwrap();
        sink.append(source);
    }
    sink.sleep_until_end();
    Ok(())
}
