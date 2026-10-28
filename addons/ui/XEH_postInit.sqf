if (!hasInterface) exitWith {};

// The player unit may not exist yet when joining a running mission.
[{!isNull player}, {
    call skw_ui_fnc_addAction;
    // Actions are lost when the player respawns; the event handler itself stays.
    player addEventHandler ["Respawn", {call skw_ui_fnc_addAction}];
}] call CBA_fnc_waitUntilAndExecute;
