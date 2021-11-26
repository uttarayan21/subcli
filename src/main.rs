#![allow(warnings)]
mod config;
mod error;

use config::Config;
use error::Result;

use rodio::{Decoder, OutputStream, Sink};
use sunk::Streamable;

pub fn main() -> Result<()> {
    env_logger::init();

    let Config {
        server,
        username,
        password,
        bitrate,
    } = Config::try_load()?;

    let client = sunk::Client::new(&server, &username, &password)?.with_target("1.16.0".into());

    // Update the library.
    client.scan_library()?;

    // Fetch some songs and play them.
    let random = sunk::song::Song::random(&client, 1)?;
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;

    for mut song in random {
        match &song.artist {
            Some(artist) => println!("Playing {} by {:?}", song.title, artist),
            None => println!("Playing {}", song.title),
        };
        song.set_max_bit_rate(bitrate);
        let bytes: Vec<u8> = song.stream(&client)?;
        // Use cursor around the bytes vector since Vec<u8> doesn't impl Read + Seek
        let cursor = std::io::Cursor::new(bytes);

        let source = Decoder::new(cursor)?;
        sink.append(source);
    }
    sink.sleep_until_end();
    Ok(())
}
