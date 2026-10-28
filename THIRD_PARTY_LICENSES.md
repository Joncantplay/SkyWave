# Third-party components

SkyWave is licensed under the GNU General Public License v3.0 or later (see `LICENSE`).
The audio extension (`skywave_x64.dll`) includes the following third-party components.
All of them allow redistribution, including on the Steam Workshop, and are compatible with GPL-3.0.

## FAAD2 — AAC / HE-AAC decoder

- Version: 2.11.4, https://github.com/FreewareAdvancedAudio/faad2
- License: GNU General Public License v2.0 or later (used under GPL-3.0)
- Code from FAAD2 is copyright (c) Nero AG, www.nero.com

## Symphonia — MP3 decoder

- Version: 0.5.x, https://github.com/pdeljanov/Symphonia
- License: Mozilla Public License 2.0. Source code is available at the address above;
  SkyWave uses it unmodified.

## cpal — audio output

- Version: 0.15.x, https://github.com/RustAudio/cpal
- License: Apache License 2.0

## native-tls — HTTPS

- Version: 0.2.x, https://github.com/sfackler/rust-native-tls
- License: MIT or Apache License 2.0 (uses the TLS implementation built into Windows)

## Online services

SkyWave downloads station logos from the stations' own websites. When *Show album covers*
is on, it looks up track titles at MusicBrainz (https://musicbrainz.org, data under CC0)
and loads cover images from the Cover Art Archive (https://coverartarchive.org). Covers and
logos are only displayed in the game, not redistributed with the mod; their copyright stays
with their owners.

## Rust standard library and further crates

The extension is built with the Rust compiler and links the Rust standard library
(MIT or Apache License 2.0). Further crates linked into the extension, all under
permissive licenses:

| Crate | License |
|---|---|
| serde_core, serde_json, itoa (reading MusicBrainz answers) | MIT or Apache License 2.0 |
| memchr | Unlicense or MIT |
| zmij | MIT |
| aes, block-padding, cbc, cipher, cpufeatures, crypto-common, inout, typenum (AES-128 for encrypted HLS streams) | MIT or Apache License 2.0 |
| generic-array | MIT |
| arrayvec, bitflags, cfg-if, core_detect, dasp_sample, lazy_static, log, multiversion_no_op, rustversion, scopeguard, simdutf8, windows, windows-core, windows-link, windows-result, windows-sys, windows-targets, windows_x86_64_msvc | MIT or Apache License 2.0 |
| bytemuck | Zlib, Apache License 2.0 or MIT |
| encoding_rs | (Apache License 2.0 or MIT) and BSD-3-Clause |
| schannel | MIT |
| symphonia-core, symphonia-bundle-mp3, symphonia-metadata | Mozilla Public License 2.0 |
