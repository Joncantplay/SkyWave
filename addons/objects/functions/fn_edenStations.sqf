/*
 * Fills the Eden station attribute: "(off)" and the built-in stations by name. A saved
 * value that is not one of them (a stream URL or a station added by script) stays in the
 * list, so older missions keep their setting.
 *
 * Arguments:
 * 0: Combo box <CONTROL>
 * 1: Saved value <STRING>
 */
params ["_combo", ["_value", "", [""]]];

lbClear _combo;
_combo lbSetData [_combo lbAdd localize "STR_skw_objects_attributeOff", ""];

private _stations = ("true" configClasses (configFile >> "CfgSkywaveStations")) apply {
    [getText (_x >> "displayName"), configName _x]
};
_stations sort true;
{
    _x params ["_name", "_class"];
    _combo lbSetData [_combo lbAdd _name, _class];
} forEach _stations;

private _selected = 0;
for "_i" from 1 to (lbSize _combo - 1) do {
    if ((_combo lbData _i) == _value) exitWith {
        _selected = _i;
    };
};
if (_selected == 0 && {_value != ""}) then {
    _selected = _combo lbAdd _value;
    _combo lbSetData [_selected, _value];
};
_combo lbSetCurSel _selected;
