skw_audio_available = false;
skw_audio_nextId = 0;
skw_audio_playing = createHashMap;     // source id -> [object, url]
skw_audio_status = createHashMap;      // source id -> [state, detail]
skw_audio_titles = createHashMap;      // source id -> [title, station name, cover URL]
skw_audio_history = createHashMap;     // source id -> last track titles, newest first
skw_audio_closedCache = createHashMap; // vehicle class -> closed cabin?
skw_audio_logos = createHashMap;       // station key -> [state, data URL]
skw_audio_hintToken = 0;

private _categoryAudio = ["SkyWave", localize "STR_skw_audio_category_audio"];

[
    "skw_audio_masterVolume", "SLIDER",
    [localize "STR_skw_audio_masterVolume", localize "STR_skw_audio_masterVolume_tooltip"],
    _categoryAudio,
    [0, 1, 0.8, 0, true],
    0
] call CBA_fnc_addSetting;

[
    "skw_audio_volumeBoost", "LIST",
    [localize "STR_skw_audio_volumeBoost", localize "STR_skw_audio_volumeBoost_tooltip"],
    _categoryAudio,
    [[1, 1.5, 2, 3, 4], ["1x", "1.5x", "2x", "3x", "4x"], 0],
    0
] call CBA_fnc_addSetting;

[
    "skw_audio_range", "SLIDER",
    [localize "STR_skw_audio_range", localize "STR_skw_audio_range_tooltip"],
    _categoryAudio,
    [30, 300, 120, 0],
    1
] call CBA_fnc_addSetting;

[
    "skw_audio_vehicle3D", "LIST",
    [localize "STR_skw_audio_vehicle3D", localize "STR_skw_audio_vehicle3D_tooltip"],
    _categoryAudio,
    [[0, 1, 2], [localize "STR_skw_audio_vehicleStereo", localize "STR_skw_audio_vehicleGround", localize "STR_skw_audio_vehicleAll"], 1],
    0
] call CBA_fnc_addSetting;

[
    "skw_audio_muteUnfocused", "CHECKBOX",
    [localize "STR_skw_audio_muteUnfocused", localize "STR_skw_audio_muteUnfocused_tooltip"],
    _categoryAudio,
    false,
    0
] call CBA_fnc_addSetting;

[
    "skw_audio_muteAll", "CHECKBOX",
    [localize "STR_skw_audio_muteAll", localize "STR_skw_audio_muteAll_tooltip"],
    _categoryAudio,
    false,
    0
] call CBA_fnc_addSetting;

[
    "skw_audio_streamerMode", "CHECKBOX",
    [localize "STR_skw_audio_streamerMode", localize "STR_skw_audio_streamerMode_tooltip"],
    ["SkyWave", localize "STR_skw_audio_category_streamer"],
    false,
    0
] call CBA_fnc_addSetting;

[
    "skw_audio_statusDisplay", "LIST",
    [localize "STR_skw_audio_statusDisplay", localize "STR_skw_audio_statusDisplay_tooltip"],
    ["SkyWave", localize "STR_skw_audio_category_status"],
    [[0, 1, 2], [localize "STR_skw_audio_statusOff", "Hint", "Chat"], 0],
    0
] call CBA_fnc_addSetting;

[
    "skw_audio_covers", "CHECKBOX",
    [localize "STR_skw_audio_covers", localize "STR_skw_audio_covers_tooltip"],
    ["SkyWave", localize "STR_skw_audio_category_status"],
    true,
    0
] call CBA_fnc_addSetting;

[
    "skw_audio_zeusMessages", "CHECKBOX",
    [localize "STR_skw_audio_zeusMessages", localize "STR_skw_audio_zeusMessages_tooltip"],
    ["SkyWave", localize "STR_skw_audio_category_status"],
    true,
    0
] call CBA_fnc_addSetting;

if (!hasInterface) exitWith {};

[
    "SkyWave", "skw_audio_toggleStreamer",
    [localize "STR_skw_audio_keyStreamer", localize "STR_skw_audio_streamerMode_tooltip"],
    {["skw_audio_streamerMode", "STR_skw_audio_streamerOn", "STR_skw_audio_streamerOff"] call skw_audio_fnc_toggleSetting; true},
    {}
] call CBA_fnc_addKeybind;

[
    "SkyWave", "skw_audio_toggleMuteAll",
    [localize "STR_skw_audio_keyMuteAll", localize "STR_skw_audio_muteAll_tooltip"],
    {["skw_audio_muteAll", "STR_skw_audio_muteAllOn", "STR_skw_audio_muteAllOff"] call skw_audio_fnc_toggleSetting; true},
    {}
] call CBA_fnc_addKeybind;
