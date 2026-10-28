/*
 * Saves a CBA setting changed from SkyWave's own windows, so the Addon Options show it
 * no matter which source tab (server / client) is open there.
 *
 * The setting is written to its own source and, on the host or in singleplayer, also to
 * the other one. Each source keeps its current priority ("overwrite" flags stay as set).
 *
 * Arguments:
 * 0: Setting name <STRING>
 * 1: Value <ANY>
 * 2: Source the setting belongs to: "client" (player setting) or "server" <STRING>
 *
 * Return: true if the main source was written <BOOL>
 */
params [["_setting", "", [""]], "_value", ["_source", "client", [""]]];

private _sources = [_source];
if (isServer) then {
    _sources pushBackUnique (["server", "client"] select (_source == "server"));
};

private _results = _sources apply {
    [_setting, _value, [_setting, _x] call CBA_settings_fnc_priority, _x, true] call CBA_settings_fnc_set
};
_results select 0
