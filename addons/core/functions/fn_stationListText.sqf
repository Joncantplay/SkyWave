/*
 * Turns station entries back into the text of a station list setting.
 * Characters that separate fields and stations ("|" and ";") are removed from the values.
 *
 * Arguments:
 * 0: [[name, url, genre, logo, copyrightFree], ...]
 *
 * Return: Setting text <STRING>
 */
params [["_entries", [], [[]]]];

private _clean = {trim ((_this splitString "|;") joinString " ")};
(_entries apply {
    _x params ["_name", "_url", ["_genre", ""], ["_logo", ""], ["_free", false]];
    private _fields = [_name call _clean, _url call _clean];
    if (_free) then {
        _fields pushBack "free";
    };
    if (trim _genre != "") then {
        _fields pushBack ("genre=" + (_genre call _clean));
    };
    if (trim _logo != "") then {
        _fields pushBack ("logo=" + (_logo call _clean));
    };
    _fields joinString " | "
}) joinString "; "
