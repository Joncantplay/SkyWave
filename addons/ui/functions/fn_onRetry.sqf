/*
 * "Reconnect now" button.
 */
private _object = uiNamespace getVariable ["skw_ui_target", objNull];
_object call skw_audio_fnc_retry;
