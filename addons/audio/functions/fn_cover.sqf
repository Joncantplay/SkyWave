/*
 * Returns the album cover of a track as a data URL for the browser control, starting the
 * lookup on first use. Uses the cover the station sends, otherwise searches by
 * "Artist - Title" (MusicBrainz / Cover Art Archive). Covers are kept for the session.
 *
 * Arguments:
 * 0: Track title <STRING>
 * 1: Cover URL sent by the station, "" if none <STRING>
 *
 * Return: Data URL, or "" while loading, when nothing was found or covers are off <STRING>
 */
params [["_title", "", [""]], ["_coverUrl", "", [""]]];

if (!skw_audio_available || {!skw_audio_covers}) exitWith {""};
if (_coverUrl != "") exitWith {
    ["cover:" + _coverUrl, "", _coverUrl] call skw_audio_fnc_logo
};
if (_title == "") exitWith {""};

private _key = "cover:" + _title;
private _entry = skw_audio_logos getOrDefault [_key, []];
if (_entry isEqualTo []) exitWith {
    skw_audio_logos set [_key, ["loading", ""]];
    "skywave" callExtension ["cover", [_key, _title]];
    ""
};
_entry select 1
