/*
 * Station manager closed: refreshes the station list of the radio window, if open.
 */
uiNamespace setVariable ["skw_ui_managerDisplay", displayNull];

private _object = uiNamespace getVariable ["skw_ui_target", objNull];
if (isNull (uiNamespace getVariable ["skw_ui_display", displayNull]) || {isNull _object}) exitWith {};
skw_ui_stations = [_object, player] call skw_core_fnc_getStations;
call skw_ui_fnc_fillStations;
