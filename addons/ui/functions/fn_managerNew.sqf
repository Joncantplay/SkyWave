/*
 * Station manager "New": clears the form for a new station.
 */
private _display = uiNamespace getVariable ["skw_ui_managerDisplay", displayNull];
if (isNull _display) exitWith {};

skw_ui_managerIndex = -1;
{
    (_display displayCtrl _x) ctrlSetText "";
} forEach [2201, 2202, 2203, 2204, 2304];
(_display displayCtrl 2205) cbSetChecked false;

skw_ui_managerUpdating = true;
(_display displayCtrl 2100) lnbSetCurSelRow -1;
skw_ui_managerUpdating = false;
