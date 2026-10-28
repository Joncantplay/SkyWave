/*
 * On/off button. Switching on resumes the last station (with its label), else plays the
 * selected or first station in the list.
 */
private _display = uiNamespace getVariable ["skw_ui_display", displayNull];
private _object = uiNamespace getVariable ["skw_ui_target", objNull];
if (isNull _object) exitWith {};

(_object call skw_core_fnc_getState) params ["_station", "", "_label"];
if (_station != "") exitWith {
    _object setVariable ["skw_ui_lastStation", [_station, _label]];
    _object call skw_core_fnc_turnOff;
};

(_object getVariable ["skw_ui_lastStation", ["", ""]]) params ["_last", "_lastLabel"];
if (_last != "") exitWith {
    [_object, _last, _lastLabel] call skw_core_fnc_setStation;
};

// Same as clicking the row, so personal stations are played by URL.
private _list = _display displayCtrl 1100;
if (((lnbSize _list) select 0) == 0) exitWith {
    (localize "STR_skw_ui_noStations") call skw_audio_fnc_hint;
};
[_list, (lnbCurSelRow _list) max 0] call skw_ui_fnc_onStationSelected;
