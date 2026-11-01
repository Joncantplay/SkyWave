/*
 * "For everyone" box next to the volume. Ticked: the radio's volume is shared and changes
 * by others apply. Unticked: this player keeps an own volume for this radio, starting at
 * the current one.
 *
 * Arguments:
 * 0: Check box <CONTROL>
 * 1: Checked (0 or 1) <NUMBER>
 */
params ["", "_checked"];

private _object = uiNamespace getVariable ["skw_ui_target", objNull];
if (isNull _object) exitWith {};

if (_checked == 1) then {
    _object setVariable ["skw_audio_ownVolume", -1];
} else {
    _object setVariable ["skw_audio_ownVolume", (_object call skw_core_fnc_getState) select 1];
};
skw_ui_volumeTouched = -10;
call skw_ui_fnc_refresh;
