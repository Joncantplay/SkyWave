/*
 * Switches a radio off, for all players.
 *
 * Arguments:
 * 0: Object <OBJECT>
 */
params [["_object", objNull, [objNull]]];

[_object, ""] call skw_core_fnc_setStation;
