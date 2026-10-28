// All versions share one signing key, so Arma lets players in with a different SkyWave
// version than the server. Tell them, since radios may then not fully work.
private _version = ((getArray (configFile >> "CfgPatches" >> "skw_main" >> "versionAr")) select [0, 3]) joinString ".";

if (isServer) then {
    missionNamespace setVariable ["skw_core_serverVersion", _version, true];
};

if (hasInterface && {!isServer}) then {
    [
        {!isNil "skw_core_serverVersion" && {!isNull findDisplay 46}},
        {
            params ["_version"];
            if (skw_core_serverVersion != _version) then {
                private _message = format [localize "STR_skw_core_versionMismatch", skw_core_serverVersion, _version];
                systemChat ("SkyWave: " + _message);
                diag_log text ("[SkyWave] " + _message);
            };
        },
        [_version],
        120
    ] call CBA_fnc_waitUntilAndExecute;
};
