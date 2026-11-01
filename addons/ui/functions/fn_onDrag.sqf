/*
 * Runs every frame while a window is dragged: follows the mouse.
 */
if (skw_ui_drag isEqualTo []) exitWith {
    call skw_ui_fnc_stopDrag;
};
skw_ui_drag params ["_mode", "_start", "_window", "_target"];
private _manager = _target == "manager";
if (isNull (uiNamespace getVariable [["skw_ui_display", "skw_ui_managerDisplay"] select _manager, displayNull])) exitWith {
    call skw_ui_fnc_stopDrag;
};

_window params ["_left", "_top", "_extraW", "_extraH"];
getMousePosition params ["_mouseX", "_mouseY"];
private _cellW = ((safezoneW / safezoneH) min 1.2) / 40 * skw_ui_layoutScale;
private _cellH = (((safezoneW / safezoneH) min 1.2) / 1.2) / 25 * skw_ui_layoutScale;
private _dx = _mouseX - (_start select 0);
private _dy = _mouseY - (_start select 1);

if (_mode == "move") then {
    _window = [_left + _dx, _top + _dy, _extraW, _extraH];
} else {
    // The right edge, the bottom edge or the corner (both) follows the mouse.
    if (_mode in ["width", "corner"]) then {_extraW = _extraW + _dx / _cellW};
    if (_mode in ["height", "corner"]) then {_extraH = _extraH + _dy / _cellH};
    _window = [_left, _top, _extraW, _extraH];
};
if (_manager) then {
    skw_ui_managerWindow = _window;
    call skw_ui_fnc_managerLayout;
} else {
    skw_ui_window = _window;
    call skw_ui_fnc_layout;
};
