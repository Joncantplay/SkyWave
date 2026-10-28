/*
 * Fills the station list of the open dialog (order: skw_ui_fnc_listStations).
 * Favourites get a star.
 */
#define STAR "\z\skw\addons\ui\data\star_ca.paa"
#define NO_PICTURE "#(argb,8,8,3)color(0,0,0,0)"

private _display = uiNamespace getVariable ["skw_ui_display", displayNull];
if (isNull _display) exitWith {};

private _search = toLower trim ctrlText (_display displayCtrl 1700);
private _list = _display displayCtrl 1100;
skw_ui_updating = true;
lnbClear _list;
{
    private _personal = (_x get "source") == "personal";
    private _label = _x get "genre";
    if (_personal && {_label == ""}) then {
        _label = localize "STR_skw_ui_myStation";
    };
    if (_x get "copyrightFree") then {
        _label = trim format ["%1 %2", _label, localize "STR_skw_ui_freeMark"];
    };
    private _row = _list lnbAddRow [_x get "name", _label];
    _list lnbSetData [[_row, 0], _x get "class"];
    // Personal stations get an orange genre. Leave the others alone, a custom colour is
    // unreadable on the selected row.
    if (_personal) then {
        _list lnbSetColor [[_row, 1], [0.96, 0.58, 0.16, 1]];
    };
    // Empty picture on the other rows keeps the names aligned.
    _list lnbSetPicture [[_row, 0], [NO_PICTURE, STAR] select ((_x get "class") in skw_ui_favourites)];
    _list lnbSetPictureColor [[_row, 0], [0.96, 0.58, 0.16, 1]];
    if ((_x get "description") != "") then {
        _list lnbSetTooltip [[_row, 0], _x get "description"];
    };
} forEach ([skw_ui_stations, _search] call skw_ui_fnc_listStations);
_list lnbSetCurSelRow -1;
skw_ui_updating = false;

(_display displayCtrl 1710) ctrlSetText localize "STR_skw_ui_station";
(_display displayCtrl 1711) ctrlSetText localize "STR_skw_ui_genre";
call skw_ui_fnc_placeSortArrow;
