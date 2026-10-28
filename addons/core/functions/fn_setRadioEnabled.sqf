/*
 * Gives any object a radio, or removes the radio from any object (switching it off),
 * including vehicles that have one by default. The change reaches all players, including
 * those joining later.
 *
 * Arguments:
 * 0: Object <OBJECT>
 * 1: Radio on the object <BOOL>
 */
params [["_object", objNull, [objNull]], ["_enabled", true, [true]]];

if (isNull _object) exitWith {};

if (!_enabled) exitWith {
    _object call skw_core_fnc_turnOff;
    _object setVariable ["skw_core_radioEnabled", false, true];
    _object setVariable ["skw_core_radioDisabled", true, true];
};

// Vehicles with a radio by default only need the removal undone; other objects are marked,
// which also lets players operate them from outside.
_object setVariable ["skw_core_radioDisabled", false, true];
if !(_object call skw_core_fnc_isRadioCapable) then {
    _object setVariable ["skw_core_radioEnabled", true, true];
    // Lets add-ons (e.g. the ACE menu) attach to the object, also for players joining later.
    ["skw_core_radioEnabled", [_object], "skw_radioEnabled_" + (_object call BIS_fnc_netId)] call CBA_fnc_globalEventJIP;
};
