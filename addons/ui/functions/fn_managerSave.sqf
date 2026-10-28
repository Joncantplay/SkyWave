/*
 * Station manager "Save": stores the form as a new station or over the selected one.
 * A new station with the name of an existing one replaces it.
 */
private _display = uiNamespace getVariable ["skw_ui_managerDisplay", displayNull];
if (isNull _display || {!skw_ui_managerEditable}) exitWith {};

private _message = _display displayCtrl 2304;
private _name = trim ctrlText (_display displayCtrl 2201);
private _url = trim ctrlText (_display displayCtrl 2202);
private _entry = [
    _name,
    _url,
    trim ctrlText (_display displayCtrl 2203),
    trim ctrlText (_display displayCtrl 2204),
    cbChecked (_display displayCtrl 2205)
];

if (_name == "") exitWith {
    _message ctrlSetText localize "STR_skw_ui_needName";
};
private _lower = toLower _url;
if !((_lower select [0, 7]) == "http://" || {(_lower select [0, 8]) == "https://"}) exitWith {
    _message ctrlSetText localize "STR_skw_ui_invalidUrl";
};

private _index = skw_ui_managerIndex;
if (_index < 0) then {
    _index = skw_ui_managerEntries findIf {toLower (_x select 0) == toLower _name};
};
if (_index < 0) then {
    _index = skw_ui_managerEntries pushBack _entry;
} else {
    skw_ui_managerEntries set [_index, _entry];
};

call skw_ui_fnc_managerWrite;
call skw_ui_fnc_managerFill;
skw_ui_managerIndex = _index;
skw_ui_managerUpdating = true;
(_display displayCtrl 2100) lnbSetCurSelRow _index;
skw_ui_managerUpdating = false;
_message ctrlSetText format [localize "STR_skw_ui_saved", _name];
