/*
 * Where a window opens the first time (or after a reset): centred, normal size.
 *
 * Arguments:
 * 0: Smallest size in grid cells [w, h] <ARRAY> (default: radio window)
 * 1: Extra size it opens with [w, h] <ARRAY> (default: radio window)
 *
 * Return: [x, y, extra width, extra height] <ARRAY>
 */
params [["_size", skw_ui_windowSize, [[]]], ["_extra", skw_ui_windowExtra, [[]]]];

private _cellW = ((safezoneW / safezoneH) min 1.2) / 40 * skw_ui_scale;
private _cellH = (((safezoneW / safezoneH) min 1.2) / 1.2) / 25 * skw_ui_scale;
_extra params ["_extraW", "_extraH"];
[
    safezoneX + (safezoneW - ((_size select 0) + _extraW) * _cellW) / 2,
    safezoneY + (safezoneH - ((_size select 1) + _extraH) * _cellH) / 2,
    _extraW,
    _extraH
]
