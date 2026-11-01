/*
 * Fills the station manager's list: the entries being edited, or on the "Built-in" tab
 * every built-in station, switched-off ones marked.
 */
private _display = uiNamespace getVariable ["skw_ui_managerDisplay", displayNull];
if (isNull _display) exitWith {};

private _list = _display displayCtrl 2100;
skw_ui_managerUpdating = true;
lnbClear _list;
if (skw_ui_managerList == "builtin") then {
    private _disabled = skw_core_disabledStations call skw_core_fnc_disabledStationList;
    private _rows = [];
    {
        _rows pushBack [toLower (_y get "name"), _x, _y];
    } forEach skw_core_configStations;
    _rows sort true;
    {
        _x params ["", "_class", "_record"];
        private _off = _class in _disabled;
        private _info = [_record get "genre", localize "STR_skw_ui_disabledMark"] select _off;
        private _row = _list lnbAddRow [_record get "name", _info];
        _list lnbSetData [[_row, 0], _class];
        _list lnbSetTooltip [[_row, 0], _record get "name"];
        _list lnbSetColor [[_row, 0], [[1, 1, 1, 1], [0.5, 0.52, 0.56, 1]] select _off];
        _list lnbSetColor [[_row, 1], [[0.62, 0.64, 0.68, 1], [0.95, 0.35, 0.3, 1]] select _off];
    } forEach _rows;
} else {
    {
        _x params ["_name", "", "_genre", "", "_free"];
        private _info = _genre;
        if (_free) then {
            _info = trim format ["%1 %2", _genre, localize "STR_skw_ui_freeMark"];
        };
        private _row = _list lnbAddRow [_name, _info];
        _list lnbSetData [[_row, 0], str _forEachIndex];
        _list lnbSetTooltip [[_row, 0], _name];
        _list lnbSetColor [[_row, 1], [0.62, 0.64, 0.68, 1]];
    } forEach skw_ui_managerEntries;
};
_list lnbSetCurSelRow -1;
skw_ui_managerUpdating = false;
