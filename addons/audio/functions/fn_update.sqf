/*
 * Runs every 50 ms: decides which radios are in range, starts and stops them in the
 * extension, and sends listener and radio positions for 3D audio.
 */
if (!skw_audio_available) exitWith {};

(call skw_audio_fnc_listener) params ["_listenerPos", "_forward", "_up"];

private _master = skw_audio_masterVolume * skw_audio_volumeBoost * soundVolume;
if (skw_audio_muteUnfocused && {!isGameFocused}) then {
    _master = 0;
};

// Sitting in a closed vehicle muffles every radio outside of it.
// cameraOn returns the vehicle when seated, we need the unit.
private _inZeus = !isNull curatorCamera;
private _listenerUnit = [focusOn, player] select (isNull focusOn);
private _listenerVehicle = vehicle _listenerUnit;
private _listenerMuffle = 0;
if (!_inZeus && {_listenerVehicle != _listenerUnit} && {!isTurnedOut _listenerUnit} && {_listenerVehicle call skw_audio_fnc_isClosed}) then {
    _listenerMuffle = 0.6;
};

private _range = skw_audio_range;
private _wanted = createHashMap;
private _sources = [];

{
    private _object = _x;
    if (!alive _object) then {continue};

    (_object call skw_core_fnc_getState) params ["_station", "_volume"];
    private _url = _station call skw_core_fnc_stationUrl;
    if (_url == "") then {continue};
    // Muted for this player: don't even download it. Engine off is the exception, the
    // stream keeps running at zero volume so it comes back instantly.
    private _muteReason = [_object, _station] call skw_audio_fnc_muteReason;
    if (_muteReason != "" && {_muteReason != "engine"}) then {continue};
    if (_muteReason == "engine") then {
        _volume = 0;
    };

    private _id = _object call skw_audio_fnc_sourceId;
    private _position = getPosASLVisual _object;
    // Some extra range for radios already playing, otherwise they flicker at the edge.
    private _limit = [_range, _range + 25] select (_id in skw_audio_playing);
    if ((_listenerPos distance _position) > _limit) then {continue};

    private _inside = !_inZeus && {_listenerVehicle == _object};
    // Own vehicle: stereo, or a speaker at the front of the cabin (setting). In third person
    // the speaker is placed relative to the camera instead of the head, sounds the same.
    private _stereo = _inside;
    if (_inside && {
        skw_audio_vehicle3D == 2 || {skw_audio_vehicle3D == 1 && {_object isKindOf "LandVehicle" || {_object isKindOf "Ship"}}}
    }) then {
        _stereo = false;
        _position = _object call skw_audio_fnc_cabinSpeaker;
        if !(cameraView in ["INTERNAL", "GUNNER"]) then {
            private _head = eyePos _listenerUnit;
            _position = _listenerPos vectorAdd (_position vectorDiff _head);
        };
    };
    private _muffle = 0;
    if (!_inside) then {
        if (_object call skw_audio_fnc_isClosed) then {
            _muffle = 0.6;
        };
        _muffle = (_muffle + _listenerMuffle) min 0.9;
    };

    _wanted set [_id, [_object, _url]];
    _sources pushBack ([
        _id,
        (_position select 0) toFixed 2,
        (_position select 1) toFixed 2,
        (_position select 2) toFixed 2,
        _volume toFixed 3,
        _muffle toFixed 2,
        [0, 1] select _stereo,
        _range toFixed 0
    ] joinString "|");
} forEach (call skw_core_fnc_getActiveRadios);

// Stop radios that were switched off or left the range.
{
    if !(_x in _wanted) then {
        "skywave" callExtension ["stop", [_x]];
        skw_audio_playing deleteAt _x;
        skw_audio_status deleteAt _x;
        skw_audio_titles deleteAt _x;
    };
} forEach (keys skw_audio_playing);

// Start new radios and retune changed ones.
{
    _y params ["_object", "_url"];
    if (((skw_audio_playing getOrDefault [_x, [objNull, ""]]) select 1) != _url) then {
        "skywave" callExtension ["play", [_x, _url]];
        skw_audio_playing set [_x, [_object, _url]];
        // A new station: forget the old one's title, cover and status.
        skw_audio_titles deleteAt _x;
        skw_audio_status deleteAt _x;
    };
} forEach _wanted;

private _listener = (_listenerPos + _forward + _up) apply {_x toFixed 3};
"skywave" callExtension ["frame", [_listener joinString ",", _master toFixed 3, _sources joinString ";"]];
