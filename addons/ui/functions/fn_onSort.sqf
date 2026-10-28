/*
 * Click on a column header: sorts the station list by it; clicking again reverses.
 *
 * Arguments:
 * 0: Column: "name" or "genre" <STRING>
 */
params [["_key", "name", [""]]];

skw_ui_sort params ["_current", "_ascending"];
skw_ui_sort = [_key, [true, !_ascending] select (_key == _current)];
call skw_ui_fnc_fillStations;
call skw_ui_fnc_refresh;
