diag_log text format ["[SkyWave] Version %1", skw_core_version];

if (isServer) then {
    missionNamespace setVariable ["skw_core_serverVersion", skw_core_version, true];

    // Nobody left on the server: switch every radio off, so the next players find it quiet.
    if (isMultiplayer) then {
        addMissionEventHandler ["PlayerDisconnected", {
            if (!skw_core_offWhenEmpty) exitWith {};
            // The leaving player still counts for a moment.
            [{
                if (skw_core_offWhenEmpty && {(allPlayers - entities "HeadlessClient_F") isEqualTo []}) then {
                    {
                        [_x, "", (_x call skw_core_fnc_getState) select 1] call skw_core_fnc_applyState;
                    } forEach (call skw_core_fnc_getActiveRadios);
                };
            }, [], 3] call CBA_fnc_waitAndExecute;
        }];
    };
};

// Players on a dedicated server: compare versions. All versions share one signing key, so
// Arma lets players in with a different SkyWave version, or onto a server without SkyWave.
if (hasInterface && {!isServer}) then {
    [{!isNull findDisplay 46}, {
        [
            {!isNil "skw_core_serverVersion"},
            {
                if (skw_core_serverVersion != skw_core_version) then {
                    private _message = format [localize "STR_skw_core_versionMismatch", skw_core_serverVersion, skw_core_version];
                    systemChat ("SkyWave: " + _message);
                    diag_log text ("[SkyWave] " + _message);
                };
            },
            [],
            30,
            {
                // The server never told its version: SkyWave does not run there.
                systemChat ("SkyWave: " + localize "STR_skw_core_serverMissing");
                diag_log text "[SkyWave] The server does not run SkyWave, radios cannot be switched on.";
            }
        ] call CBA_fnc_waitUntilAndExecute;
    }] call CBA_fnc_waitUntilAndExecute;
};
