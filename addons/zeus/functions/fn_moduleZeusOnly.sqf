/*
 * Zeus module "Zeus control only": locks the radio of a vehicle or object so players can
 * no longer operate it, from inside or outside; only Zeus can. Placed again on the same
 * object, it unlocks the radio.
 *
 * Arguments:
 * 0: Module logic <OBJECT>
 */
params [["_logic", objNull, [objNull]]];

if (!local _logic) exitWith {};

private _object = attachedTo _logic;
deleteVehicle _logic;

if (isNull _object || {!(_object call skw_core_fnc_isRadioCapable)}) exitWith {
    "STR_skw_zeus_noRadio" call skw_zeus_fnc_hint;
};

private _locked = !(_object getVariable ["skw_core_zeusOnly", false]);
_object setVariable ["skw_core_zeusOnly", _locked, true];
(["STR_skw_zeus_unlocked", "STR_skw_zeus_locked"] select _locked) call skw_zeus_fnc_hint;
