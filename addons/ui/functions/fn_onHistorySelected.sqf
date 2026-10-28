/*
 * Click on a track in the history: copies it to the clipboard.
 *
 * Arguments:
 * 0: List control <CONTROL>
 * 1: Selected row <NUMBER>
 */
params ["_list", "_row"];

if (skw_ui_updating || {_row < 0}) exitWith {};
private _title = _list lnbData [_row, 0];
if (_title != "") then {
    _title call skw_ui_fnc_copyTitle;
};

// Deselect, so the same track can be clicked again.
skw_ui_updating = true;
_list lnbSetCurSelRow -1;
skw_ui_updating = false;
