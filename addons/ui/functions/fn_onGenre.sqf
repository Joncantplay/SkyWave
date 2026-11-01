/*
 * Genre box changed: shows only stations of that genre.
 *
 * Arguments:
 * 0: Combo box <CONTROL>
 * 1: Selected row <NUMBER>
 */
params ["_combo", "_row"];

if (skw_ui_updating || {_row < 0}) exitWith {};
skw_ui_genreFilter = _combo lbData _row;
call skw_ui_fnc_fillStations;
call skw_ui_fnc_refresh;
