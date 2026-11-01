/*
 * Places all controls of the radio window for its current position and size
 * (skw_ui_window = [x, y, extra width, extra height]).
 */
private _display = uiNamespace getVariable ["skw_ui_display", displayNull];
if (isNull _display) exitWith {};

skw_ui_window = [_display, skw_ui_layoutTable, skw_ui_windowSize, skw_ui_window] call skw_ui_fnc_layoutWindow;
call skw_ui_fnc_placeSortArrow;
