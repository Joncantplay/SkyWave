// Objects with a radio that is switched on. The server owns this list and broadcasts it;
// a joining player may already have received it before this runs, so keep that value.
if (isNil "skw_core_radios") then {
    skw_core_radios = [];
};

// Built-in stations from CfgSkywaveStations in mod configs.
skw_core_configStations = createHashMap;
{
    private _station = [_x] call skw_core_fnc_readStationConfig;
    if (count _station > 0) then {
        skw_core_configStations set [configName _x, _station];
    };
} forEach ("true" configClasses (configFile >> "CfgSkywaveStations"));

// Stations added by script: [revision, [station arguments, ...]], broadcast by the server.
if (isNil "skw_core_scriptStations") then {
    skw_core_scriptStations = [0, []];
};
skw_core_scriptStationsRevision = -1;
skw_core_scriptStationMap = createHashMap;
skw_core_stationIndexSources = [];
skw_core_stationIndex = createHashMap;

[
    "skw_core_allowCustomUrls", "CHECKBOX",
    [localize "STR_skw_core_allowCustomUrls", localize "STR_skw_core_allowCustomUrls_tooltip"],
    ["SkyWave", localize "STR_skw_core_category_server"],
    true,
    1
] call CBA_fnc_addSetting;

[
    "skw_core_allowCustomImages", "CHECKBOX",
    [localize "STR_skw_core_allowCustomImages", localize "STR_skw_core_allowCustomImages_tooltip"],
    ["SkyWave", localize "STR_skw_core_category_server"],
    true,
    1
] call CBA_fnc_addSetting;

[
    "skw_core_allowOutside", "CHECKBOX",
    [localize "STR_skw_core_allowOutside", localize "STR_skw_core_allowOutside_tooltip"],
    ["SkyWave", localize "STR_skw_core_category_server"],
    false,
    1
] call CBA_fnc_addSetting;

[
    "skw_core_driverCommanderOnly", "CHECKBOX",
    [localize "STR_skw_core_driverCommanderOnly", localize "STR_skw_core_driverCommanderOnly_tooltip"],
    ["SkyWave", localize "STR_skw_core_category_server"],
    false,
    1
] call CBA_fnc_addSetting;

[
    "skw_core_engineOnly", "CHECKBOX",
    [localize "STR_skw_core_engineOnly", localize "STR_skw_core_engineOnly_tooltip"],
    ["SkyWave", localize "STR_skw_core_category_server"],
    false,
    1
] call CBA_fnc_addSetting;

[
    "skw_core_copyrightFreeOnly", "CHECKBOX",
    [localize "STR_skw_core_copyrightFreeOnly", localize "STR_skw_core_copyrightFreeOnly_tooltip"],
    ["SkyWave", localize "STR_skw_core_category_server"],
    false,
    1
] call CBA_fnc_addSetting;

[
    "skw_core_defaultStations", "CHECKBOX",
    [localize "STR_skw_core_defaultStations", localize "STR_skw_core_defaultStations_tooltip"],
    ["SkyWave", localize "STR_skw_core_category_stations"],
    true,
    1
] call CBA_fnc_addSetting;

[
    "skw_core_serverStations", "EDITBOX",
    [localize "STR_skw_core_serverStations", localize "STR_skw_core_stationList_tooltip"],
    ["SkyWave", localize "STR_skw_core_category_stations"],
    "",
    1
] call CBA_fnc_addSetting;

[
    "skw_core_personalStations", "EDITBOX",
    [localize "STR_skw_core_personalStations", localize "STR_skw_core_stationList_tooltip"],
    ["SkyWave", localize "STR_skw_core_category_stations"],
    "",
    0
] call CBA_fnc_addSetting;
