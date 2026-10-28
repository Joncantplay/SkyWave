/*
 * Handles events from the extension.
 *
 * Arguments:
 * 0: Function: "status", "meta" or "log" <STRING>
 * 1: Data <STRING>
 */
params ["_function", "_data"];

switch (_function) do {
    case "status": {
        (parseSimpleArray _data) params ["_id", "_state", "_detail"];
        if !(_id in skw_audio_playing) exitWith {};
        skw_audio_status set [_id, [_state, _detail]];
        private _text = switch (_state) do {
            case "connecting": {localize "STR_skw_audio_msgConnecting"};
            case "buffering": {format [localize "STR_skw_audio_msgLoading", _detail]};
            case "playing": {localize "STR_skw_audio_msgPlaying"};
            case "reconnecting": {
                [localize "STR_skw_audio_msgReconnecting", format [localize "STR_skw_audio_msgConnectionLost", _detail]] select (_detail != "")
            };
            default {_state};
        };
        [_text, _id] call skw_audio_fnc_notify;
    };
    case "meta": {
        (parseSimpleArray _data) params ["_id", "_title", "_station", ["_cover", ""]];
        if !(_id in skw_audio_playing) exitWith {};
        skw_audio_titles set [_id, [_title, _station, _cover]];
        // Remember the last ten tracks for the history in the radio window.
        private _history = skw_audio_history getOrDefault [_id, []];
        if (_title != "" && {(_history param [0, ""]) != _title}) then {
            _history = [_title] + (_history select [0, 9]);
            skw_audio_history set [_id, _history];
        };
        if (_title != "") then {
            [format [localize "STR_skw_audio_msgNowPlaying", _title], _id] call skw_audio_fnc_notify;
        };
    };
    case "logo": {
        (parseSimpleArray _data) params ["_key", "_state", "_chunks"];
        private _logo = "";
        if (_state == "ready") then {
            for "_i" from 0 to (_chunks - 1) do {
                _logo = _logo + (("skywave" callExtension ["logoChunk", [_key, _i]]) select 0);
            };
        };
        skw_audio_logos set [_key, [_state, _logo]];
    };
    case "log": {
        diag_log text ("[SkyWave] " + _data);
    };
};
