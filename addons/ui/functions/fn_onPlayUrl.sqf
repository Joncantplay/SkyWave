/*
 * "Play URL" button: tunes the radio to the address in the text field.
 */
private _display = uiNamespace getVariable ["skw_ui_display", displayNull];
private _object = uiNamespace getVariable ["skw_ui_target", objNull];
if (isNull _object) exitWith {};

private _url = trim ctrlText (_display displayCtrl 1500);
private _lower = toLower _url;
if !((_lower select [0, 7]) == "http://" || {(_lower select [0, 8]) == "https://"}) exitWith {
    (localize "STR_skw_ui_invalidUrl") call skw_audio_fnc_hint;
};
[_object, _url] call skw_core_fnc_setStation;
