/*
 * Keybind: makes the radio the player sits in or looks at louder or quieter.
 *
 * Arguments:
 * 0: Change, e.g. 0.1 or -0.1 <NUMBER>
 *
 * Return: true if a radio was found <BOOL>
 */
params [["_change", 0.1, [0]]];

private _object = call skw_ui_fnc_actionTarget;
if (isNull _object) exitWith {false};

(_object call skw_core_fnc_getState) params ["_station", "_volume"];
if (_station == "") exitWith {
    (localize "STR_skw_ui_statusOff") call skw_audio_fnc_hint;
    true
};
// Own volume for this radio if the player chose one, else the shared volume.
private _ownVolume = _object getVariable ["skw_audio_ownVolume", -1];
if (_ownVolume >= 0) then {
    _volume = _ownVolume;
};
private _new = ((round ((_volume + _change) * 100)) / 100) max 0 min 1;
if (_ownVolume >= 0) then {
    _object setVariable ["skw_audio_ownVolume", _new];
} else {
    [_object, _new] call skw_core_fnc_setVolume;
};
(format [localize "STR_skw_ui_volume", round (_new * 100)]) call skw_audio_fnc_hint;
true
