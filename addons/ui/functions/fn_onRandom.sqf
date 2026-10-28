/*
 * "Random station" button: tunes to a random station from the list as it is shown
 * (search and "copyright-free only" apply), never the one already playing.
 */
private _display = uiNamespace getVariable ["skw_ui_display", displayNull];
if (isNull _display) exitWith {};

private _list = _display displayCtrl 1100;
private _current = lnbCurSelRow _list;
private _choices = [];
for "_i" from 0 to (((lnbSize _list) select 0) - 1) do {
    if (_i != _current) then {
        _choices pushBack _i;
    };
};
if (_choices isEqualTo []) exitWith {};

// Selecting the row tunes the radio (skw_ui_fnc_onStationSelected).
_list lnbSetCurSelRow (selectRandom _choices);
