/*
 * Shows a Zeus module confirmation to the Zeus who used the module.
 *
 * Arguments:
 * 0: Stringtable key <STRING>
 */
params [["_key", "", [""]]];

(localize _key) call skw_audio_fnc_hint;
