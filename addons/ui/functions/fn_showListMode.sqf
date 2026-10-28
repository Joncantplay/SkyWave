/*
 * Shows either the station list with its column headers or the track history.
 */
private _display = uiNamespace getVariable ["skw_ui_display", displayNull];
if (isNull _display) exitWith {};

{
    (_display displayCtrl _x) ctrlShow !skw_ui_historyMode;
} forEach [1100, 1710, 1711, 1712];
(_display displayCtrl 1105) ctrlShow skw_ui_historyMode;
(_display displayCtrl 1714) ctrlSetText localize (["STR_skw_ui_history", "STR_skw_ui_backToStations"] select skw_ui_historyMode);
