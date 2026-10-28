# Settings

All settings are in *Options → Addon Options → SkyWave*. "Server" settings are decided by
the server (or the mission) and apply to everyone; the others are each player's own.

| Category | Setting | Scope | Default | Meaning |
|---|---|---|---|---|
| Audio | Master volume | player | 80 % | Volume of all radios for this player. |
| Audio | Volume boost | player | 1x | Multiplies the volume (up to 4x) for quiet streams; loud peaks are rounded off instead of distorting. |
| Audio | Audible range | server | 120 m | Distance up to which radios can be heard. |
| Audio | 3D sound inside vehicles | player | Cars and boats only | The radio of your own vehicle sounds as if it came from the front centre of the cabin (also in third person). Off: plain stereo. Also: all vehicles (including helicopters and planes). |
| Audio | Mute in background | player | off | Silence radios while Arma is not the active window. |
| Audio | Mute all radios | player | off | Silence every radio for this player only. |
| Streamer Mode | Streamer Mode | player | off | Silence stations not marked copyright-free, for this player only. |
| Status | Status messages | player | Off | Show connection status and track changes as hint or in the chat. |
| Status | Messages from Zeus radios | player | on | Status messages also for radios a Zeus created; off: only for normal radios. |
| Status | Show album covers | player | on | Look up the cover of the current track (track titles go to musicbrainz.org, images come from coverartarchive.org). |
| Server | Allow custom stream URLs | server | on | Players may play any stream address (Zeus always may). |
| Server | Allow images for custom streams | server | on | Logos and album covers for streams played by URL (custom URLs, players' own stations). Off: those show the default radio icon. |
| Server | Radio only with engine running | server | off | Vehicle radios fall silent while the engine is off (the stream keeps running, so it plays on at once). Placed radios are not affected. |
| Server | Copyright-free stations only | server | off | Only stations marked copyright-free can be played. |
| Server | Operate from outside | server | off | Players standing next to a vehicle (about 4 m from it) can operate its radio. |
| Server | Driver and commander only | server | off | Inside a vehicle only driver (pilot) and commander can operate the radio. |
| Stations | SkyWave default stations | server | on | Offer the stations that come with SkyWave. |
| Stations | Server stations | server | empty | Station list for everyone, see [stations.md](stations.md). |
| Stations | My stations | player | empty | Personal station list, see [stations.md](stations.md). |
| Controls | Scroll wheel entry | player | on | Show "SkyWave radio" in the action menu. With ACE the radio is also in the interaction menu. |
| Controls | Radio window size | player | 100 % | Size of text, buttons and title bar in the radio window (80–140 %). The window itself is resized by dragging its right or bottom edge or its corner. |
| Controls | Reset radio window | player | — | Tick and press OK: the radio window goes back to the middle of the screen at normal size (if it got lost while dragging). Unticks itself. |

## Keys

*Options → Controls → Configure Addons → SkyWave*:

| Action | Default |
|---|---|
| Open radio | Ctrl + Shift + R |
| Toggle Streamer Mode | not set |
| Mute / unmute all radios | not set |
| Radio louder / quieter (10 % steps) | not set |
| Next / previous station | not set |

The last four work on the vehicle you sit in or the radio you look at, without opening the
window. Next/previous follow the order of the radio window: favourites first.

## Streamer Mode

Streamer Mode only changes what the player hears: stations without the copyright-free mark
are not downloaded or played for them, while other players keep listening. A stream played
by URL counts as copyright-free when a known station with the same URL is marked so.
Players toggle it in the settings, with the button in the radio window's title bar, or with
a key.

## Who can operate a radio

- **Zeus:** every radio, also through the Zeus module *SkyWave radio* (place it on a vehicle or radio).
- **Vehicles:** everyone inside; with *Driver and commander only* just those two seats; with
  *Operate from outside* also players standing next to it (about 4 m from the vehicle).
- **Placed radios** (*SkyWave radio* object in the editor and for Zeus): everyone within about 3 m.
- **Any object Zeus turned into a radio** (module *Turn object into radio*): from close by,
  and from inside if it is a vehicle.

Zeus removes the radio of any vehicle or object with the module *Remove radio*; *Turn object
into radio* gives it back.

With the module *Zeus control only* Zeus locks a radio so players can no longer operate it,
from inside or outside; placing it again unlocks it.

Distances count from the object's edge, so large objects like buildings work as well.

The radio opens from the action menu (scroll wheel) when sitting in a vehicle or looking at
one or at a placed radio, or with the key *Open radio*.

With ACE, the radio is also in the ACE interaction menu (interaction key) of vehicles, placed
radios and objects Zeus turned into a radio. Sitting in a vehicle, the interaction key shows
it among the vehicle's own actions, next to "Change seat".

## Eden editor

Vehicles and SkyWave radios have the attributes *SkyWave → Station at mission start* (a list of
the built-in stations, or *(off)*) and *Radio volume*. For a stream URL or a station from a
server list, put `if (isServer) then {[this, "URL or station"] call skw_core_fnc_setStation}`
in the object's init field.

## Radio window

- **Title bar:** drag it to move the window; the button switches Streamer Mode. Drag the right
  edge, the bottom edge or the bottom right corner to resize; the station list grows with the
  window. Position and size are kept per player.
- **Now playing:** station logo or album cover, station name, current track (click to copy it)
  and connection status. *Retry* appears next to the status when the connection was lost.
- **Star** next to the station name: adds the station to your favourites; they stand at the
  top of the list (kept in your profile).
- **Buttons:** on/off (resumes the last station), *Random station* (from the list as shown,
  search and filter apply), *Mute for me* (only you).
- **Station list:** search, *Copyright-free only*, click a column header to sort by name or
  genre (click again to reverse). Copyright-free stations are marked, personal ones are orange.
- **Recently played** above the list: the last ten tracks of this radio; click one to copy it.
- **Custom stream URL** (if the server allows it) and *Manage stations* for the station lists.
- Status messages (setting *Status messages*) only come from the radio you sit in, stand
  next to or have open.
