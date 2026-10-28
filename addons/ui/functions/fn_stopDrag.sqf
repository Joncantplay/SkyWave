/*
 * Ends moving or resizing the radio window.
 */
skw_ui_drag = [];
if (skw_ui_dragPfh >= 0) then {
    [skw_ui_dragPfh] call CBA_fnc_removePerFrameHandler;
    skw_ui_dragPfh = -1;
};
