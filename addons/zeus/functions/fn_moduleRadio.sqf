/*
 * Zeus module "SkyWave radio": opens the radio window of the vehicle or radio the module
 * was placed on. Runs on every machine; only Zeus' own machine (where the module is
 * local) acts.
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

// Open once the Zeus interface has finished handling the placement.
[{_this call skw_ui_fnc_open}, _object] call CBA_fnc_execNextFrame;
