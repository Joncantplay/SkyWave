/*
 * Returns why a radio is silent for this player, or "" if it may play.
 * Muted radios are not downloaded at all. Only affects this player.
 *
 * Arguments:
 * 0: Radio object <OBJECT>
 * 1: Station <STRING>
 *
 * Return: "", "radio" (this radio muted), "all" (all radios muted), "streamer"
 *         (Streamer Mode, not copyright-free) or "server" (server allows only
 *         copyright-free stations) or "engine" (vehicle engine off while the server
 *         wants radios only with the engine running) <STRING>
 */
params [["_object", objNull, [objNull]], ["_station", "", [""]]];

switch (true) do {
    case (_object getVariable ["skw_audio_muted", false]): {"radio"};
    case (skw_audio_muteAll): {"all"};
    case (
        skw_core_engineOnly
        && {_object isKindOf "AllVehicles"}
        && {!(_object isKindOf "CAManBase")}
        && {!isEngineOn _object}
    ): {"engine"};
    case ((skw_core_copyrightFreeOnly || skw_audio_streamerMode) && {!(_station call skw_core_fnc_isCopyrightFree)}): {
        ["streamer", "server"] select skw_core_copyrightFreeOnly
    };
    default {""};
};
