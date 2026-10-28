/*
 * Flips one of this player's on/off settings (Streamer Mode, mute all), stores it in the
 * profile and confirms with a hint. Does nothing if the server or mission enforces it.
 *
 * Arguments:
 * 0: Setting name <STRING>
 * 1: Stringtable key for the hint when switched on <STRING>
 * 2: Stringtable key for the hint when switched off <STRING>
 */
params ["_setting", "_onKey", "_offKey"];

if ((_setting call CBA_settings_fnc_priority) != "client") exitWith {
    (localize "STR_skw_audio_forced") call skw_audio_fnc_hint;
};

private _value = !(missionNamespace getVariable [_setting, false]);
[_setting, _value, "client"] call skw_core_fnc_storeSetting;
(localize ([_offKey, _onKey] select _value)) call skw_audio_fnc_hint;
