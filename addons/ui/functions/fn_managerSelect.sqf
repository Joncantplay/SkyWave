/*
 * Station manager list selection: loads the station into the form.
 *
 * Arguments:
 * 0: List control <CONTROL>
 * 1: Selected row <NUMBER>
 */
params ["_list", "_row"];

if (skw_ui_managerUpdating || {_row < 0}) exitWith {};
private _display = uiNamespace getVariable ["skw_ui_managerDisplay", displayNull];

skw_ui_managerIndex = parseNumber (_list lnbData [_row, 0]);
(skw_ui_managerEntries select skw_ui_managerIndex) params ["_name", "_url", "_genre", "_logo", "_free"];
(_display displayCtrl 2201) ctrlSetText _name;
(_display displayCtrl 2202) ctrlSetText _url;
(_display displayCtrl 2203) ctrlSetText _genre;
(_display displayCtrl 2204) ctrlSetText _logo;
(_display displayCtrl 2205) cbSetChecked _free;
(_display displayCtrl 2304) ctrlSetText "";
