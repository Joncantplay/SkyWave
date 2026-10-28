/*
 * Copies a track title to the clipboard (through the extension, since the game's own copy
 * command does not work in multiplayer). Without argument: the track playing now.
 *
 * Arguments:
 * 0: Title <STRING> (default: "" = current track)
 */
params [["_title", "", [""]]];

if (_title == "") then {
    private _object = uiNamespace getVariable ["skw_ui_target", objNull];
    private _id = _object getVariable ["skw_audio_id", ""];
    _title = (skw_audio_titles getOrDefault [_id, [""]]) select 0;
};
if (_title == "") exitWith {};

private _result = "skywave" callExtension ["clipboard", [_title]];
if ((_result select 1) == 0) then {
    (format [localize "STR_skw_ui_copied", _title]) call skw_audio_fnc_hint;
} else {
    (localize "STR_skw_ui_copyFailed") call skw_audio_fnc_hint;
};
