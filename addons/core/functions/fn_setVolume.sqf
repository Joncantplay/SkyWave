/*
 * Changes the volume of a radio, for all players.
 * Can be called anywhere; from a client the player must be allowed to operate the radio.
 *
 * Arguments:
 * 0: Object <OBJECT>
 * 1: Volume 0..1 <NUMBER>
 */
params [["_object", objNull, [objNull]], ["_volume", 0.7, [0]]];

(_object call skw_core_fnc_getState) params ["_station", "", "_label"];
[_object, _station, _volume, _label] call skw_core_fnc_request;
