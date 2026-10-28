/*
 * Returns every known station by class: built-in (mod config), the server-wide list
 * from the CBA settings, this player's personal list, and stations added by script.
 * Later sources win when classes collide. The result is cached; do not modify it.
 *
 * Return: Stations by class <HASHMAP>
 */
private _sources = [skw_core_serverStations, skw_core_personalStations, skw_core_scriptStations select 0, skw_core_defaultStations];
if (_sources isEqualTo skw_core_stationIndexSources) exitWith {skw_core_stationIndex};

call skw_core_fnc_syncScriptStations;
private _all = createHashMap;
{
    if (skw_core_defaultStations || {(_y get "source") != "default"}) then {
        _all set [_x, _y];
    };
} forEach skw_core_configStations;
{
    private _map = _x;
    {
        _all set [_x, _y];
    } forEach _map;
} forEach [
    [skw_core_serverStations, "global"] call skw_core_fnc_parseStationList,
    [skw_core_personalStations, "personal"] call skw_core_fnc_parseStationList,
    skw_core_scriptStationMap
];

skw_core_stationIndexSources = _sources;
skw_core_stationIndex = _all;
_all
