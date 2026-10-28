/*
 * "Mute for me" button: toggles this radio for the local player only.
 */
private _object = uiNamespace getVariable ["skw_ui_target", objNull];
if (isNull _object) exitWith {};

[_object, !(_object getVariable ["skw_audio_muted", false])] call skw_audio_fnc_setMuted;
call skw_ui_fnc_refresh;
