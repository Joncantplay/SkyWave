# Adding stations

SkyWave combines stations from four sources:

1. **Built-in** stations shipped with SkyWave or another mod (`CfgSkywaveStations` in a mod's `config.cpp`).
2. **Server stations**: the CBA setting *Addon Options → SkyWave → Stations → Server stations*, set by the server admin or mission maker; every player sees them.
3. **My stations**: the same kind of list, but per player (client setting).
4. **Script stations**, added on the server during a mission.

Both lists can also be edited in game in the station manager (radio window → *Manage stations*): players edit their own list, the host and logged-in admins also the server list.

The radio window lists stations alphabetically (or by genre), favourites first; each player picks their favourites with the star next to the station name.

The setting *SkyWave default stations* hides the stations that come with SkyWave, so a server can offer only its own.

## Station lists in the CBA settings

Stations are separated by `;`, the parts of a station by `|`:

```
Name | URL | option | option ...; Next station | URL
```

Options (all optional, any order):

| Option | Meaning |
|---|---|
| `free` | The station plays no copyrighted music and stays audible in Streamer Mode. |
| `genre=...` | Shown in the second column of the station list. |
| `logo=...` | Logo override: an image URL, a web page to take the logo from, or a `.paa` path. Without it, the logo is looked up on the station's website. |

Example:

```
Lo-Fi Beats | https://example.com/lofi.mp3 | free | genre=Lo-Fi; News 24 | https://example.com/news.aac | logo=https://example.com/logo.png
```

Personal stations exist only for the player who added them. When a player tunes a radio to
one, the radio plays its URL for everybody, labelled with the station's name. Servers that
disable *Allow custom stream URLs* therefore also disable personal stations. A logo set for a
personal station is only seen by its owner; other players see the logo found on the stream's
website, unless the server switches off *Allow images for custom streams*.

Supported streams: Icecast/Shoutcast (MP3, AAC, HE-AAC), `.m3u` and `.pls` playlists, and
HLS (`.m3u8`, including AES-128 encrypted streams).

## Logos

Unless a station sets a logo, SkyWave looks for one on the station's website: the address
the stream announces (`icy-url`), otherwise the main domain of the stream server. It uses
the site's app icon, large favicon or share image. Logos need Arma 3 2.20 or newer.

## Mod config

```cpp
class CfgSkywaveStations {
    class myTag_classicRock {
        displayName = "Classic Rock 24/7";            // shown in the station list
        url = "https://stream.example.com/rock.mp3";  // required
        fallbackUrls[] = {                            // optional, tried in order if the URL fails
            "https://backup.example.com/rock.aac"
        };
        logo = "https://rock.example.com";             // optional override: image URL, web page or .paa path
        genre = "Rock";                                // optional
        description = "Rock classics without ads.";    // optional, tooltip
        copyrightFree = 0;                             // 1 = no copyrighted music, plays in Streamer Mode
        condition = "side group _unit == west";        // optional SQF, see below
        vehicles[] = {"Car", "Helicopter"};            // optional, only on these classes (isKindOf)
        hidden = 0;                                    // 1 = not listed, but can be set by script
    };
};
```

### Conditions

`condition` is SQF code that returns `true` when the station is available. It runs with
`_object` (the vehicle or radio) and `_unit` (the player choosing the station). It is
checked on the player's machine for the station list and again on the server when the
station is chosen. Zeus may choose any existing station.

## Adding stations by script

On the server, during the mission:

```sqf
// class, name, url, fallbacks, logo, genre, description, copyrightFree, condition, vehicles, hidden
["myMission_news", "Field News", "https://example.com/news.mp3", [], "", "News"] call skw_core_fnc_addStation;
"myMission_news" call skw_core_fnc_removeStation;
```

Script stations reach every player, including players who join later.

## Controlling radios by script

```sqf
[_vehicle, "myTag_classicRock"] call skw_core_fnc_setStation;          // station class
[_vehicle, "https://example.com/x.mp3", "My Stream"] call skw_core_fnc_setStation; // URL with a label
[_vehicle, 0.5] call skw_core_fnc_setVolume;                            // 0..1
_vehicle call skw_core_fnc_turnOff;
(_vehicle call skw_core_fnc_getState) params ["_station", "_volume", "_label"];
```

Called on the server, these apply directly. Called on a client, the server checks that the
player may operate the radio and that the station is available.

SkyWave comes with a placeable radio (`skw_radio`). Other objects get a radio when their config sets `skw_radio = 1;`.
