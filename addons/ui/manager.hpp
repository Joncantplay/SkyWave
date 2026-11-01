// Station manager: edit the personal and the server-wide station lists, and switch
// built-in stations off. Positions and font sizes come from skw_ui_managerTable
// (skw_ui_fnc_managerLayout), so the window can be moved and resized like the radio window.
class skw_ui_manager {
    idd = 72101;
    movingEnable = 0;
    onLoad = "uiNamespace setVariable ['skw_ui_managerDisplay', _this select 0]";
    onUnload = "call skw_ui_fnc_managerClose";

    class controlsBackground {
        class Panel: RscText {
            idc = 2010;
            PLACEHOLDER;
            colorBackground[] = COLOR_PANEL;
        };
        class Header: RscText {
            idc = 2000;
            PLACEHOLDER;
            text = "$STR_skw_ui_managerTitle";
            colorBackground[] = COLOR_HEADER;
        };
        class AccentLine: RscText {
            idc = 2011;
            PLACEHOLDER;
            colorBackground[] = COLOR_ACCENT;
        };
        class NameLabel: RscText {
            idc = 2012;
            PLACEHOLDER;
            text = "$STR_skw_ui_name";
            colorText[] = COLOR_MUTED;
        };
        class UrlLabel: RscText {
            idc = 2013;
            PLACEHOLDER;
            text = "$STR_skw_ui_streamUrl";
            colorText[] = COLOR_MUTED;
        };
        class GenreLabel: RscText {
            idc = 2014;
            PLACEHOLDER;
            text = "$STR_skw_ui_genreOptional";
            colorText[] = COLOR_MUTED;
        };
        class LogoLabel: RscText {
            idc = 2015;
            PLACEHOLDER;
            text = "$STR_skw_ui_logoOptional";
            colorText[] = COLOR_MUTED;
        };
        class FreeLabel: RscText {
            idc = 2016;
            PLACEHOLDER;
            text = "$STR_skw_ui_copyrightFree";
        };
        class GripPicture: RscPicture {
            idc = 2021;
            PLACEHOLDER;
            text = "\z\skw\addons\ui\data\grip_ca.paa";
        };
    };

    class controls {
        // Invisible handle over the title bar: drag to move the window.
        class HeaderDrag: skw_ui_clickable {
            idc = 2017;
            PLACEHOLDER;
        };
        class CloseX: skw_ui_button {
            idc = 2009;
            PLACEHOLDER;
            text = "X";
            colorBackground[] = COLOR_HEADER;
            onButtonClick = "closeDialog 0";
        };
        class TabPersonal: skw_ui_button {
            idc = 2001;
            PLACEHOLDER;
            text = "$STR_skw_ui_tabPersonal";
            onButtonClick = "'personal' call skw_ui_fnc_managerShow";
        };
        class TabServer: skw_ui_button {
            idc = 2002;
            PLACEHOLDER;
            text = "$STR_skw_ui_tabServer";
            onButtonClick = "'global' call skw_ui_fnc_managerShow";
        };
        class TabBuiltin: skw_ui_button {
            idc = 2004;
            PLACEHOLDER;
            text = "$STR_skw_ui_tabBuiltin";
            onButtonClick = "'builtin' call skw_ui_fnc_managerShow";
        };
        class Note: RscText {
            idc = 2003;
            PLACEHOLDER;
            colorText[] = COLOR_MUTED;
        };
        class List: RscListNBox {
            idc = 2100;
            PLACEHOLDER;
            columns[] = {0, 0.6};
            drawSideArrows = 0;
            idcLeft = -1;
            idcRight = -1;
            colorBackground[] = COLOR_FIELD;
            colorSelectBackground[] = COLOR_ACCENT;
            colorSelectBackground2[] = COLOR_ACCENT;
            onLBSelChanged = "_this call skw_ui_fnc_managerSelect";
        };
        // Input fields show their whole text as tooltip (skw_ui_fnc_fieldTooltip).
        class Name: RscEdit {
            idc = 2201;
            PLACEHOLDER;
            onKeyUp = "(_this select 0) call skw_ui_fnc_fieldTooltip";
        };
        class Url: RscEdit {
            idc = 2202;
            PLACEHOLDER;
            onKeyUp = "(_this select 0) call skw_ui_fnc_fieldTooltip";
        };
        class Genre: RscEdit {
            idc = 2203;
            PLACEHOLDER;
            onKeyUp = "(_this select 0) call skw_ui_fnc_fieldTooltip";
        };
        class Logo: RscEdit {
            idc = 2204;
            PLACEHOLDER;
            tooltip = "$STR_skw_ui_logoTooltip";
            skw_tooltip = "STR_skw_ui_logoTooltip";
            onKeyUp = "(_this select 0) call skw_ui_fnc_fieldTooltip";
        };
        class Free: RscCheckBox {
            idc = 2205;
            PLACEHOLDER;
            tooltip = "$STR_skw_ui_copyrightFreeTooltip";
        };
        class New: skw_ui_button {
            idc = 2301;
            PLACEHOLDER;
            text = "$STR_skw_ui_new";
            onButtonClick = "call skw_ui_fnc_managerNew";
        };
        class Save: skw_ui_button {
            idc = 2302;
            PLACEHOLDER;
            text = "$STR_skw_ui_save";
            onButtonClick = "call skw_ui_fnc_managerSave";
        };
        class Delete: skw_ui_button {
            idc = 2303;
            PLACEHOLDER;
            text = "$STR_skw_ui_delete";
            onButtonClick = "call skw_ui_fnc_managerDelete";
        };
        class Toggle: skw_ui_button {
            idc = 2306;
            PLACEHOLDER;
            text = "$STR_skw_ui_disable";
            onButtonClick = "call skw_ui_fnc_managerToggle";
        };
        class Message: RscText {
            idc = 2304;
            PLACEHOLDER;
            style = ST_MULTI;
            lineSpacing = 1;
            colorText[] = COLOR_ACCENT;
        };
        class Close: skw_ui_button {
            idc = 2305;
            PLACEHOLDER;
            text = "$STR_skw_ui_close";
            onButtonClick = "closeDialog 0";
        };
        // Invisible handles on the right and bottom edge and over the grip: drag to resize.
        class EdgeRight: skw_ui_clickable {
            idc = 2018;
            PLACEHOLDER;
        };
        class EdgeBottom: skw_ui_clickable {
            idc = 2019;
            PLACEHOLDER;
        };
        class Grip: skw_ui_clickable {
            idc = 2020;
            PLACEHOLDER;
            tooltip = "$STR_skw_ui_resizeHint";
        };
    };
};
