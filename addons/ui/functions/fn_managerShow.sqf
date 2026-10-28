/*
 * Opens the station manager (if needed) and shows one of the two lists.
 *
 * Arguments:
 * 0: List: "personal" (this player) or "global" (server-wide) <STRING>
 */
params [["_list", "personal", [""]]];

private _display = uiNamespace getVariable ["skw_ui_managerDisplay", displayNull];
if (isNull _display) then {
    createDialog "skw_ui_manager";
    _display = uiNamespace getVariable ["skw_ui_managerDisplay", displayNull];
};
if (isNull _display) exitWith {};

(_display displayCtrl 2000) ctrlSetText ("  SkyWave - " + localize "STR_skw_ui_managerTitle");

// The server list can be changed by the host and by logged-in admins.
private _editable = _list == "personal" || {isServer} || {[] call CBA_settings_fnc_whitelisted};
skw_ui_managerList = _list;
skw_ui_managerEditable = _editable;
skw_ui_managerEntries = [[skw_core_personalStations, skw_core_serverStations] select (_list == "global")] call skw_core_fnc_stationListEntries;

private _active = [0.96, 0.58, 0.16, 1];
private _inactive = [1, 1, 1, 1];
(_display displayCtrl 2001) ctrlSetTextColor ([_inactive, _active] select (_list == "personal"));
(_display displayCtrl 2002) ctrlSetTextColor ([_inactive, _active] select (_list == "global"));
(_display displayCtrl 2003) ctrlSetText localize (switch (true) do {
    case (_list == "personal"): {"STR_skw_ui_notePersonal"};
    case (_editable): {"STR_skw_ui_noteServer"};
    default {"STR_skw_ui_noteServerReadOnly"};
});
{
    (_display displayCtrl _x) ctrlEnable _editable;
} forEach [2201, 2202, 2203, 2204, 2205, 2301, 2302, 2303];

call skw_ui_fnc_managerFill;
call skw_ui_fnc_managerNew;
