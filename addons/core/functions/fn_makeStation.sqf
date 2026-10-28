/*
 * Builds a station record. All stations, from config, CBA settings or script, use this format.
 *
 * Arguments:
 * 0: Class name (unique id) <STRING>
 * 1: Display name <STRING>
 * 2: Stream URL <STRING>
 * 3: Fallback URLs, tried in order when the main URL fails <ARRAY of STRINGs> (default: [])
 * 4: Logo override: an image URL or web page (http/https), or a .paa path. Empty = find the logo
 *    on the station's website automatically <STRING> (default: "")
 * 5: Genre <STRING> (default: "")
 * 6: Description <STRING> (default: "")
 * 7: Free of copyrighted music (plays in Streamer Mode) <BOOL> (default: false)
 * 8: Condition: SQF code returning true when available; gets _object and _unit <STRING> (default: "true")
 * 9: Only on these object classes (isKindOf) <ARRAY of STRINGs> (default: [] = all)
 * 10: Hidden from the station list <BOOL> (default: false)
 * 11: Source: "builtin", "global", "personal" or "script" <STRING> (default: "script")
 *
 * Return: Station <HASHMAP>, empty if invalid
 */
params [
    ["_class", "", [""]],
    ["_name", "", [""]],
    ["_url", "", [""]],
    ["_fallbacks", [], [[]]],
    ["_logo", "", [""]],
    ["_genre", "", [""]],
    ["_description", "", [""]],
    ["_copyrightFree", false, [false]],
    ["_condition", "true", [""]],
    ["_vehicles", [], [[]]],
    ["_hidden", false, [false]],
    ["_source", "script", [""]]
];

if (_class == "" || {_url == ""}) exitWith {createHashMap};
if (_name == "") then {
    _name = _class;
};
if (trim _condition == "") then {
    _condition = "true";
};

// A web address is downloaded by the extension, anything else is a game texture path.
private _lowerLogo = toLower _logo;
private _logoIsUrl = (_lowerLogo select [0, 7]) == "http://" || {(_lowerLogo select [0, 8]) == "https://"};

createHashMapFromArray [
    ["class", _class],
    ["name", _name],
    ["url", _url],
    ["fallbacks", _fallbacks select {_x isEqualType "" && {_x != ""}}],
    ["image", ["", _logo] select (!_logoIsUrl)],
    ["logo", ["", _logo] select _logoIsUrl],
    ["genre", _genre],
    ["description", _description],
    ["copyrightFree", _copyrightFree],
    ["condition", compile ("params ['_object', '_unit']; " + _condition)],
    ["vehicles", _vehicles select {_x isEqualType ""}],
    ["hidden", _hidden],
    ["source", _source]
]
