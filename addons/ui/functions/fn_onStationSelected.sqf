/*
 * Station list selection: tunes the radio to the chosen station.
 *
 * Arguments:
 * 0: List control <CONTROL>
 * 1: Selected row <NUMBER>
 */
params ["_list", "_row"];

if (skw_ui_updating || {_row < 0}) exitWith {};

private _object = uiNamespace getVariable ["skw_ui_target", objNull];
[_object, (_list lnbData [_row, 0]) call skw_core_fnc_getStation] call skw_ui_fnc_playRecord;
