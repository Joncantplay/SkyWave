/*
 * Starts moving ("move") or resizing a window with the mouse: "width" (right edge),
 * "height" (bottom edge) or "corner" (both).
 *
 * Arguments:
 * 0: Mode <STRING>
 * 1: Window: "radio" or "manager" <STRING> (default: "radio")
 */
params [["_mode", "move", [""]], ["_target", "radio", [""]]];

skw_ui_drag = [_mode, getMousePosition, +([skw_ui_window, skw_ui_managerWindow] select (_target == "manager")), _target];
if (skw_ui_dragPfh < 0) then {
    skw_ui_dragPfh = [{call skw_ui_fnc_onDrag}, 0] call CBA_fnc_addPerFrameHandler;
};
