/*
 * Removes a station added with skw_core_fnc_addStation. Server only.
 *
 * Arguments:
 * 0: Class name <STRING>
 *
 * Return: true if a station was removed <BOOL>
 */
params [["_class", "", [""]]];

if (!isServer) exitWith {false};

skw_core_scriptStations params ["_revision", "_list"];
private _remaining = _list select {(_x select 0) != _class};
if (count _remaining == count _list) exitWith {false};
skw_core_scriptStations = [_revision + 1, _remaining];
publicVariable "skw_core_scriptStations";
true
