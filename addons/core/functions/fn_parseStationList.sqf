/*
 * Reads a station list setting into station records (format: see skw_core_fnc_stationListEntries).
 *
 * Arguments:
 * 0: Setting text <STRING>
 * 1: Source: "global" or "personal" (also used as class prefix) <STRING>
 *
 * Return: Stations by class <HASHMAP>
 */
params [["_text", "", [""]], ["_source", "global", [""]]];

private _result = createHashMap;
{
    _x params ["_name", "_url", "_genre", "_logo", "_free"];
    private _class = format ["%1:%2", _source, toLower _name];
    // Backup addresses follow the main one, separated by commas.
    private _addresses = ((_url splitString ",") apply {trim _x}) select {_x != ""};
    private _station = [_class, _name, _addresses param [0, ""], _addresses select [1], _logo, _genre, "", _free, "true", [], false, _source] call skw_core_fnc_makeStation;
    if (count _station > 0) then {
        _result set [_class, _station];
    };
} forEach ([_text] call skw_core_fnc_stationListEntries);
_result
