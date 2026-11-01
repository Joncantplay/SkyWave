/*
 * Makes sure the player has the "SkyWave radio" entry in the action menu. It shows while
 * the player can operate a radio: the vehicle they sit in, or a vehicle or placed radio they
 * look at.
 *
 * Called regularly: missions with their own revive or respawn (Antistasi, for example) remove
 * all actions or move the player into another unit, so the entry is added again when it is
 * missing. Does nothing while it is in place.
 */
private _unit = player;
if (isNull _unit) exitWith {};

if (
    _unit isEqualTo (missionNamespace getVariable ["skw_ui_actionUnit", objNull])
    && {(missionNamespace getVariable ["skw_ui_actionId", -1]) in actionIDs _unit}
) exitWith {};

// Player moved into another unit: take the entry off the old one, else it would show when
// looking at that unit.
private _oldUnit = missionNamespace getVariable ["skw_ui_actionUnit", objNull];
if (!isNull _oldUnit && {_oldUnit isNotEqualTo _unit}) then {
    _oldUnit removeAction (missionNamespace getVariable ["skw_ui_actionId", -1]);
};

skw_ui_actionUnit = _unit;
skw_ui_actionId = _unit addAction [
    localize "STR_skw_ui_action",
    {call skw_ui_fnc_openForPlayer},
    nil,
    0.5,
    false,
    true,
    "",
    "skw_ui_scrollAction && {call skw_ui_fnc_canOpen}"
];
