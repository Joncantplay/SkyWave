/*
 * Returns the station record for a class.
 *
 * Arguments:
 * 0: Class name <STRING>
 *
 * Return: Station <HASHMAP>, empty if unknown
 */
params [["_class", "", [""]]];

if (_class == "") exitWith {createHashMap};
(call skw_core_fnc_allStations) getOrDefault [_class, createHashMap]
