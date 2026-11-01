/*
 * Station manager list selection: loads the station into the form.
 *
 * Arguments:
 * 0: List control <CONTROL>
 * 1: Selected row <NUMBER>
 */
params ["_list", "_row"];

if (skw_ui_managerUpdating || {_row < 0}) exitWith {};
private _display = uiNamespace getVariable ["skw_ui_managerDisplay", displayNull];

private _fields = if (skw_ui_managerList == "builtin") then {
    skw_ui_managerClass = _list lnbData [_row, 0];
    private _record = skw_core_configStations getOrDefault [skw_ui_managerClass, createHashMap];
    private _off = skw_ui_managerClass in (skw_core_disabledStations call skw_core_fnc_disabledStationList);
    (_display displayCtrl 2306) ctrlSetText localize (["STR_skw_ui_disable", "STR_skw_ui_enable"] select _off);
    [
        _record getOrDefault ["name", ""],
        ([_record getOrDefault ["url", ""]] + (_record getOrDefault ["fallbacks", []])) joinString ", ",
        _record getOrDefault ["genre", ""],
        _record getOrDefault ["logo", ""],
        _record getOrDefault ["copyrightFree", false]
    ]
} else {
    skw_ui_managerIndex = parseNumber (_list lnbData [_row, 0]);
    skw_ui_managerEntries select skw_ui_managerIndex
};
_fields params ["_name", "_url", "_genre", "_logo", "_free"];
(_display displayCtrl 2201) ctrlSetText _name;
(_display displayCtrl 2202) ctrlSetText _url;
(_display displayCtrl 2203) ctrlSetText _genre;
(_display displayCtrl 2204) ctrlSetText _logo;
(_display displayCtrl 2205) cbSetChecked _free;
(_display displayCtrl 2304) ctrlSetText "";
{
    (_display displayCtrl _x) call skw_ui_fnc_fieldTooltip;
} forEach [2201, 2202, 2203, 2204];
