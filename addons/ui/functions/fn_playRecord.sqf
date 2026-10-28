/*
 * Tunes a radio to a station from the station list. Personal stations exist only for this
 * player, so they are sent as URL with their name as label.
 *
 * Arguments:
 * 0: Radio object <OBJECT>
 * 1: Station record <HASHMAP>
 */
params [["_object", objNull, [objNull]], ["_record", createHashMap, [createHashMap]]];

if (isNull _object || {count _record == 0}) exitWith {};

private _station = _record get "class";
private _label = "";
if ((_record get "source") == "personal") then {
    _station = _record get "url";
    _label = _record get "name";
};

(_object call skw_core_fnc_getState) params ["_current"];
if (_station == _current) exitWith {};
[_object, _station, _label] call skw_core_fnc_setStation;
