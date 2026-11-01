/*
 * Lets the player move and resize a window with the mouse: drag the title bar to move,
 * the right edge, the bottom edge or the corner to resize.
 *
 * Arguments:
 * 0: Display <DISPLAY>
 * 1: Handles: [[idc, mode], ...], mode "move", "width", "height" or "corner" <ARRAY>
 * 2: Window: "radio" or "manager" <STRING>
 */
params ["_display", "_handles", "_target"];

{
    _x params ["_idc", "_mode"];
    private _control = _display displayCtrl _idc;
    _control setVariable ["skw_ui_drag", [_mode, _target]];
    _control ctrlAddEventHandler ["MouseButtonDown", {
        params ["_control", "_button"];
        if (_button == 0) then {(_control getVariable "skw_ui_drag") call skw_ui_fnc_startDrag};
    }];
    _control ctrlAddEventHandler ["MouseButtonUp", {call skw_ui_fnc_stopDrag}];
} forEach _handles;
_display displayAddEventHandler ["MouseButtonUp", {call skw_ui_fnc_stopDrag}];
