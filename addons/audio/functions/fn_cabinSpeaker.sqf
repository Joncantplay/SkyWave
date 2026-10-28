/*
 * Where a built-in radio sits in a vehicle: on the centre line, a little ahead of and
 * below the eyes of the driver (or of the player when nobody drives).
 *
 * Arguments:
 * 0: Vehicle <OBJECT>
 *
 * Return: position ASL <ARRAY>
 */
params [["_vehicle", objNull, [objNull]]];

private _seat = driver _vehicle;
if (isNull _seat || {!alive _seat}) then {
    _seat = [focusOn, player] select (isNull focusOn);
};
private _eyes = _vehicle worldToModelVisual (ASLToAGL eyePos _seat);
_vehicle modelToWorldVisualWorld [0, (_eyes select 1) + 0.7, (_eyes select 2) - 0.25]
