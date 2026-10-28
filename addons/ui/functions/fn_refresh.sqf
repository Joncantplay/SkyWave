/*
 * Updates the open dialog from the radio's shared state and the local stream status.
 * Runs five times per second while the dialog is open.
 */
#define DEFAULT_PICTURE "\A3\ui_f\data\gui\Rsc\RscDisplayArsenal\radio_ca.paa"

private _display = uiNamespace getVariable ["skw_ui_display", displayNull];
if (isNull _display) exitWith {};

private _object = uiNamespace getVariable ["skw_ui_target", objNull];
if (
    isNull _object
    || {!alive _object}
    || {!(_object call skw_core_fnc_isRadioCapable)}
    || {!([player, _object] call skw_core_fnc_canControl)}
) exitWith {
    closeDialog 0;
};

// Volume changes from the slider are sent at most five times per second.
if (skw_ui_pendingVolume >= 0) then {
    [_object, skw_ui_pendingVolume] call skw_core_fnc_setVolume;
    skw_ui_pendingVolume = -1;
};

(_object call skw_core_fnc_getState) params ["_station", "_volume", "_label"];
private _muteReason = [_object, _station] call skw_audio_fnc_muteReason;
private _muted = _muteReason != "";
private _id = _object getVariable ["skw_audio_id", ""];
(skw_audio_status getOrDefault [_id, ["", ""]]) params ["_state", "_detail"];
(skw_audio_titles getOrDefault [_id, ["", "", ""]]) params ["_title", "_streamName", ["_coverUrl", ""]];

// A station played by URL may still be one of the listed (e.g. personal) stations.
private _record = _station call skw_core_fnc_getStation;
if (count _record == 0 && {_station != ""}) then {
    private _index = skw_ui_stations findIf {(_x get "url") == _station};
    if (_index >= 0) then {
        _record = skw_ui_stations select _index;
    };
};
private _known = count _record > 0;

// Now playing card.
private _name = switch (true) do {
    case (_station == ""): {localize "STR_skw_ui_off"};
    case (_label != ""): {_label};
    case (_known): {_record get "name"};
    case (_streamName != ""): {_streamName};
    default {localize "STR_skw_ui_customStream"};
};
(_display displayCtrl 1200) ctrlSetText _name;

// Picture: the album cover of the current track, else a configured .paa, else the
// downloaded station logo, else the default radio icon. The server can switch off
// downloaded pictures for streams played by URL.
private _paa = ["", _record getOrDefault ["image", ""]] select _known;
private _lowerStation = toLower _station;
private _pictures = skw_core_allowCustomImages
    || {!((_lowerStation select [0, 7]) == "http://" || {(_lowerStation select [0, 8]) == "https://"})};
private _logo = "";
private _logoKey = _station;
if (_station != "" && {!_muted} && {_pictures}) then {
    _logo = [_title, _coverUrl] call skw_audio_fnc_cover;
    _logoKey = format ["cover:%1|%2", _title, _coverUrl];
};
if (_logo == "" && {_station != ""} && {_paa == ""} && {_pictures}) then {
    private _logoUrl = ["", _record getOrDefault ["logo", ""]] select _known;
    _logo = [_station, _station call skw_core_fnc_stationUrl, _logoUrl] call skw_audio_fnc_logo;
    _logoKey = _station;
};
private _web = _display displayCtrl 1205;
if (_logo != "") then {
    if (skw_ui_logoShown != _logoKey) then {
        skw_ui_logoShown = _logoKey;
        [_web, "<html><body style='margin:0;background:transparent;overflow:hidden'><img src='" + _logo
            + "' style='width:100vw;height:100vh;object-fit:contain'></body></html>"] call skw_ui_showHtml;
    };
    _web ctrlShow true;
    (_display displayCtrl 1203) ctrlShow false;
} else {
    _web ctrlShow false;
    (_display displayCtrl 1203) ctrlShow true;
    (_display displayCtrl 1203) ctrlSetText ([DEFAULT_PICTURE, _paa] select (_paa != ""));
};

private _titleText = switch (true) do {
    case (_station == "" || _muted): {""};
    case (_title != ""): {_title};
    default {localize "STR_skw_ui_noTitle"};
};
private _titleCtrl = _display displayCtrl 1201;
_titleCtrl ctrlSetText _titleText;
_titleCtrl ctrlSetTooltip ([localize "STR_skw_ui_clickToCopy", ""] select (_title == "" || _muted));

private _status = switch (true) do {
    case (_station == ""): {[localize "STR_skw_ui_statusOff", [0.62, 0.64, 0.68, 1]]};
    case (_muted): {[localize (switch (_muteReason) do {
        case "all": {"STR_skw_ui_statusMutedAll"};
        case "streamer": {"STR_skw_ui_statusMutedStreamer"};
        case "server": {"STR_skw_ui_statusMutedServer"};
        case "engine": {"STR_skw_ui_statusMutedEngine"};
        default {"STR_skw_ui_statusMuted"};
    }), [0.62, 0.64, 0.68, 1]]};
    case (_state == "playing"): {[localize "STR_skw_ui_statusConnected", [0.35, 0.85, 0.45, 1]]};
    case (_state == "reconnecting"): {
        [format [localize "STR_skw_ui_statusRetrying", ["", format [" (%1)", _detail]] select (_detail != "")], [0.95, 0.35, 0.3, 1]]
    };
    default {[localize "STR_skw_ui_statusLoading", [0.96, 0.78, 0.25, 1]]};
};
private _statusCtrl = _display displayCtrl 1202;
_statusCtrl ctrlSetText (_status select 0);
_statusCtrl ctrlSetTextColor (_status select 1);
(_display displayCtrl 1204) ctrlSetBackgroundColor (_status select 1);

// Buttons.
(_display displayCtrl 1400) ctrlSetText localize (["STR_skw_ui_turnOn", "STR_skw_ui_turnOff"] select (_station != ""));
(_display displayCtrl 1401) ctrlShow (_station != "" && {!_muted} && {_state == "reconnecting"});
private _radioMuted = _object getVariable ["skw_audio_muted", false];
(_display displayCtrl 1402) ctrlSetText localize (["STR_skw_ui_muteForMe", "STR_skw_ui_unmuteForMe"] select _radioMuted);
(_display displayCtrl 1402) ctrlEnable (_station != "");

private _streamerButton = _display displayCtrl 1602;
_streamerButton ctrlSetText localize (["STR_skw_ui_streamerOff", "STR_skw_ui_streamerOn"] select skw_audio_streamerMode);
_streamerButton ctrlSetTextColor ([[0.62, 0.64, 0.68, 1], [0.96, 0.58, 0.16, 1]] select skw_audio_streamerMode);

// Follow volume changes by others, but not while the player is dragging the slider.
if (diag_tickTime - skw_ui_volumeTouched > 1) then {
    (_display displayCtrl 1300) sliderSetPosition (_volume * 100);
};
(_display displayCtrl 1301) ctrlSetText format [localize "STR_skw_ui_volume", round (_volume * 100)];

// Highlight the current station in the (possibly filtered) list.
private _list = _display displayCtrl 1100;
private _listKey = [_station, _record getOrDefault ["class", _station]] select _known;
private _row = -1;
for "_i" from 0 to (((lnbSize _list) select 0) - 1) do {
    if ((_list lnbData [_i, 0]) == _listKey) exitWith {
        _row = _i;
    };
};
if (lnbCurSelRow _list != _row) then {
    skw_ui_updating = true;
    _list lnbSetCurSelRow _row;
    skw_ui_updating = false;
};

// Favourite star for the station playing now (only stations from the list).
skw_ui_currentKey = ["", _listKey] select (_station != "" && {_known});
private _favourite = skw_ui_currentKey in skw_ui_favourites;
{
    (_display displayCtrl _x) ctrlShow (skw_ui_currentKey != "");
} forEach [1206, 1207];
(_display displayCtrl 1206) ctrlSetTextColor ([[0.62, 0.64, 0.68, 0.6], [0.96, 0.58, 0.16, 1]] select _favourite);
(_display displayCtrl 1207) ctrlSetTooltip localize (["STR_skw_ui_favAdd", "STR_skw_ui_favRemove"] select _favourite);

// Last tracks of this radio, newest first; refilled only when they change.
if (skw_ui_historyMode) then {
    private _history = skw_audio_history getOrDefault [_id, []];
    if !(_history isEqualTo skw_ui_historyShown) then {
        skw_ui_historyShown = +_history;
        private _historyList = _display displayCtrl 1105;
        skw_ui_updating = true;
        lnbClear _historyList;
        if (_history isEqualTo []) then {
            _historyList lnbAddRow [localize "STR_skw_ui_historyEmpty"];
        } else {
            {
                private _historyRow = _historyList lnbAddRow [_x];
                _historyList lnbSetData [[_historyRow, 0], _x];
            } forEach _history;
        };
        _historyList lnbSetCurSelRow -1;
        skw_ui_updating = false;
    };
};
