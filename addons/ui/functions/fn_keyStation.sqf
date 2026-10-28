/*
 * Keybind: switches the radio the player sits in or looks at to the next or previous
 * station, in the order of the radio window (favourites first, then by name). Switches it
 * on if it was off.
 *
 * Arguments:
 * 0: Direction: 1 = next, -1 = previous <NUMBER>
 *
 * Return: true if a radio was found <BOOL>
 */
params [["_step", 1, [0]]];

private _object = call skw_ui_fnc_actionTarget;
if (isNull _object) exitWith {false};

private _stations = [[_object, player] call skw_core_fnc_getStations] call skw_ui_fnc_listStations;
if (_stations isEqualTo []) exitWith {
    (localize "STR_skw_ui_noStations") call skw_audio_fnc_hint;
    true
};

(_object call skw_core_fnc_getState) params ["_current"];
private _index = _stations findIf {(_x get "class") == _current || {(_x get "url") == _current}};
_index = if (_index < 0) then {
    [0, count _stations - 1] select (_step < 0)
} else {
    (_index + _step + count _stations) % (count _stations)
};
private _record = _stations select _index;
[_object, _record] call skw_ui_fnc_playRecord;
(_record get "name") call skw_audio_fnc_hint;
true
