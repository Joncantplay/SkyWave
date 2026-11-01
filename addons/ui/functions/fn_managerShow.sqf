/*
 * Opens the station manager (if needed) and shows one of its lists.
 *
 * Arguments:
 * 0: List: "personal" (this player), "global" (server-wide) or "builtin" (stations that
 *    come with SkyWave or other mods, which the server can switch off) <STRING>
 */
params [["_list", "personal", [""]]];

private _display = uiNamespace getVariable ["skw_ui_managerDisplay", displayNull];
if (isNull _display) then {
    createDialog "skw_ui_manager";
    _display = uiNamespace getVariable ["skw_ui_managerDisplay", displayNull];
    if (isNull _display) exitWith {};
    // Position and size from last time, else centred.
    skw_ui_managerWindow = profileNamespace getVariable ["skw_ui_managerWindow", []];
    if !(skw_ui_managerWindow isEqualTypeArray [0, 0, 0, 0]) then {
        skw_ui_managerWindow = [skw_ui_managerSize, [0, 0]] call skw_ui_fnc_defaultWindow;
    };
    call skw_ui_fnc_managerLayout;
    [_display, [[2017, "move"], [2018, "width"], [2019, "height"], [2020, "corner"]], "manager"] call skw_ui_fnc_addDragHandles;
};
if (isNull _display) exitWith {};

(_display displayCtrl 2000) ctrlSetText ("  SkyWave - " + localize "STR_skw_ui_managerTitle");

// The server list and the built-in stations can be changed by the host and logged-in admins.
private _builtin = _list == "builtin";
private _editable = _list == "personal" || {isServer} || {[] call CBA_settings_fnc_whitelisted};
skw_ui_managerList = _list;
skw_ui_managerEditable = _editable;
skw_ui_managerEntries = switch (_list) do {
    case "personal": {[skw_core_personalStations] call skw_core_fnc_stationListEntries};
    case "global": {[skw_core_serverStations] call skw_core_fnc_stationListEntries};
    default {[]};
};

private _active = [0.96, 0.58, 0.16, 1];
private _inactive = [1, 1, 1, 1];
{
    _x params ["_idc", "_name"];
    (_display displayCtrl _idc) ctrlSetTextColor ([_inactive, _active] select (_list == _name));
} forEach [[2001, "personal"], [2002, "global"], [2004, "builtin"]];
(_display displayCtrl 2003) ctrlSetText localize (switch (true) do {
    case (_list == "personal"): {"STR_skw_ui_notePersonal"};
    case (_builtin && {_editable}): {"STR_skw_ui_noteBuiltin"};
    case (_builtin): {"STR_skw_ui_noteBuiltinReadOnly"};
    case (_editable): {"STR_skw_ui_noteServer"};
    default {"STR_skw_ui_noteServerReadOnly"};
});

// Built-in stations are only shown and switched on or off, not edited.
{
    (_display displayCtrl _x) ctrlEnable (_editable && {!_builtin});
} forEach [2201, 2202, 2203, 2204, 2205];
{
    (_display displayCtrl _x) ctrlEnable _editable;
    (_display displayCtrl _x) ctrlShow !_builtin;
} forEach [2301, 2302, 2303];
(_display displayCtrl 2306) ctrlShow _builtin;
(_display displayCtrl 2306) ctrlEnable _editable;

call skw_ui_fnc_managerFill;
call skw_ui_fnc_managerNew;
