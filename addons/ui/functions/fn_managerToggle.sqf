/*
 * "Built-in" tab: switches the selected built-in station off for everyone on the server,
 * or on again. Host and logged-in admins only.
 */
private _display = uiNamespace getVariable ["skw_ui_managerDisplay", displayNull];
if (isNull _display || {!skw_ui_managerEditable} || {skw_ui_managerList != "builtin"}) exitWith {};

if (skw_ui_managerClass == "") exitWith {
    (_display displayCtrl 2304) ctrlSetText localize "STR_skw_ui_selectFirst";
};

private _class = skw_ui_managerClass;
private _disabled = skw_core_disabledStations call skw_core_fnc_disabledStationList;
private _off = _class in _disabled;
if (_off) then {
    _disabled = _disabled - [_class];
} else {
    _disabled pushBack _class;
};
["skw_core_disabledStations", _disabled joinString ", ", "server"] call skw_core_fnc_storeSetting;

private _name = (skw_core_configStations getOrDefault [_class, createHashMap]) getOrDefault ["name", _class];
call skw_ui_fnc_managerFill;
// Keep the station selected so it can be switched back right away.
private _list = _display displayCtrl 2100;
for "_i" from 0 to (((lnbSize _list) select 0) - 1) do {
    if ((_list lnbData [_i, 0]) == _class) exitWith {
        _list lnbSetCurSelRow _i;
    };
};
(_display displayCtrl 2304) ctrlSetText format [localize (["STR_skw_ui_disabled", "STR_skw_ui_enabled"] select _off), _name];
