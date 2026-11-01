skw_ui_stations = [];
skw_ui_updating = false;
skw_ui_pendingVolume = -1;
skw_ui_volumeTouched = -10;
skw_ui_pfh = -1;
skw_ui_logoShown = "";
// Favourite stations of this player (station classes), kept in the profile.
skw_ui_favourites = profileNamespace getVariable ["skw_ui_favourites", []];
skw_ui_currentKey = "";
// The list area shows the last tracks instead of the stations.
skw_ui_historyMode = false;
skw_ui_historyShown = [];
skw_ui_managerList = "personal";
skw_ui_managerEntries = [];
skw_ui_managerIndex = -1;
skw_ui_managerClass = "";
skw_ui_managerEditable = false;
skw_ui_managerUpdating = false;

// Radio window: [x, y, extra width, extra height] (x/y = top left corner in screen units,
// extras in grid cells beyond the smallest size), remembered per player. The size of text
// and controls comes from the setting skw_ui_scale.
skw_ui_window = [];
skw_ui_layoutScale = 1;
skw_ui_drag = [];
// Station manager window, same format as skw_ui_window.
skw_ui_managerWindow = [];
skw_ui_dragPfh = -1;
skw_ui_sort = ["name", true];
skw_ui_freeOnly = false;
// Genre shown in the station list, "" = all.
skw_ui_genreFilter = "";

// Window layout in grid cells at scale 1 and the smallest size (30 x 19.8 cells):
// [idc, x, y, w, h, font height (0 = none), stretch].
// Stretch [x, y, w, h] says how much of the extra width/height each value takes on:
// e.g. [0, 0, 1, 1] grows with the window, [1, 1, 0, 0] stays in the bottom right corner.
// A new window gets skw_ui_windowExtra, so the list shows a few more rows than at minimum.
skw_ui_windowSize = [30, 19.8];
skw_ui_windowExtra = [0, 2.5];
skw_ui_layoutTable = [
    [1001, 0, 0, 30, 19.8, 0, [0, 0, 1, 1]],
    [1010, 0, 0, 30, 1.2, 0, [0, 0, 1, 0]],
    [1011, 0.3, 0.15, 0.9, 0.9, 0, [0, 0, 0, 0]],
    [1000, 1.35, 0, 20.7, 1.2, 0.85, [0, 0, 1, 0]],
    [1012, 0, 1.2, 30, 0.1, 0, [0, 0, 1, 0]],
    [1017, 0, 0, 22.1, 1.2, 0, [0, 0, 1, 0]],
    [1602, 22.3, 0.15, 6.0, 0.9, 0.75, [1, 0, 0, 0]],
    [1601, 28.5, 0.15, 1.3, 0.9, 0.8, [1, 0, 0, 0]],
    [1013, 0.5, 1.65, 29, 4.6, 0, [0, 0, 1, 0]],
    [1014, 0.8, 1.95, 4.0, 4.0, 0, [0, 0, 0, 0]],
    [1205, 1.0, 2.15, 3.6, 3.6, 0, [0, 0, 0, 0]],
    [1203, 1.0, 2.15, 3.6, 3.6, 0, [0, 0, 0, 0]],
    [1200, 5.3, 1.95, 22.4, 1.4, 1.2, [0, 0, 1, 0]],
    [1206, 28.1, 2.15, 0.9, 0.9, 0, [1, 0, 0, 0]],
    [1207, 27.9, 1.95, 1.3, 1.3, 0, [1, 0, 0, 0]],
    [1201, 5.3, 3.4, 23.9, 1.2, 0.9, [0, 0, 1, 0]],
    [1204, 5.4, 5.15, 0.5, 0.5, 0, [0, 0, 0, 0]],
    [1202, 6.1, 4.85, 18.4, 1.1, 0.85, [0, 0, 1, 0]],
    [1401, 24.7, 4.85, 4.5, 1.1, 0.8, [1, 0, 0, 0]],
    [1400, 0.5, 6.7, 9.4, 1.3, 0.9, [0, 0, 1/3, 0]],
    [1403, 10.3, 6.7, 9.4, 1.3, 0.9, [1/3, 0, 1/3, 0]],
    [1402, 20.1, 6.7, 9.4, 1.3, 0.9, [2/3, 0, 1/3, 0]],
    [1301, 0.5, 8.4, 4.0, 1, 0.85, [0, 0, 0, 0]],
    [1300, 4.6, 8.4, 18.0, 1, 0, [0, 0, 1, 0]],
    [1304, 22.9, 8.4, 1, 1, 0, [1, 0, 0, 0]],
    [1305, 24.0, 8.4, 5.5, 1, 0.85, [1, 0, 0, 0]],
    [1015, 0.5, 9.9, 3.6, 1, 0.85, [0, 0, 0, 0]],
    [1700, 4.6, 9.9, 12.4, 1, 0.85, [0, 0, 1, 0]],
    [1704, 17.3, 9.9, 5.3, 1, 0.85, [1, 0, 0, 0]],
    [1702, 22.9, 9.9, 1, 1, 0, [1, 0, 0, 0]],
    [1703, 24.0, 9.9, 5.5, 1, 0.85, [1, 0, 0, 0]],
    [1710, 0.5, 11.2, 17.5, 0.9, 0.8, [0, 0, 0.6, 0]],
    [1711, 18.4, 11.2, 5.0, 0.9, 0.8, [0.6, 0, 0, 0]],
    [1714, 21.5, 11.2, 8.0, 0.9, 0.8, [1, 0, 0, 0]],
    [1100, 0.5, 12.1, 29, 4.5, 0.85, [0, 0, 1, 1]],
    [1105, 0.5, 12.1, 29, 4.5, 0.85, [0, 0, 1, 1]],
    [1502, 0.5, 17.0, 7.5, 1, 0.85, [0, 1, 0, 0]],
    [1500, 8.1, 17.0, 14.4, 1, 0.85, [0, 1, 1, 0]],
    [1501, 22.7, 17.0, 6.8, 1, 0.85, [1, 1, 0, 0]],
    [1507, 0.5, 18.3, 27.8, 1.1, 0.9, [0, 1, 1, 0]],
    [1622, 29.55, 1.3, 0.45, 17.0, 0, [1, 0, 0, 1]],
    [1623, 0.5, 19.45, 27.8, 0.35, 0, [0, 1, 1, 0]],
    [1621, 28.65, 18.55, 1.1, 1.1, 0, [1, 1, 0, 0]],
    [1620, 28.4, 18.3, 1.6, 1.5, 0, [1, 1, 0, 0]]
];

// Station manager layout, same format as the radio window's (30 x 20.6 cells at least).
skw_ui_managerSize = [30, 20.6];
skw_ui_managerTable = [
    [2010, 0, 0, 30, 20.6, 0, [0, 0, 1, 1]],
    [2000, 0, 0, 30, 1.4, 1.0, [0, 0, 1, 0]],
    [2017, 0, 0, 28.6, 1.4, 0, [0, 0, 1, 0]],
    [2009, 28.6, 0, 1.4, 1.4, 0.9, [1, 0, 0, 0]],
    [2011, 0, 1.4, 30, 0.12, 0, [0, 0, 1, 0]],
    [2001, 0.5, 2, 9.5, 1.2, 0.9, [0, 0, 1/3, 0]],
    [2002, 10.1, 2, 9.5, 1.2, 0.9, [1/3, 0, 1/3, 0]],
    [2004, 19.7, 2, 9.8, 1.2, 0.9, [2/3, 0, 1/3, 0]],
    [2003, 0.5, 3.4, 29, 1, 0.85, [0, 0, 1, 0]],
    [2100, 0.5, 4.8, 13, 14.8, 0.85, [0, 0, 0.4, 1]],
    [2012, 14, 4.8, 15.5, 0.9, 0.85, [0.4, 0, 0.6, 0]],
    [2201, 14, 5.7, 15.5, 1, 0.85, [0.4, 0, 0.6, 0]],
    [2013, 14, 6.9, 15.5, 0.9, 0.85, [0.4, 0, 0.6, 0]],
    [2202, 14, 7.8, 15.5, 1, 0.85, [0.4, 0, 0.6, 0]],
    [2014, 14, 9.0, 15.5, 0.9, 0.85, [0.4, 0, 0.6, 0]],
    [2203, 14, 9.9, 15.5, 1, 0.85, [0.4, 0, 0.6, 0]],
    [2015, 14, 11.1, 15.5, 0.9, 0.85, [0.4, 0, 0.6, 0]],
    [2204, 14, 12.0, 15.5, 1, 0.85, [0.4, 0, 0.6, 0]],
    [2205, 14, 13.3, 1, 1, 0, [0.4, 0, 0, 0]],
    [2016, 15.2, 13.3, 14.3, 1, 0.85, [0.4, 0, 0.6, 0]],
    [2301, 14, 14.9, 5, 1.2, 0.9, [0.4, 0, 0.2, 0]],
    [2302, 19.25, 14.9, 5, 1.2, 0.9, [0.6, 0, 0.2, 0]],
    [2303, 24.5, 14.9, 5, 1.2, 0.9, [0.8, 0, 0.2, 0]],
    [2306, 14, 14.9, 15.5, 1.2, 0.9, [0.4, 0, 0.6, 0]],
    [2304, 14, 16.4, 15.5, 1.9, 0.8, [0.4, 0, 0.6, 0]],
    [2305, 24.5, 18.4, 5, 1.2, 0.9, [1, 1, 0, 0]],
    [2018, 29.55, 1.6, 0.45, 17.9, 0, [1, 0, 0, 1]],
    [2019, 0.5, 20.25, 28.0, 0.35, 0, [0, 1, 1, 0]],
    [2021, 29.05, 19.7, 0.85, 0.85, 0, [1, 1, 0, 0]],
    [2020, 28.7, 19.6, 1.3, 1.0, 0, [1, 1, 0, 0]]
];

// Shows an HTML document in a browser control. Compiled from text because the build
// tool's SQF checker does not know this Arma 3 2.20 command yet.
skw_ui_showHtml = compile "params ['_control', '_html']; _control ctrlWebBrowserAction ['OpenDataAsURL', _html]";

[
    "skw_ui_scrollAction", "CHECKBOX",
    [localize "STR_skw_ui_scrollAction", localize "STR_skw_ui_scrollAction_tooltip"],
    ["SkyWave", localize "STR_skw_ui_category"],
    true,
    0
] call CBA_fnc_addSetting;

[
    "skw_ui_scale", "LIST",
    [localize "STR_skw_ui_scale", localize "STR_skw_ui_scale_tooltip"],
    ["SkyWave", localize "STR_skw_ui_category"],
    [[0.8, 0.9, 1, 1.1, 1.25, 1.4], ["80 %", "90 %", "100 %", "110 %", "125 %", "140 %"], 2],
    0,
    {call skw_ui_fnc_layout}
] call CBA_fnc_addSetting;

// Works like a button: ticking it resets the window, then it unticks itself.
[
    "skw_ui_resetWindow", "CHECKBOX",
    [localize "STR_skw_ui_resetWindow", localize "STR_skw_ui_resetWindow_tooltip"],
    ["SkyWave", localize "STR_skw_ui_category"],
    false,
    0,
    {
        params ["_value"];
        if (_value) then {
            call skw_ui_fnc_resetWindow;
            [{["skw_ui_resetWindow", false, "client"] call skw_core_fnc_storeSetting}] call CBA_fnc_execNextFrame;
        };
    }
] call CBA_fnc_addSetting;

if (!hasInterface) exitWith {};

[
    "SkyWave", "skw_ui_open",
    [localize "STR_skw_ui_keybind", localize "STR_skw_ui_keybind"],
    {call skw_ui_fnc_openForPlayer; true},
    {},
    [0x13, [true, true, false]] // Shift + Ctrl + R
] call CBA_fnc_addKeybind;

// Operate the radio without opening the window; no keys by default, to avoid clashes.
{
    _x params ["_name", "_title", "_code"];
    [
        "SkyWave", _name,
        [localize _title, localize "STR_skw_ui_keyRadioHint"],
        _code,
        {}
    ] call CBA_fnc_addKeybind;
} forEach [
    ["skw_ui_volumeUp", "STR_skw_ui_keyVolumeUp", {0.1 call skw_ui_fnc_keyVolume}],
    ["skw_ui_volumeDown", "STR_skw_ui_keyVolumeDown", {-0.1 call skw_ui_fnc_keyVolume}],
    ["skw_ui_nextStation", "STR_skw_ui_keyNextStation", {1 call skw_ui_fnc_keyStation}],
    ["skw_ui_prevStation", "STR_skw_ui_keyPrevStation", {-1 call skw_ui_fnc_keyStation}]
];
