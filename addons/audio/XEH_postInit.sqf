if (!hasInterface) exitWith {};

private _version = "skywave" callExtension "version";
if (_version == "") exitWith {
    diag_log text "[SkyWave] Extension skywave_x64.dll could not be loaded, radios stay silent.";
    systemChat ("SkyWave: " + localize "STR_skw_audio_extensionMissing");
};

skw_audio_available = true;
diag_log text format ["[SkyWave] Extension %1 loaded.", _version];

addMissionEventHandler ["ExtensionCallback", {
    params ["_name", "_function", "_data"];
    if (_name != "skywave") exitWith {};
    [_function, _data] call skw_audio_fnc_onCallback;
}];
addMissionEventHandler ["Ended", {call skw_audio_fnc_reset}];
addMissionEventHandler ["MPEnded", {call skw_audio_fnc_reset}];

// Radios from a previous mission may still be running inside the extension.
call skw_audio_fnc_reset;

[{call skw_audio_fnc_update}, 0.05] call CBA_fnc_addPerFrameHandler;
