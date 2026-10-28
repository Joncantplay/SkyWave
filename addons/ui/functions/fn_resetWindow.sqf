/*
 * Puts the radio window back in the middle of the screen at its normal size, e.g. after it
 * got dragged somewhere awkward. Also applies to an open window.
 */
skw_ui_window = call skw_ui_fnc_defaultWindow;
profileNamespace setVariable ["skw_ui_window", skw_ui_window];
saveProfileNamespace;
call skw_ui_fnc_layout;
