if (!hasInterface) exitWith {};

// Adds the action menu entry and puts it back whenever it goes missing: after a respawn,
// after a mission removed all actions, or when the player is moved into another unit.
[{call skw_ui_fnc_addAction}, 2] call CBA_fnc_addPerFrameHandler;
