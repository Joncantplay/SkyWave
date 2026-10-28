/*
 * Opens the radio dialog for an object, where the player last left it.
 *
 * Arguments:
 * 0: Radio object <OBJECT>
 */
params [["_object", objNull, [objNull]]];

if (isNull _object) exitWith {};
if (!isNull (uiNamespace getVariable ["skw_ui_display", displayNull])) exitWith {};

uiNamespace setVariable ["skw_ui_target", _object];
if !(createDialog "skw_ui_radio") exitWith {};

private _display = uiNamespace getVariable ["skw_ui_display", displayNull];
(_display displayCtrl 1000) ctrlSetText format ["SkyWave - %1", getText (configOf _object >> "displayName")];

// Position and size from last time, else centred at normal size.
skw_ui_window = profileNamespace getVariable ["skw_ui_window", []];
if !(skw_ui_window isEqualTypeArray [0, 0, 0, 0]) then {
    skw_ui_window = call skw_ui_fnc_defaultWindow;
};
call skw_ui_fnc_layout;

// Drag the title bar to move, the right/bottom edge or the corner to resize.
{
    _x params ["_idc", "_mode"];
    private _control = _display displayCtrl _idc;
    _control setVariable ["skw_ui_dragMode", _mode];
    _control ctrlAddEventHandler ["MouseButtonDown", {
        params ["_control", "_button"];
        if (_button == 0) then {(_control getVariable "skw_ui_dragMode") call skw_ui_fnc_startDrag};
    }];
    _control ctrlAddEventHandler ["MouseButtonUp", {call skw_ui_fnc_stopDrag}];
} forEach [[1017, "move"], [1622, "width"], [1623, "height"], [1620, "corner"]];
_display displayAddEventHandler ["MouseButtonUp", {call skw_ui_fnc_stopDrag}];

// Focused buttons pulse forever, so move the focus away after a couple of pulses.
{
    (_display displayCtrl _x) ctrlAddEventHandler ["ButtonClick", {
        [{
            private _display = uiNamespace getVariable ["skw_ui_display", displayNull];
            if (!isNull _display) then {
                ctrlSetFocus (_display displayCtrl 1018);
            };
        }, [], 2.5] call CBA_fnc_waitAndExecute;
        false
    }];
} forEach [1400, 1401, 1402, 1403, 1501, 1507, 1601, 1602];

private _slider = _display displayCtrl 1300;
_slider sliderSetRange [0, 100];
_slider sliderSetSpeed [5, 10];
_slider sliderSetPosition (((_object call skw_core_fnc_getState) select 1) * 100);

// Custom URLs only when the server allows them (Zeus always may).
private _urlAllowed = skw_core_allowCustomUrls || {!isNull getAssignedCuratorLogic player};
{
    (_display displayCtrl _x) ctrlShow _urlAllowed;
} forEach [1500, 1501, 1502];

skw_ui_pendingVolume = -1;
skw_ui_logoShown = "";
skw_ui_volumeTouched = -10;
skw_ui_stations = [_object, player] call skw_core_fnc_getStations;
(_display displayCtrl 1702) cbSetChecked skw_ui_freeOnly;
call skw_ui_fnc_fillStations;
skw_ui_historyShown = "";
call skw_ui_fnc_showListMode;
call skw_ui_fnc_refresh;
skw_ui_pfh = [{call skw_ui_fnc_refresh}, 0.2] call CBA_fnc_addPerFrameHandler;
