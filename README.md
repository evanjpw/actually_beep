# actually_beep

_A library to **actually** beep._

[![crates.io](https://img.shields.io/crates/v/actually_beep.svg)](https://crates.io/crates/actually_beep)
[![docs.rs](https://docs.rs/actually_beep/badge.svg)](https://docs.rs/actually_beep)
![MSRV](https://img.shields.io/crates/msrv/actually_beep)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
<!-- (https://img.shields.io/crates/l/actually_beep.svg) -->

## Overview

This is a cross-platform library to provides easy to use beep functions that actually beep. It
provides very basic audio tones (beeps) for whatever use you may have for that. It’s built on
the AMAZING [CPAL](https://crates.io/crates/beep) crate, which provides the low level audio
support, & also provides excellent examples from which this crate stole heavily.

## Install

Are we really doing this?

```toml
# Add this to your [dependencies] section in Cargo.toml
actually_beep = "0.1.0"
```

## Usage

```rust
use actually_beep::beep_with_hz_and_millis;

fn main() {
  let middle_e_hz = 329;
  let a_bit_more_than_a_second_and_a_half_ms = 1600;

  beep_with_hz_and_millis(middle_e_hz, a_bit_more_than_a_second_and_a_half_ms).unwrap();
}
```

## Crate features

* `jack` - Use the "jack" feature of `cpal`. Only useful on Linux & the BSDs.

## Motivation

I was writing one of my [1970s](https://github.com/evanjpw/startrust)
[retro-game](https://github.com/evanjpw/wump)
[re-writes](https://github.com/evanjpw/retro-robots), & I needed to be able to
generate a beep (for period *verisimilitude*). I had needed this before, & I found the
[beep crate](https://crates.io/crates/beep), which I used, but had to stop using
because it wasn't cross-platform.

Then I was writing a new 1970s retro game & needed a beep, so I looked further. There are a
variety of platform dependent crates that have a similar function, including:

* [beep](https://crates.io/crates/beep)
* [beep-evdev](https://crates.io/crates/beep-evdev)
* [win-beep](https://crates.io/crates/win-beep)

None of these are cross-platform, & they are all different. This crate fixes that by providing
a general beeping interface that works on all platforms.

There are also some cool libraries that are **AWESOME**, but way more than I needed.
One of these was [CPAL](https://crates.io/crates/cpal), which was not what I needed,
but with which I could _build_ what I needed.

# Futures

Currently, there is one call that blocks. There is no “beep indefinitely” call as the beep
crate provides, & it doesn't provide any way to adjust things like the audio device, host
audio system, or anything else.

These things are potentially, possibly, something that may be worked on.

<!-- ## Notes -->

## License

Licensed under:

* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

## Thanks

Thanks to the authors of `beep`, `beep_evdev`, & `win-beep` for inspiration.

Unimaginable, ridiculous thanks to the author(s) of `CPAL`.

<!-- ## Contributing -->
