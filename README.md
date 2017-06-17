# pulseaudio-next-sink
[![Cargo version][cargo-image]][cargo-url]

pulseaudio-next-sink is a command-line program that:
* Finds and set the next available default sink
* Also transfer all sink-inputs to the same new default sink

## Full Usage
```
pulseaudio-next-sink
```

## Installation
* Install rust (https://www.rust-lang.org/en-US/downloads.html)
* Run `cargo install pulseaudio-next-sink`
* Run `~/.cargo/bin/pulseaudio-next-sink` (you can add `~/.cargo/bin/` to `PATH`)

[cargo-image]: https://img.shields.io/crates/v/pulseaudio-next-sink.svg
[cargo-url]: https://crates.io/crates/pulseaudio-next-sink
