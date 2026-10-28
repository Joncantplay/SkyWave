/*
 * Puts the sort arrow right after the text of the column header the list is sorted by,
 * pointing up for ascending and down for descending order.
 */
private _display = uiNamespace getVariable ["skw_ui_display", displayNull];
if (isNull _display) exitWith {};

skw_ui_sort params ["_column", "_ascending"];
private _header = _display displayCtrl ([1710, 1711] select (_column == "genre"));
private _arrow = _display displayCtrl 1712;
_arrow ctrlSetText (["\z\skw\addons\ui\data\sort_down_ca.paa", "\z\skw\addons\ui\data\sort_up_ca.paa"] select _ascending);

private _scale = skw_ui_layoutScale;
private _cellW = ((safezoneW / safezoneH) min 1.2) / 40 * _scale;
private _cellH = (((safezoneW / safezoneH) min 1.2) / 1.2) / 25 * _scale;
ctrlPosition _header params ["_left", "_top", "", "_height"];
// Square on screen: convert the height to a width via the screen's pixel aspect.
private _size = 0.6 * _cellH;
getResolution params ["_screenW", "_screenH"];
private _width = _size * (safezoneW / safezoneH) * (_screenH / _screenW);
_arrow ctrlSetPosition [
    _left + ctrlTextWidth _header + 0.3 * _cellW,
    _top + (_height - _size) / 2,
    _width,
    _size
];
_arrow ctrlCommit 0;
