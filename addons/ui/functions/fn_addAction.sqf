/*
 * Adds the "SkyWave radio" entry to the player's action menu. It shows while the player
 * can operate a radio: the vehicle they sit in, or a vehicle or placed radio they look at.
 */
player addAction [
    localize "STR_skw_ui_action",
    {call skw_ui_fnc_openForPlayer},
    nil,
    0.5,
    false,
    true,
    "",
    "skw_ui_scrollAction && {call skw_ui_fnc_canOpen}"
];
