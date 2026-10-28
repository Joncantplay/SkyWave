/*
 * Returns the radio state of an object.
 *
 * Arguments:
 * 0: Object <OBJECT>
 *
 * Return: [station <STRING> ("" = off), volume <NUMBER> 0..1, label <STRING>]
 *         The label names a station played by URL (e.g. a player's personal station).
 */
params [["_object", objNull, [objNull]]];

(_object getVariable ["skw_core_state", []]) params [["_station", ""], ["_volume", 0.7], ["_label", ""]];
[_station, _volume, _label]
