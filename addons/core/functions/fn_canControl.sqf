/*
 * Returns whether a unit may operate a radio.
 *
 * - Zeus: always. Zeus can lock a radio so only Zeus may operate it (skw_core_zeusOnly).
 * - Vehicles: from inside; with "Driver and commander only" only those two seats.
 *   With "Operate from outside" also from close by outside the vehicle.
 * - Objects Zeus turned into a radio: from close by, vehicles also from inside.
 * - Placed radio objects: from close by.
 * "Close by" counts from the object's edge, so large objects work too.
 *
 * Arguments:
 * 0: Unit <OBJECT>
 * 1: Radio object <OBJECT>
 *
 * Return: <BOOL>
 */
#define OUTSIDE_RANGE 4
#define OBJECT_RANGE 3

params [["_unit", objNull, [objNull]], ["_object", objNull, [objNull]]];

if (isNull _unit || {isNull _object} || {!alive _unit}) exitWith {false};
if (!isNull getAssignedCuratorLogic _unit) exitWith {true};
// Zeus locked this radio for everyone else.
if (_object getVariable ["skw_core_zeusOnly", false]) exitWith {false};

// Rough distance to the object's edge (minus half the bounding radius).
private _gap = (_unit distance _object) - ((boundingBoxReal _object) select 2) / 2;
private _zeusRadio = _object getVariable ["skw_core_radioEnabled", false];

if (_object isKindOf "AllVehicles" && {!(_object isKindOf "CAManBase")}) exitWith {
    if (vehicle _unit == _object) then {
        !skw_core_driverCommanderOnly || {_unit in [driver _object, commander _object, effectiveCommander _object]}
    } else {
        (skw_core_allowOutside || _zeusRadio) && {isNull objectParent _unit} && {_gap < OUTSIDE_RANGE}
    };
};

_gap < OBJECT_RANGE
