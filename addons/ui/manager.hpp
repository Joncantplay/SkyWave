// Station manager: edit the personal and the server-wide station lists.
class skw_ui_manager {
    idd = 72101;
    movingEnable = 0;
    onLoad = "uiNamespace setVariable ['skw_ui_managerDisplay', _this select 0]";
    onUnload = "call skw_ui_fnc_managerClose";

    class controlsBackground {
        class Panel: RscText {
            idc = -1;
            x = POS_X(5);
            y = POS_Y(3);
            w = SIZE_W(30);
            h = SIZE_H(19);
            colorBackground[] = COLOR_PANEL;
        };
        class Header: RscText {
            idc = 2000;
            text = "$STR_skw_ui_managerTitle";
            x = POS_X(5);
            y = POS_Y(3);
            w = SIZE_W(30);
            h = SIZE_H(1.4);
            sizeEx = SIZE_H(1);
            colorBackground[] = COLOR_HEADER;
        };
        class AccentLine: RscText {
            idc = -1;
            x = POS_X(5);
            y = POS_Y(4.4);
            w = SIZE_W(30);
            h = SIZE_H(0.12);
            colorBackground[] = COLOR_ACCENT;
        };
        class NameLabel: RscText {
            idc = -1;
            text = "$STR_skw_ui_name";
            x = POS_X(19);
            y = POS_Y(6.7);
            w = SIZE_W(15.5);
            h = SIZE_H(0.9);
            colorText[] = COLOR_MUTED;
        };
        class UrlLabel: RscText {
            idc = -1;
            text = "$STR_skw_ui_streamUrl";
            x = POS_X(19);
            y = POS_Y(8.8);
            w = SIZE_W(15.5);
            h = SIZE_H(0.9);
            colorText[] = COLOR_MUTED;
        };
        class GenreLabel: RscText {
            idc = -1;
            text = "$STR_skw_ui_genreOptional";
            x = POS_X(19);
            y = POS_Y(10.9);
            w = SIZE_W(15.5);
            h = SIZE_H(0.9);
            colorText[] = COLOR_MUTED;
        };
        class LogoLabel: RscText {
            idc = -1;
            text = "$STR_skw_ui_logoOptional";
            x = POS_X(19);
            y = POS_Y(13);
            w = SIZE_W(15.5);
            h = SIZE_H(0.9);
            colorText[] = COLOR_MUTED;
        };
        class FreeLabel: RscText {
            idc = -1;
            text = "$STR_skw_ui_copyrightFree";
            x = POS_X(20.2);
            y = POS_Y(15.2);
            w = SIZE_W(14.3);
            h = SIZE_H(1);
        };
    };

    class controls {
        class CloseX: skw_ui_button {
            idc = 2009;
            text = "X";
            x = POS_X(33.6);
            y = POS_Y(3);
            w = SIZE_W(1.4);
            h = SIZE_H(1.4);
            colorBackground[] = COLOR_HEADER;
            onButtonClick = "closeDialog 0";
        };
        class TabPersonal: skw_ui_button {
            idc = 2001;
            text = "$STR_skw_ui_tabPersonal";
            x = POS_X(5.5);
            y = POS_Y(5);
            w = SIZE_W(6.5);
            h = SIZE_H(1.2);
            onButtonClick = "'personal' call skw_ui_fnc_managerShow";
        };
        class TabServer: skw_ui_button {
            idc = 2002;
            text = "$STR_skw_ui_tabServer";
            x = POS_X(12.2);
            y = POS_Y(5);
            w = SIZE_W(6.5);
            h = SIZE_H(1.2);
            onButtonClick = "'global' call skw_ui_fnc_managerShow";
        };
        class Note: RscText {
            idc = 2003;
            x = POS_X(19);
            y = POS_Y(5);
            w = SIZE_W(15.5);
            h = SIZE_H(1.2);
            colorText[] = COLOR_MUTED;
        };
        class List: RscListNBox {
            idc = 2100;
            x = POS_X(5.5);
            y = POS_Y(6.7);
            w = SIZE_W(13);
            h = SIZE_H(14.8);
            columns[] = {0, 0.6};
            drawSideArrows = 0;
            idcLeft = -1;
            idcRight = -1;
            colorBackground[] = COLOR_FIELD;
            colorSelectBackground[] = COLOR_ACCENT;
            colorSelectBackground2[] = COLOR_ACCENT;
            onLBSelChanged = "_this call skw_ui_fnc_managerSelect";
        };
        class Name: RscEdit {
            idc = 2201;
            x = POS_X(19);
            y = POS_Y(7.6);
            w = SIZE_W(15.5);
            h = SIZE_H(1);
        };
        class Url: RscEdit {
            idc = 2202;
            x = POS_X(19);
            y = POS_Y(9.7);
            w = SIZE_W(15.5);
            h = SIZE_H(1);
        };
        class Genre: RscEdit {
            idc = 2203;
            x = POS_X(19);
            y = POS_Y(11.8);
            w = SIZE_W(15.5);
            h = SIZE_H(1);
        };
        class Logo: RscEdit {
            idc = 2204;
            tooltip = "$STR_skw_ui_logoTooltip";
            x = POS_X(19);
            y = POS_Y(13.9);
            w = SIZE_W(15.5);
            h = SIZE_H(1);
        };
        class Free: RscCheckBox {
            idc = 2205;
            tooltip = "$STR_skw_ui_copyrightFreeTooltip";
            x = POS_X(19);
            y = POS_Y(15.2);
            w = SIZE_W(1);
            h = SIZE_H(1);
        };
        class New: skw_ui_button {
            idc = 2301;
            text = "$STR_skw_ui_new";
            x = POS_X(19);
            y = POS_Y(16.8);
            w = SIZE_W(5);
            h = SIZE_H(1.2);
            onButtonClick = "call skw_ui_fnc_managerNew";
        };
        class Save: skw_ui_button {
            idc = 2302;
            text = "$STR_skw_ui_save";
            x = POS_X(24.25);
            y = POS_Y(16.8);
            w = SIZE_W(5);
            h = SIZE_H(1.2);
            onButtonClick = "call skw_ui_fnc_managerSave";
        };
        class Delete: skw_ui_button {
            idc = 2303;
            text = "$STR_skw_ui_delete";
            x = POS_X(29.5);
            y = POS_Y(16.8);
            w = SIZE_W(5);
            h = SIZE_H(1.2);
            onButtonClick = "call skw_ui_fnc_managerDelete";
        };
        class Message: RscText {
            idc = 2304;
            style = ST_MULTI;
            lineSpacing = 1;
            x = POS_X(19);
            y = POS_Y(18.3);
            w = SIZE_W(15.5);
            h = SIZE_H(1.9);
            colorText[] = COLOR_ACCENT;
        };
        class Close: skw_ui_button {
            idc = 2305;
            text = "$STR_skw_ui_close";
            x = POS_X(29.5);
            y = POS_Y(20.3);
            w = SIZE_W(5);
            h = SIZE_H(1.2);
            onButtonClick = "closeDialog 0";
        };
    };
};
