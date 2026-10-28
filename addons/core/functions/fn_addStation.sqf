/*
 * Adds a station at runtime, for all players including those joining later.
 * A station with the same class replaces the previous one. Server only.
 *
 * Arguments: same as skw_core_fnc_makeStation
 *
 * Return: true if the station was added <BOOL>
 *
 * Example:
 * ["myMission_news", "Field News", "https://example.com/news.mp3"] call skw_core_fnc_addStation
 */
if (!isServer) exitWith {false};
if (count (_this call skw_core_fnc_makeStation) == 0) exitWith {false};

skw_core_scriptStations params ["_revision", "_list"];
private _class = _this select 0;
_list = _list select {(_x select 0) != _class};
_list pushBack +_this;
skw_core_scriptStations = [_revision + 1, _list];
publicVariable "skw_core_scriptStations";
true
