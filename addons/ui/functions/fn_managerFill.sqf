/*
 * Fills the station manager's list from the entries being edited.
 */
private _display = uiNamespace getVariable ["skw_ui_managerDisplay", displayNull];
if (isNull _display) exitWith {};

private _list = _display displayCtrl 2100;
skw_ui_managerUpdating = true;
lnbClear _list;
{
    _x params ["_name", "", "_genre", "", "_free"];
    private _info = _genre;
    if (_free) then {
        _info = trim format ["%1 %2", _genre, localize "STR_skw_ui_freeMark"];
    };
    private _row = _list lnbAddRow [_name, _info];
    _list lnbSetData [[_row, 0], str _forEachIndex];
    _list lnbSetColor [[_row, 1], [0.62, 0.64, 0.68, 1]];
} forEach skw_ui_managerEntries;
_list lnbSetCurSelRow -1;
skw_ui_managerUpdating = false;
