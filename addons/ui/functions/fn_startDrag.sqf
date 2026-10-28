/*
 * Starts moving ("move") or resizing the radio window with the mouse: "width" (right edge),
 * "height" (bottom edge) or "corner" (both).
 *
 * Arguments:
 * 0: Mode <STRING>
 */
params [["_mode", "move", [""]]];

skw_ui_drag = [_mode, getMousePosition, +skw_ui_window];
if (skw_ui_dragPfh < 0) then {
    skw_ui_dragPfh = [{call skw_ui_fnc_onDrag}, 0] call CBA_fnc_addPerFrameHandler;
};
