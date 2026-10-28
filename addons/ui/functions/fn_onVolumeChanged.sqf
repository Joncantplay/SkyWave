/*
 * Volume slider moved: remembers the value; skw_ui_fnc_refresh sends it.
 *
 * Arguments:
 * 0: Slider <CONTROL>
 * 1: Position 0..100 <NUMBER>
 */
params ["", "_position"];

skw_ui_volumeTouched = diag_tickTime;
skw_ui_pendingVolume = (_position / 100) max 0 min 1;
