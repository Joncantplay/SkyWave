// ACE interaction menu entries for SkyWave radios.
if (!hasInterface) exitWith {};

#define ICON "\A3\ui_f\data\gui\Rsc\RscDisplayArsenal\radio_ca.paa"

private _condition = {(_target call skw_core_fnc_isRadioCapable) && {[_player, _target] call skw_core_fnc_canControl}};
private _action = [
    "skw_radio",
    localize "STR_skw_ui_action",
    ICON,
    {_target call skw_ui_fnc_open},
    _condition
] call ace_interact_menu_fnc_createAction;

{
    // Sitting inside: ACE shows the vehicle's self-actions with the interaction key, next to
    // "Change seat".
    [_x, 1, ["ACE_SelfActions"], _action, true] call ace_interact_menu_fnc_addActionToClass;
    // Standing outside: the vehicle's interaction menu.
    [_x, 0, ["ACE_MainActions"], _action, true] call ace_interact_menu_fnc_addActionToClass;
} forEach ["LandVehicle", "Air", "Ship"];
["skw_radio", 0, ["ACE_MainActions"], _action, true] call ace_interact_menu_fnc_addActionToClass;

// Objects Zeus turns into a radio have no class entry: add the action to the object itself.
skw_compat_ace_objectAction = _action;
["skw_core_radioEnabled", {
    params [["_object", objNull]];
    if (isNull _object || {_object getVariable ["skw_compat_ace_hasAction", false]}) exitWith {};
    _object setVariable ["skw_compat_ace_hasAction", true];
    [_object, 0, [], skw_compat_ace_objectAction] call ace_interact_menu_fnc_addActionToObject;
}] call CBA_fnc_addEventHandler;
