/*
 * Saves the edited list into its CBA setting: the personal list in this player's profile,
 * the server list on the server (sent there for admins).
 */
private _text = [skw_ui_managerEntries] call skw_core_fnc_stationListText;
if (skw_ui_managerList == "personal") then {
    ["skw_core_personalStations", _text, "client"] call skw_core_fnc_storeSetting;
} else {
    ["skw_core_serverStations", _text, "server"] call skw_core_fnc_storeSetting;
};
