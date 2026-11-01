/*
 * Places all controls of the station manager for its current position and size
 * (skw_ui_managerWindow = [x, y, extra width, extra height]).
 */
private _display = uiNamespace getVariable ["skw_ui_managerDisplay", displayNull];
if (isNull _display) exitWith {};

skw_ui_managerWindow = [_display, skw_ui_managerTable, skw_ui_managerSize, skw_ui_managerWindow] call skw_ui_fnc_layoutWindow;
