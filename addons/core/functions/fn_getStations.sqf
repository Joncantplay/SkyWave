/*
 * Returns the stations a unit may choose for a radio object, sorted by name.
 * Hidden stations are left out, and so is everything not copyright-free when the server
 * allows only copyright-free stations.
 *
 * Arguments:
 * 0: Radio object <OBJECT>
 * 1: Unit <OBJECT>
 *
 * Return: Stations <ARRAY of HASHMAPs>
 */
params [["_object", objNull, [objNull]], ["_unit", objNull, [objNull]]];

private _list = [];
{
    if (
        !(_y get "hidden")
        && {!skw_core_copyrightFreeOnly || {_y get "copyrightFree"}}
        && {[_y, _object, _unit] call skw_core_fnc_isStationAvailable}
    ) then {
        _list pushBack [toLower (_y get "name"), _x, _y];
    };
} forEach (call skw_core_fnc_allStations);

_list sort true;
_list apply {_x select 2}
