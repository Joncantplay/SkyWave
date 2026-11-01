/*
 * "Play URL" button: tunes the radio to the address in the text field.
 */
private _display = uiNamespace getVariable ["skw_ui_display", displayNull];
private _object = uiNamespace getVariable ["skw_ui_target", objNull];
if (isNull _object) exitWith {};

// Backup addresses may follow, separated by commas.
private _addresses = ((ctrlText (_display displayCtrl 1500) splitString ", ") apply {trim _x}) select {_x != ""};
private _invalid = _addresses findIf {
    private _lower = toLower _x;
    !((_lower select [0, 7]) == "http://" || {(_lower select [0, 8]) == "https://"})
};
if (_addresses isEqualTo [] || {_invalid >= 0}) exitWith {
    (localize "STR_skw_ui_invalidUrl") call skw_audio_fnc_hint;
};
[_object, _addresses joinString " "] call skw_core_fnc_setStation;
