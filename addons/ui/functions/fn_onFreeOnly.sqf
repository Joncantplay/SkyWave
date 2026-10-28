/*
 * "Copyright-free only" box: shows only stations without copyrighted music.
 *
 * Arguments:
 * 0: Check box <CONTROL>
 * 1: Checked (0 or 1) <NUMBER>
 */
params ["", "_checked"];

skw_ui_freeOnly = _checked == 1;
call skw_ui_fnc_fillStations;
call skw_ui_fnc_refresh;
