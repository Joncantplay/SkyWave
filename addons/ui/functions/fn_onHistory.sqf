/*
 * Switches the list area between the stations and the last tracks of this radio.
 */
skw_ui_historyMode = !skw_ui_historyMode;
skw_ui_historyShown = ""; // forces a refill
call skw_ui_fnc_showListMode;
call skw_ui_fnc_refresh;
