/*
 * Station manager closed: remembers where it was and refreshes the station list of the
 * radio window, if open.
 */
call skw_ui_fnc_stopDrag;
uiNamespace setVariable ["skw_ui_managerDisplay", displayNull];
if (skw_ui_managerWindow isEqualTypeArray [0, 0, 0, 0]) then {
    profileNamespace setVariable ["skw_ui_managerWindow", skw_ui_managerWindow];
    saveProfileNamespace;
};

private _object = uiNamespace getVariable ["skw_ui_target", objNull];
if (isNull (uiNamespace getVariable ["skw_ui_display", displayNull]) || {isNull _object}) exitWith {};
skw_ui_stations = [_object, player] call skw_core_fnc_getStations;
call skw_ui_fnc_fillGenres;
call skw_ui_fnc_fillStations;
