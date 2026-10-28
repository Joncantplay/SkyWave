/*
 * Station manager "Delete": removes the selected station.
 */
private _display = uiNamespace getVariable ["skw_ui_managerDisplay", displayNull];
if (isNull _display || {!skw_ui_managerEditable}) exitWith {};

if (skw_ui_managerIndex < 0) exitWith {
    (_display displayCtrl 2304) ctrlSetText localize "STR_skw_ui_selectFirst";
};

private _name = (skw_ui_managerEntries select skw_ui_managerIndex) select 0;
skw_ui_managerEntries deleteAt skw_ui_managerIndex;
call skw_ui_fnc_managerWrite;
call skw_ui_fnc_managerFill;
call skw_ui_fnc_managerNew;
(_display displayCtrl 2304) ctrlSetText format [localize "STR_skw_ui_removed", _name];
