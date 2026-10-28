/*
 * Returns whether a station may be played on an object by a unit: it must exist,
 * match the station's object classes and pass its condition.
 *
 * Arguments:
 * 0: Station class or record <STRING or HASHMAP>
 * 1: Radio object <OBJECT>
 * 2: Unit <OBJECT>
 *
 * Return: <BOOL>
 */
params [["_station", "", ["", createHashMap]], ["_object", objNull, [objNull]], ["_unit", objNull, [objNull]]];

if (_station isEqualType "") then {
    _station = _station call skw_core_fnc_getStation;
};
if (count _station == 0) exitWith {false};

private _classes = _station get "vehicles";
if (count _classes > 0 && {(_classes findIf {_object isKindOf _x}) < 0}) exitWith {false};

([_object, _unit] call (_station get "condition")) isEqualTo true
