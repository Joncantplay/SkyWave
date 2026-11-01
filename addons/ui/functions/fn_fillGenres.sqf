/*
 * Fills the genre box of the radio window with the genres of the stations on offer.
 */
private _display = uiNamespace getVariable ["skw_ui_display", displayNull];
if (isNull _display) exitWith {};

private _genres = [];
{
    private _genre = _x get "genre";
    if (_genre != "") then {
        _genres pushBackUnique _genre;
    };
} forEach skw_ui_stations;
_genres sort true;
// A genre no station has any more would leave the list empty.
if !(skw_ui_genreFilter in _genres) then {
    skw_ui_genreFilter = "";
};

private _combo = _display displayCtrl 1704;
skw_ui_updating = true;
lbClear _combo;
_combo lbSetData [_combo lbAdd localize "STR_skw_ui_allGenres", ""];
{
    private _row = _combo lbAdd _x;
    _combo lbSetData [_row, _x];
    if (_x == skw_ui_genreFilter) then {
        _combo lbSetCurSel _row;
    };
} forEach _genres;
if (skw_ui_genreFilter == "") then {
    _combo lbSetCurSel 0;
};
skw_ui_updating = false;
