// Radio window. Positions and font sizes are set by skw_ui_fnc_layout from
// skw_ui_layoutTable, so the window can be moved and scaled; the values here are only
// placeholders until the first layout.
#define PLACEHOLDER x = 0; y = 0; w = 0; h = 0

class skw_ui_clickable: RscButton {
    style = 0;
    colorBackground[] = {0, 0, 0, 0};
    colorBackgroundActive[] = {0, 0, 0, 0};
    colorBackgroundDisabled[] = {0, 0, 0, 0};
    colorFocused[] = {0, 0, 0, 0};
    colorShadow[] = {0, 0, 0, 0};
    colorBorder[] = {0, 0, 0, 0};
    colorText[] = {1, 1, 1, 1};
    colorDisabled[] = {1, 1, 1, 1};
    offsetPressedX = 0;
    offsetPressedY = 0;
    text = "";
};

class skw_ui_radio {
    idd = 72100;
    movingEnable = 0;
    onLoad = "uiNamespace setVariable ['skw_ui_display', _this select 0]";
    onUnload = "call skw_ui_fnc_onClose";

    class controlsBackground {
        class Panel: RscText {
            idc = 1001;
            PLACEHOLDER;
            colorBackground[] = COLOR_PANEL;
        };
        class HeaderBackground: RscPicture {
            idc = 1010;
            PLACEHOLDER;
            text = "\z\skw\addons\ui\data\header_ca.paa";
        };
        class HeaderLogo: RscPicture {
            idc = 1011;
            PLACEHOLDER;
            text = "\z\skw\addons\ui\data\logo_white_ca.paa";
            colorText[] = {1, 1, 1, 1};
        };
        class Header: RscText {
            idc = 1000;
            PLACEHOLDER;
            text = "SKYWAVE";
            colorBackground[] = {0, 0, 0, 0};
            shadow = 1;
        };
        class AccentLine: RscPicture {
            idc = 1012;
            PLACEHOLDER;
            text = "\z\skw\addons\ui\data\accent_ca.paa";
        };
        class Card: RscText {
            idc = 1013;
            PLACEHOLDER;
            colorBackground[] = COLOR_CARD;
        };
        class PictureFrame: RscText {
            idc = 1014;
            PLACEHOLDER;
            colorBackground[] = COLOR_FIELD;
        };
        class SearchLabel: RscText {
            idc = 1015;
            PLACEHOLDER;
            text = "$STR_skw_ui_search";
            colorText[] = COLOR_MUTED;
        };
        class FavouriteStar: RscPicture {
            idc = 1206;
            PLACEHOLDER;
            text = "\z\skw\addons\ui\data\star_ca.paa";
            colorText[] = COLOR_MUTED;
        };
        // Sort direction, placed right after the active column's header text.
        class SortArrow: RscPicture {
            idc = 1712;
            PLACEHOLDER;
            text = "\z\skw\addons\ui\data\sort_up_ca.paa";
            colorText[] = COLOR_ACCENT;
        };
        class GripPicture: RscPicture {
            idc = 1621;
            PLACEHOLDER;
            text = "\z\skw\addons\ui\data\grip_ca.paa";
        };
    };

    class controls {
        // Gets the focus a moment after a button click so the button stops pulsing.
        // Invisible, zero size, not in the layout table.
        class FocusSink: skw_ui_clickable {
            idc = 1018;
            PLACEHOLDER;
        };
        // Invisible handle over the title bar: drag to move the window.
        class HeaderDrag: skw_ui_clickable {
            idc = 1017;
            PLACEHOLDER;
        };
        class Streamer: skw_ui_button {
            idc = 1602;
            PLACEHOLDER;
            text = "";
            colorBackground[] = {0, 0, 0, 0.25};
            tooltip = "$STR_skw_audio_streamerMode_tooltip";
            onButtonClick = "['skw_audio_streamerMode', 'STR_skw_audio_streamerOn', 'STR_skw_audio_streamerOff'] call skw_audio_fnc_toggleSetting";
        };
        class CloseX: skw_ui_button {
            idc = 1601;
            PLACEHOLDER;
            text = "X";
            colorBackground[] = {0, 0, 0, 0.25};
            onButtonClick = "closeDialog 0";
        };
        // Station logos come as embedded images, which only the browser control can show.
        class Logo {
            idc = 1205;
            type = 106;
            style = 0;
            PLACEHOLDER;
            url = "";
            text = "";
            font = "RobotoCondensed";
            sizeEx = 0.03;
            colorText[] = {1, 1, 1, 1};
            colorBackground[] = {0, 0, 0, 0};
        };
        class Picture: RscPicture {
            idc = 1203;
            PLACEHOLDER;
            text = "\A3\ui_f\data\gui\Rsc\RscDisplayArsenal\radio_ca.paa";
        };
        class StationName: RscText {
            idc = 1200;
            PLACEHOLDER;
        };
        // Click to copy the track title.
        class TrackTitle: skw_ui_clickable {
            idc = 1201;
            PLACEHOLDER;
            colorText[] = {0.9, 0.9, 0.92, 1};
            onButtonClick = "call skw_ui_fnc_copyTitle";
        };
        class StatusDot: RscText {
            idc = 1204;
            PLACEHOLDER;
            colorBackground[] = COLOR_MUTED;
        };
        class Status: RscText {
            idc = 1202;
            PLACEHOLDER;
            colorText[] = COLOR_MUTED;
        };
        class Power: skw_ui_button {
            idc = 1400;
            PLACEHOLDER;
            text = "$STR_skw_ui_turnOn";
            onButtonClick = "call skw_ui_fnc_onPower";
        };
        class Retry: skw_ui_button {
            idc = 1401;
            PLACEHOLDER;
            text = "$STR_skw_ui_retry";
            onButtonClick = "call skw_ui_fnc_onRetry";
        };
        class Random: skw_ui_button {
            idc = 1403;
            PLACEHOLDER;
            text = "$STR_skw_ui_random";
            tooltip = "$STR_skw_ui_random_tooltip";
            onButtonClick = "call skw_ui_fnc_onRandom";
        };
        class Mute: skw_ui_button {
            idc = 1402;
            PLACEHOLDER;
            text = "$STR_skw_ui_muteForMe";
            onButtonClick = "call skw_ui_fnc_onMute";
        };
        class VolumeLabel: RscText {
            idc = 1301;
            PLACEHOLDER;
        };
        class Volume: RscXSliderH {
            idc = 1300;
            PLACEHOLDER;
            onSliderPosChanged = "_this call skw_ui_fnc_onVolumeChanged";
        };
        class Search: RscEdit {
            idc = 1700;
            PLACEHOLDER;
            onKeyUp = "call skw_ui_fnc_onSearch";
        };
        class FreeOnly: RscCheckBox {
            idc = 1702;
            PLACEHOLDER;
            tooltip = "$STR_skw_ui_freeOnlyTooltip";
            onCheckedChanged = "_this call skw_ui_fnc_onFreeOnly";
        };
        class FreeOnlyLabel: RscText {
            idc = 1703;
            PLACEHOLDER;
            text = "$STR_skw_ui_freeOnly";
        };
        class SortStation: skw_ui_clickable {
            idc = 1710;
            PLACEHOLDER;
            colorText[] = COLOR_ACCENT;
            tooltip = "$STR_skw_ui_sortHint";
            onButtonClick = "'name' call skw_ui_fnc_onSort";
        };
        class SortGenre: skw_ui_clickable {
            idc = 1711;
            PLACEHOLDER;
            colorText[] = COLOR_ACCENT;
            tooltip = "$STR_skw_ui_sortHint";
            onButtonClick = "'genre' call skw_ui_fnc_onSort";
        };
        class StationList: RscListNBox {
            idc = 1100;
            PLACEHOLDER;
            columns[] = {0, 0.6};
            drawSideArrows = 0;
            idcLeft = -1;
            idcRight = -1;
            colorBackground[] = COLOR_FIELD;
            colorSelectBackground[] = COLOR_ACCENT;
            colorSelectBackground2[] = COLOR_ACCENT;
            onLBSelChanged = "_this call skw_ui_fnc_onStationSelected";
        };
        // Switches between the station list and the last tracks of this radio.
        class HistoryToggle: skw_ui_clickable {
            idc = 1714;
            PLACEHOLDER;
            style = 1;
            colorText[] = COLOR_ACCENT;
            tooltip = "$STR_skw_ui_historyTooltip";
            onButtonClick = "call skw_ui_fnc_onHistory";
        };
        class HistoryList: RscListNBox {
            idc = 1105;
            PLACEHOLDER;
            columns[] = {0};
            drawSideArrows = 0;
            idcLeft = -1;
            idcRight = -1;
            colorBackground[] = COLOR_FIELD;
            colorSelectBackground[] = COLOR_ACCENT;
            colorSelectBackground2[] = COLOR_ACCENT;
            onLBSelChanged = "_this call skw_ui_fnc_onHistorySelected";
        };
        // Star next to the station name: adds the station to the favourites.
        class Favourite: skw_ui_clickable {
            idc = 1207;
            PLACEHOLDER;
            onButtonClick = "call skw_ui_fnc_onFavourite";
        };
        class UrlLabel: RscText {
            idc = 1502;
            PLACEHOLDER;
            text = "$STR_skw_ui_customUrl";
            colorText[] = COLOR_MUTED;
        };
        class Url: RscEdit {
            idc = 1500;
            PLACEHOLDER;
        };
        class UrlPlay: skw_ui_button {
            idc = 1501;
            PLACEHOLDER;
            text = "$STR_skw_ui_playUrl";
            onButtonClick = "call skw_ui_fnc_onPlayUrl";
        };
        class Manage: skw_ui_button {
            idc = 1507;
            PLACEHOLDER;
            text = "$STR_skw_ui_manage";
            onButtonClick = "'personal' call skw_ui_fnc_managerShow";
        };
        // Invisible handles on the right and bottom edge: drag to make the window wider or taller.
        class EdgeRight: skw_ui_clickable {
            idc = 1622;
            PLACEHOLDER;
        };
        class EdgeBottom: skw_ui_clickable {
            idc = 1623;
            PLACEHOLDER;
        };
        // Invisible handle over the grip: drag to make the window wider and taller at once.
        class Grip: skw_ui_clickable {
            idc = 1620;
            PLACEHOLDER;
            tooltip = "$STR_skw_ui_resizeHint";
        };
    };
};
