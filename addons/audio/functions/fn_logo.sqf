/*
 * Returns a station's logo as a data URL for the browser control, starting the download
 * on first use. Logos are kept for the rest of the game session.
 *
 * Arguments:
 * 0: Key (station class or URL) <STRING>
 * 1: Stream address list (see skw_core_fnc_stationUrl) <STRING>
 * 2: Logo URL override, "" to search the station website <STRING>
 *
 * Return: Data URL, or "" while loading or when no logo was found <STRING>
 */
params [["_key", "", [""]], ["_addresses", "", [""]], ["_logoUrl", "", [""]]];

if (!skw_audio_available || {_key == ""}) exitWith {""};

private _entry = skw_audio_logos getOrDefault [_key, []];
if (_entry isEqualTo []) exitWith {
    skw_audio_logos set [_key, ["loading", ""]];
    "skywave" callExtension ["logo", [_key, _addresses, _logoUrl]];
    ""
};
_entry select 1
