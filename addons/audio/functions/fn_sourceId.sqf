/*
 * Returns the extension's id for an object's radio, creating one on first use.
 * Ids are only meaningful on this machine.
 *
 * Arguments:
 * 0: Object <OBJECT>
 *
 * Return: <STRING>
 */
params ["_object"];

private _id = _object getVariable ["skw_audio_id", ""];
if (_id == "") then {
    skw_audio_nextId = skw_audio_nextId + 1;
    _id = format ["r%1", skw_audio_nextId];
    _object setVariable ["skw_audio_id", _id];
};
_id
