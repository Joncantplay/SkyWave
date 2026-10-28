/*
 * Dialog closed: sends a volume change still waiting, remembers where the window was
 * and stops the refresh loop.
 */
call skw_ui_fnc_stopDrag;
if (skw_ui_window isEqualTypeArray [0, 0, 0, 0]) then {
    profileNamespace setVariable ["skw_ui_window", skw_ui_window];
    saveProfileNamespace;
};

private _object = uiNamespace getVariable ["skw_ui_target", objNull];
if (skw_ui_pendingVolume >= 0 && {!isNull _object}) then {
    [_object, skw_ui_pendingVolume] call skw_core_fnc_setVolume;
};
skw_ui_pendingVolume = -1;

if (skw_ui_pfh >= 0) then {
    [skw_ui_pfh] call CBA_fnc_removePerFrameHandler;
    skw_ui_pfh = -1;
};
uiNamespace setVariable ["skw_ui_display", displayNull];
uiNamespace setVariable ["skw_ui_target", objNull];
