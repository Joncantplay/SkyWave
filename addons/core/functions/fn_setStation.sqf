/*
 * Tunes a radio to a station and switches it on, for all players.
 * Can be called anywhere; from a client the player must be allowed to operate the radio.
 *
 * Arguments:
 * 0: Object <OBJECT>
 * 1: Station: class name or a stream URL <STRING>
 * 2: Label shown to everyone when playing a URL <STRING> (default: "")
 *
 * Example:
 * [vehicle player, "https://stream.example.com/radio.mp3", "My Radio"] call skw_core_fnc_setStation
 */
params [["_object", objNull, [objNull]], ["_station", "", [""]], ["_label", "", [""]]];

(_object call skw_core_fnc_getState) params ["", "_volume"];
[_object, _station, _volume, _label] call skw_core_fnc_request;
