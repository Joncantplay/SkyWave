/*
 * Returns the name of the station a radio plays, as shown in the radio window.
 *
 * Arguments:
 * 0: Source id <STRING>
 *
 * Return: <STRING>
 */
params [["_id", "", [""]]];

private _object = (skw_audio_playing getOrDefault [_id, [objNull, ""]]) select 0;
(_object call skw_core_fnc_getState) params ["_station", "", "_label"];
private _record = _station call skw_core_fnc_getStation;
switch (true) do {
    case (_label != ""): {_label};
    case (count _record > 0): {_record get "name"};
    default {(skw_audio_titles getOrDefault [_id, ["", "Radio"]]) select 1};
};
