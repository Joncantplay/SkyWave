/*
 * Shows a short hint that disappears after four seconds, unless a newer one replaced it.
 *
 * Arguments:
 * 0: Message <STRING>
 */
params [["_message", "", [""]]];

hintSilent format ["SkyWave\n%1", _message];
skw_audio_hintToken = skw_audio_hintToken + 1;
[{
    if (skw_audio_hintToken == _this) then {
        hintSilent "";
    };
}, skw_audio_hintToken, 4] call CBA_fnc_waitAndExecute;
