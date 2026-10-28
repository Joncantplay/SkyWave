/*
 * Rebuilds the local records of script-added stations when the server's list changed.
 */
skw_core_scriptStations params [["_revision", 0], ["_list", []]];
if (_revision == skw_core_scriptStationsRevision) exitWith {};

skw_core_scriptStationsRevision = _revision;
skw_core_scriptStationMap = createHashMap;
{
    private _station = _x call skw_core_fnc_makeStation;
    if (count _station > 0) then {
        skw_core_scriptStationMap set [_station get "class", _station];
    };
} forEach _list;
