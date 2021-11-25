# subcli

Subsonic API compatible cli music player.  
I used the [Navidrome][navidrome] music server which uses 1.16 subsonic api.  
For the audio I used the [rodio][rodio] rust library.  

Currently this is more like a proof of concept.  
And can play a random song.  

I want this to use the mpris protocol so the managing is easy.  

I [updated][sunk-fork] the [sunk][sunk] library to latest versions of crates but It is still a synchronous library.  
I don't have the time or energy to convert it to an async one right now.  

# Config
Config file is stored at `~/.config/subcli/config.toml`
```toml
# ~/.config/subcli/config.toml
username = "demo"
password = "demo" 
server = "https://demo.navidrome.org" 
```


# TODO:  
- Implement mpris for better control by clients
- Try to use pulseaudio/pipewire instead or alsa/jack (should probably support all of them).
- Implement async
- Use tui-rs to make a tui

[sunk]: https://github.com/xeals/sunk
[sunk-fork]: https://github.com/uttarayan21/sunk
[navidrome]: https://navidrome.org
[rodio]: https://github.com/RustAudio/rodio
