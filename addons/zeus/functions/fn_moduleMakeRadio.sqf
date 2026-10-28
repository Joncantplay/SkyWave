/*
 * Zeus module "Turn object into radio": gives any object a radio (or gives a vehicle its
 * radio back) and opens the radio window. Players operate such radios from close by
 * (vehicles also from inside).
 *
 * Arguments:
 * 0: Module logic <OBJECT>
 */
params [["_logic", objNull, [objNull]]];

if (!local _logic) exitWith {};

private _object = attachedTo _logic;
deleteVehicle _logic;

if (isNull _object) exitWith {
    "STR_skw_zeus_noObject" call skw_zeus_fnc_hint;
};

if !(_object call skw_core_fnc_isRadioCapable) then {
    [_object, true] call skw_core_fnc_setRadioEnabled;
    "STR_skw_zeus_radioAdded" call skw_zeus_fnc_hint;
};

// Open once the Zeus interface has finished handling the placement.
[{_this call skw_ui_fnc_open}, _object] call CBA_fnc_execNextFrame;
