/*
 * Returns whether an object can carry a SkyWave radio.
 * Land vehicles, aircraft and boats qualify, static weapons do not.
 * Objects whose config sets `skw_radio = 1;` qualify, and any object Zeus turned into
 * a radio. Zeus can also remove the radio from any object (see skw_core_fnc_setRadioEnabled).
 *
 * Arguments:
 * 0: Object <OBJECT>
 *
 * Return: <BOOL>
 */
params [["_object", objNull, [objNull]]];

if (isNull _object) exitWith {false};
if (_object getVariable ["skw_core_radioDisabled", false]) exitWith {false};
if (_object getVariable ["skw_core_radioEnabled", false]) exitWith {true};
if (getNumber (configOf _object >> "skw_radio") == 1) exitWith {true};
if (_object isKindOf "StaticWeapon") exitWith {false};

(_object isKindOf "LandVehicle") || {_object isKindOf "Air"} || {_object isKindOf "Ship"}
