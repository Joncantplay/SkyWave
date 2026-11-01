/*
 * Shows the whole text of an input field as its tooltip, so long addresses can be read.
 * An empty field gets its normal tooltip back (skw_tooltip in the control's config).
 *
 * Arguments:
 * 0: Input field <CONTROL>
 */
params ["_control"];

private _config = configFile >> "skw_ui_manager" >> "controls" >> ctrlClassName _control;
if (!isClass _config) then {
    _config = configFile >> "skw_ui_radio" >> "controls" >> ctrlClassName _control;
};
private _default = getText (_config >> "skw_tooltip");
private _text = ctrlText _control;
_control ctrlSetTooltip (switch (true) do {
    case (_text != ""): {_text};
    case (_default != ""): {localize _default};
    default {""};
});
