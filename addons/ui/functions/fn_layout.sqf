/*
 * Places all controls of the radio window for the current position and size
 * (skw_ui_window = [x, y, extra width, extra height]) at the size chosen in the setting
 * skw_ui_scale, keeping the window on screen and no smaller than the layout's minimum.
 */
private _display = uiNamespace getVariable ["skw_ui_display", displayNull];
if (isNull _display) exitWith {};

private _cellW = ((safezoneW / safezoneH) min 1.2) / 40;
private _cellH = (((safezoneW / safezoneH) min 1.2) / 1.2) / 25;
skw_ui_windowSize params ["_cellsW", "_cellsH"];

skw_ui_window params ["_left", "_top", "_extraW", "_extraH"];
private _scale = skw_ui_scale min ((safezoneW / (_cellsW * _cellW)) min (safezoneH / (_cellsH * _cellH)));
skw_ui_layoutScale = _scale;
_extraW = (_extraW min (safezoneW / (_cellW * _scale) - _cellsW)) max 0;
_extraH = (_extraH min (safezoneH / (_cellH * _scale) - _cellsH)) max 0;
private _width = (_cellsW + _extraW) * _cellW * _scale;
private _height = (_cellsH + _extraH) * _cellH * _scale;
_left = (_left max safezoneX) min (safezoneX + safezoneW - _width);
_top = (_top max safezoneY) min (safezoneY + safezoneH - _height);
skw_ui_window = [_left, _top, _extraW, _extraH];

{
    _x params ["_idc", "_cx", "_cy", "_cw", "_ch", "_font", "_stretch"];
    _stretch params ["_sx", "_sy", "_sw", "_sh"];
    private _control = _display displayCtrl _idc;
    _control ctrlSetPosition [
        _left + (_cx + _sx * _extraW) * _cellW * _scale,
        _top + (_cy + _sy * _extraH) * _cellH * _scale,
        (_cw + _sw * _extraW) * _cellW * _scale,
        (_ch + _sh * _extraH) * _cellH * _scale
    ];
    if (_font > 0) then {
        _control ctrlSetFontHeight (_font * _cellH * _scale);
    };
    _control ctrlCommit 0;
} forEach skw_ui_layoutTable;
call skw_ui_fnc_placeSortArrow;
