/*
 * Puts the radio window and the station manager back in the middle of the screen at their
 * normal size, e.g. after one got dragged somewhere awkward. Also applies to open windows.
 */
skw_ui_window = call skw_ui_fnc_defaultWindow;
skw_ui_managerWindow = [skw_ui_managerSize, [0, 0]] call skw_ui_fnc_defaultWindow;
profileNamespace setVariable ["skw_ui_window", skw_ui_window];
profileNamespace setVariable ["skw_ui_managerWindow", skw_ui_managerWindow];
saveProfileNamespace;
call skw_ui_fnc_layout;
call skw_ui_fnc_managerLayout;
