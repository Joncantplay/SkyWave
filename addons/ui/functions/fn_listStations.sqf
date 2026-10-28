/*
 * Puts stations in the order of the radio window: filtered by the search text and the
 * "copyright-free only" box, sorted by the chosen column, favourites first. Personal
 * stations are left out where the server forbids custom URLs (they are played by URL).
 *
 * Arguments:
 * 0: Stations the player may choose (skw_core_fnc_getStations) <ARRAY of HASHMAPs>
 * 1: Search text, lower case <STRING> (default: "")
 *
 * Return: Stations <ARRAY of HASHMAPs>
 */
params [["_stations", [], [[]]], ["_search", "", [""]]];

private _urlAllowed = skw_core_allowCustomUrls || {!isNull getAssignedCuratorLogic player};
private _byGenre = (skw_ui_sort select 0) == "genre";
private _rows = [];
{
    private _name = toLower (_x get "name");
    private _genre = toLower (_x get "genre");
    if (
        ((_x get "source") != "personal" || _urlAllowed)
        && {_search == "" || {_name find _search >= 0} || {_genre find _search >= 0}}
        && {(_x get "copyrightFree") || {!skw_ui_freeOnly}}
    ) then {
        // Sort key first: genre sorts by genre, then name.
        _rows pushBack [[_name, _genre + "|" + _name] select _byGenre, _x];
    };
} forEach _stations;
_rows sort (skw_ui_sort select 1);

private _favourites = _rows select {((_x select 1) get "class") in skw_ui_favourites};
private _others = _rows select {!(((_x select 1) get "class") in skw_ui_favourites)};
(_favourites + _others) apply {_x select 1}
