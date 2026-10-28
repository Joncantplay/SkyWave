/*
 * Where a radio window opens the first time (or after a reset): centred, normal size.
 *
 * Return: [x, y, extra width, extra height] <ARRAY>
 */
private _cellW = ((safezoneW / safezoneH) min 1.2) / 40 * skw_ui_scale;
private _cellH = (((safezoneW / safezoneH) min 1.2) / 1.2) / 25 * skw_ui_scale;
skw_ui_windowExtra params ["_extraW", "_extraH"];
[
    safezoneX + (safezoneW - ((skw_ui_windowSize select 0) + _extraW) * _cellW) / 2,
    safezoneY + (safezoneH - ((skw_ui_windowSize select 1) + _extraH) * _cellH) / 2,
    _extraW,
    _extraH
]
