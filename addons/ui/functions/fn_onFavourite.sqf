/*
 * Star button: adds the station playing now to this player's favourites, or removes it.
 * Favourites stand at the top of the station list and are kept in the profile.
 */
if (skw_ui_currentKey == "") exitWith {};

if (skw_ui_currentKey in skw_ui_favourites) then {
    skw_ui_favourites = skw_ui_favourites - [skw_ui_currentKey];
} else {
    skw_ui_favourites pushBack skw_ui_currentKey;
};
profileNamespace setVariable ["skw_ui_favourites", skw_ui_favourites];
saveProfileNamespace;
call skw_ui_fnc_fillStations;
call skw_ui_fnc_refresh;
