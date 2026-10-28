/*
 * Server side of skw_core_fnc_request: checks the sender and applies the new state.
 *
 * Arguments:
 * 0: Requesting unit <OBJECT>
 * 1: Object <OBJECT>
 * 2: Station <STRING>
 * 3: Volume 0..1 <NUMBER>
 * 4: Label <STRING>
 */
params [["_unit", objNull, [objNull]], ["_object", objNull, [objNull]], ["_station", "", [""]], ["_volume", 0.7, [0]], ["_label", "", [""]]];

if (!isServer) exitWith {};
// The unit must belong to the machine that sent the request.
if (isMultiplayer && {owner _unit != remoteExecutedOwner}) exitWith {};

private _sender = [remoteExecutedOwner, clientOwner] select (remoteExecutedOwner == 0);
if !([_unit, _object] call skw_core_fnc_canControl) exitWith {
    ["STR_skw_core_noPermission"] remoteExecCall ["skw_core_fnc_onRejected", _sender];
};

private _lower = toLower _station;
private _isUrl = (_lower select [0, 7]) == "http://" || {(_lower select [0, 8]) == "https://"};
private _isZeus = !isNull getAssignedCuratorLogic _unit;
if (_isUrl && {!skw_core_allowCustomUrls} && {!_isZeus}) exitWith {
    ["STR_skw_core_noCustomUrls"] remoteExecCall ["skw_core_fnc_onRejected", _sender];
};

// Stations must exist; their conditions apply to everyone but Zeus.
private _available = true;
if (_station != "" && {!_isUrl}) then {
    private _record = _station call skw_core_fnc_getStation;
    _available = count _record > 0 && {_isZeus || {[_record, _object, _unit] call skw_core_fnc_isStationAvailable}};
};
if (!_available) exitWith {
    ["STR_skw_core_stationUnavailable"] remoteExecCall ["skw_core_fnc_onRejected", _sender];
};

if (skw_core_copyrightFreeOnly && {!(_station call skw_core_fnc_isCopyrightFree)}) exitWith {
    ["STR_skw_core_onlyCopyrightFree"] remoteExecCall ["skw_core_fnc_onRejected", _sender];
};

[_object, _station, _volume, _label] call skw_core_fnc_applyState;
