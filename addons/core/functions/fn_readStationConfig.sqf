/*
 * Reads one CfgSkywaveStations class (from a mod's config.cpp) into a station record.
 *
 * Arguments:
 * 0: Station config class <CONFIG>
 *
 * Return: Station <HASHMAP>, empty if invalid
 */
params ["_config"];

private _class = configName _config;
private _url = getText (_config >> "url");
if (_url == "") exitWith {
    diag_log text format ["[SkyWave] Station %1 has no url and is ignored.", _class];
    createHashMap
};

private _condition = "true";
if (isText (_config >> "condition")) then {
    _condition = getText (_config >> "condition");
};

[
    _class,
    getText (_config >> "displayName"),
    _url,
    getArray (_config >> "fallbackUrls"),
    getText (_config >> "logo"),
    getText (_config >> "genre"),
    getText (_config >> "description"),
    getNumber (_config >> "copyrightFree") == 1,
    _condition,
    getArray (_config >> "vehicles"),
    getNumber (_config >> "hidden") == 1,
    // SkyWave's own stations can be switched off in the settings.
    ["builtin", "default"] select (getNumber (_config >> "skw_default") == 1)
] call skw_core_fnc_makeStation
