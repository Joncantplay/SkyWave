/*
 * Volume slider moved. Shared volume: remembered, skw_ui_fnc_refresh sends it.
 * Own volume: applies right away, only for this player.
 *
 * Arguments:
 * 0: Slider <CONTROL>
 * 1: Position 0..100 <NUMBER>
 */
params ["", "_position"];

private _object = uiNamespace getVariable ["skw_ui_target", objNull];
private _value = (_position / 100) max 0 min 1;
skw_ui_volumeTouched = diag_tickTime;
if ((_object getVariable ["skw_audio_ownVolume", -1]) >= 0) exitWith {
    _object setVariable ["skw_audio_ownVolume", _value];
};
skw_ui_pendingVolume = _value;
