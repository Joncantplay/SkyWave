/*
 * Returns whether a station is marked as free of copyrighted music. A stream URL counts
 * as free when a known station with the same URL is marked free.
 *
 * Arguments:
 * 0: Station: class name or stream URL <STRING>
 *
 * Return: <BOOL>
 */
params [["_station", "", [""]]];

if (_station == "") exitWith {true};

private _record = _station call skw_core_fnc_getStation;
if (count _record > 0) exitWith {_record get "copyrightFree"};

// Played by URL: compare the main address (backups may follow after a space).
private _mainUrl = (_station splitString " ") param [0, ""];
private _free = false;
{
    if ((_y get "copyrightFree") && {(_y get "url") == _mainUrl}) exitWith {
        _free = true;
    };
} forEach (call skw_core_fnc_allStations);
_free
