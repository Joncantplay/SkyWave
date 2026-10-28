/*
 * Resolves a station to the address list handed to the audio extension: the stream URL
 * followed by its fallback URLs, separated by spaces.
 *
 * Arguments:
 * 0: Station: class name or a direct http(s) URL <STRING>
 *
 * Return: Address list, or "" if the station is unknown <STRING>
 */
params [["_station", "", [""]]];

if (_station == "") exitWith {""};

private _lower = toLower _station;
if ((_lower select [0, 7]) == "http://" || {(_lower select [0, 8]) == "https://"}) exitWith {_station};

private _record = _station call skw_core_fnc_getStation;
if (count _record == 0) exitWith {""};

([_record get "url"] + (_record get "fallbacks")) joinString " "
