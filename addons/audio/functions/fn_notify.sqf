/*
 * Shows a status message where the player chose in the settings: off, hint or chat.
 * Only for the radio the player is in, standing next to or has open, anything else would
 * spam the screen. Zeus radios can be switched off separately.
 *
 * Arguments:
 * 0: Message <STRING>
 * 1: Source id of the radio it is about <STRING> (default: "")
 */
params [["_message", "", [""]], ["_id", "", [""]]];

private _object = (skw_audio_playing getOrDefault [_id, [objNull, ""]]) select 0;
if (!isNull _object) then {
    private _unit = [focusOn, player] select (isNull focusOn);
    private _own = vehicle _unit == _object
        || {_object == (uiNamespace getVariable ["skw_ui_target", objNull])}
        || {(_unit distance _object) < 5};
    if (!_own || {!skw_audio_zeusMessages && {_object getVariable ["skw_core_radioEnabled", false]}}) then {
        _message = "";
    };
};
if (_message == "") exitWith {};

switch (skw_audio_statusDisplay) do {
    case 1: {
        _message call skw_audio_fnc_hint;
    };
    case 2: {
        systemChat format ["SkyWave: %1", _message];
    };
};
