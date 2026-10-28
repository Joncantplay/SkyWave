/*
 * Zeus module "Remove radio": switches the radio of a vehicle or object off and removes
 * it, so nobody can use it anymore. "Turn object into radio" gives it back.
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

[_object, false] call skw_core_fnc_setRadioEnabled;
"STR_skw_zeus_radioRemoved" call skw_zeus_fnc_hint;
