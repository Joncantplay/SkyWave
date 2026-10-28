<p align="center">
  <img src="docs/logo.png" width="560" alt="SkyWave">
</p>

<p align="center">
  <a href="https://steamcommunity.com/sharedfiles/filedetails/?id=3805849328"><img src="https://img.shields.io/badge/Steam-Workshop-1B2838?style=for-the-badge&logo=steam&logoColor=white" alt="Steam Workshop"></a>
  <a href="https://github.com/Joncantplay/SkyWave/releases"><img src="https://img.shields.io/badge/version-1.0.0-F9A41E?style=for-the-badge" alt="Version 1.0.0"></a>
  <img src="https://img.shields.io/badge/Arma%203-2.20%2B-E8336B?style=for-the-badge" alt="Arma 3 2.20 or newer">
  <a href="https://steamcommunity.com/workshop/filedetails/?id=450814997"><img src="https://img.shields.io/badge/requires-CBA__A3-4B1270?style=for-the-badge" alt="Requires CBA_A3"></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-GPL--3.0-2F6FB2?style=for-the-badge" alt="License GPL-3.0"></a>
</p>

<p align="center">
  <b>Internet radio for Arma 3.</b> Live streams in vehicles and on placed radios, in 3D and in sync for every player.
</p>

<p align="center">
  <a href="https://steamcommunity.com/sharedfiles/filedetails/?id=3805849328">Steam Workshop</a> ·
  <a href="#features">Features</a> ·
  <a href="docs/settings.md">Settings</a> ·
  <a href="docs/stations.md">Stations</a> ·
  <a href="docs/server.md">Server</a> ·
  <a href="CHANGELOG.md">Changelog</a>
</p>

## Features

**Streams**
- MP3, AAC and HE-AAC (AAC+) from Icecast/Shoutcast, `.m3u`/`.pls` playlists and HLS (`.m3u8`,
  including AES-128 encrypted streams).
- Lost connections are retried automatically; stations can have fallback addresses.
- Live track titles, station logos and album covers.

**Sound**
- 3D audio: radios get quieter with distance, come from the right direction and sound muffled
  behind closed doors.
- Inside a vehicle the radio sounds as if it came from the front of the cabin, or plain stereo
  if you prefer; selectable for cars and boats only or for all vehicles.
- Per player: master volume, volume boost up to 4x (with a soft limiter), mute one radio or all
  radios, mute while Arma is in the background. ACE earplugs lower the radio as well.

**Multiplayer**
- The server keeps every radio's station and volume; everybody hears the same, also players who
  join later.
- Permission checks on the server: who may operate a radio from where, which stations are
  allowed.

**Radio window**
- Station list with search, sorting by name or genre, a *copyright-free only* filter and
  favourites at the top.
- Now playing: station logo or album cover, track title (click to copy), connection status.
- On/off, random station, volume, mute for me, custom stream URL, the last ten tracks.
- Movable and resizable; the size of text and controls is a setting.

**Stations**
- 25 built-in stations, a server-wide list and a personal list per player, editable in game in
  the station manager.
- Stations from other mods (`CfgSkywaveStations`) and from mission scripts, with conditions per
  station.

**Streamer Mode**
- Players silence everything that is not marked copyright-free, only for themselves (setting,
  key or button in the radio window).
- Servers can allow copyright-free stations only.

**Zeus, Eden and ACE**
- Zeus modules: open any radio, turn any object into a radio, remove a radio, lock a radio for
  Zeus only.
- A placeable radio object; Eden attributes for the starting station and volume.
- Entries in the ACE interaction menu, next to the vehicle's own actions.

**Keys**
- Open the radio, louder/quieter and next/previous station without opening the window, Streamer
  Mode, mute all.

**Server options**
- Custom stream URLs on/off, pictures for custom streams on/off, copyright-free only, radio only
  with the engine running, operation from outside the vehicle, driver and commander only, audible
  range.

English and German.

## Requirements

- Arma 3 2.20 or newer
- [CBA_A3](https://steamcommunity.com/workshop/filedetails/?id=450814997)
- ACE3 is optional.

The audio runs in an extension (`skywave_x64.dll`) on each player's PC. On servers with
BattlEye the extension only loads once BattlEye has whitelisted it; until then radios stay silent
for players with BattlEye enabled. See [docs/server.md](docs/server.md).

## Documentation

- [docs/settings.md](docs/settings.md): settings, keys, the radio window, who can operate a radio,
  Eden and Zeus
- [docs/stations.md](docs/stations.md): adding stations (in game, CBA settings, mod config,
  scripts) and controlling radios by script
- [docs/server.md](docs/server.md): running SkyWave on a server
- [CHANGELOG.md](CHANGELOG.md)

## Building

The extension is written in Rust, the mod is built with HEMTT. GitHub Actions builds and tests
both on every push (`.github/workflows/build.yml`) and uploads the finished mod as an artifact.

## License

Copyright (C) 2026 Joncantplay

SkyWave is free software: you can redistribute it and/or modify it under the terms of the GNU
General Public License as published by the Free Software Foundation, either version 3 of the
License, or (at your option) any later version. See [LICENSE](LICENSE).

Third-party components and their licenses are listed in
[THIRD_PARTY_LICENSES.md](THIRD_PARTY_LICENSES.md). AAC decoding: code from FAAD2 is copyright
(c) Nero AG, www.nero.com.
