/*
 * Reconnects an object's radio right away instead of waiting for the next automatic
 * attempt. Only affects this player.
 *
 * Arguments:
 * 0: Object <OBJECT>
 */
params [["_object", objNull, [objNull]]];

private _id = _object getVariable ["skw_audio_id", ""];
if !(_id in skw_audio_playing) exitWith {};

// The next update starts the radio again with a fresh connection.
"skywave" callExtension ["stop", [_id]];
skw_audio_playing deleteAt _id;
skw_audio_status deleteAt _id;
skw_audio_titles deleteAt _id;
