/*
 * Mutes or unmutes one radio for this player only. Other players are not affected.
 *
 * Arguments:
 * 0: Object <OBJECT>
 * 1: Muted <BOOL>
 */
params [["_object", objNull, [objNull]], ["_muted", true, [true]]];

_object setVariable ["skw_audio_muted", _muted];
