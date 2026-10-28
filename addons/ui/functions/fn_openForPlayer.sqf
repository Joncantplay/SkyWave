/*
 * Opens the radio the player sits in or looks at, if allowed.
 */
private _target = call skw_ui_fnc_actionTarget;
if (isNull _target) exitWith {};
_target call skw_ui_fnc_open;
