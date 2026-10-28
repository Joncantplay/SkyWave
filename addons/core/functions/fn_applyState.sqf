/*
 * Stores the radio state of an object and shares it with every machine,
 * including players who join later. Server only; everything else goes through
 * skw_core_fnc_request.
 *
 * Arguments:
 * 0: Object <OBJECT>
 * 1: Station: class name, a stream URL, or "" for off <STRING>
 * 2: Volume 0..1 <NUMBER>
 * 3: Label shown for stations played by URL <STRING> (default: "")
 *
 * Return: true if the state was applied <BOOL>
 */
params [["_object", objNull, [objNull]], ["_station", "", [""]], ["_volume", 0.7, [0]], ["_label", "", [""]]];

if (!isServer) exitWith {false};
// Switching off always works, also when the object just stopped being a radio.
if (_station != "" && {!(_object call skw_core_fnc_isRadioCapable)}) exitWith {false};
if (count _station > 2048) exitWith {false};
if (_station == "") then {
    _label = "";
};

_object setVariable ["skw_core_state", [_station, (_volume max 0) min 1, _label select [0, 100]], true];

// Only broadcast the list when it actually changed.
private _radios = skw_core_radios select {!isNull _x && {_x != _object}};
if (_station != "") then {
    _radios pushBack _object;
};
if (count _radios != count skw_core_radios || {_station != "" && {!(_object in skw_core_radios)}}) then {
    skw_core_radios = _radios;
    publicVariable "skw_core_radios";
};
true
