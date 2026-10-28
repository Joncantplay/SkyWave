/*
 * Asks for a new radio state. On the server it is applied directly; on a client the
 * server checks whether this player may operate the radio first.
 *
 * Arguments:
 * 0: Object <OBJECT>
 * 1: Station: class name, a stream URL, or "" for off <STRING>
 * 2: Volume 0..1 <NUMBER>
 * 3: Label shown for stations played by URL <STRING> (default: "")
 */
params [["_object", objNull, [objNull]], ["_station", "", [""]], ["_volume", 0.7, [0]], ["_label", "", [""]]];

if (isServer) exitWith {
    [_object, _station, _volume, _label] call skw_core_fnc_applyState;
};
[player, _object, _station, _volume, _label] remoteExecCall ["skw_core_fnc_handleRequest", 2];
