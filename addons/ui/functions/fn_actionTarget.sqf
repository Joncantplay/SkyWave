/*
 * Returns the radio the player can open right now: the vehicle they sit in, otherwise the
 * vehicle or placed radio they look at from close by. objNull if there is none or the
 * player may not operate it.
 *
 * Return: <OBJECT>
 */
private _vehicle = vehicle player;
private _target = if (_vehicle != player) then {_vehicle} else {cursorObject};

if (
    isNull _target
    || {!(_target call skw_core_fnc_isRadioCapable)}
    || {!([player, _target] call skw_core_fnc_canControl)}
) exitWith {objNull};
_target
