/*
 * Returns whether a vehicle has a closed cabin that muffles sound.
 * Based on the vehicle's sound attenuation config; open vehicles and objects
 * without one count as open.
 *
 * Arguments:
 * 0: Vehicle <OBJECT>
 *
 * Return: <BOOL>
 */
params ["_vehicle"];

private _type = typeOf _vehicle;
private _cached = skw_audio_closedCache get _type;
if (!isNil "_cached") exitWith {_cached};

private _effect = toLower getText (configOf _vehicle >> "attenuationEffectType");
private _closed = _effect != "" && {(_effect find "open") < 0};
skw_audio_closedCache set [_type, _closed];
_closed
