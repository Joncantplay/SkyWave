class CfgPatches {
    class skw_ui {
        name = "SkyWave - User Interface";
        author = "Joncantplay";
        url = "https://github.com/Joncantplay/SkyWave";
        units[] = {};
        weapons[] = {};
        // 2.20 brought the browser control used to show station logos.
        requiredVersion = 2.20;
        requiredAddons[] = {"skw_audio", "A3_Ui_F"};
    };
};

class Extended_PreInit_EventHandlers {
    class skw_ui {
        init = "call compileScript ['\z\skw\addons\ui\XEH_preInit.sqf']";
    };
};

class Extended_PostInit_EventHandlers {
    class skw_ui {
        init = "call compileScript ['\z\skw\addons\ui\XEH_postInit.sqf']";
    };
};

class CfgFunctions {
    class skw_ui {
        tag = "skw_ui";
        class ui {
            file = "\z\skw\addons\ui\functions";
            class actionTarget {};
            class addAction {};
            class canOpen {};
            class copyTitle {};
            class defaultWindow {};
            class fillStations {};
            class keyStation {};
            class keyVolume {};
            class layout {};
            class listStations {};
            class managerClose {};
            class managerDelete {};
            class managerFill {};
            class managerNew {};
            class managerSave {};
            class managerSelect {};
            class managerShow {};
            class managerWrite {};
            class onClose {};
            class onDrag {};
            class onFavourite {};
            class onFreeOnly {};
            class onHistory {};
            class onHistorySelected {};
            class onMute {};
            class onPlayUrl {};
            class onPower {};
            class onRandom {};
            class onRetry {};
            class onSearch {};
            class onSort {};
            class onStationSelected {};
            class onVolumeChanged {};
            class open {};
            class openForPlayer {};
            class placeSortArrow {};
            class playRecord {};
            class refresh {};
            class resetWindow {};
            class showListMode {};
            class startDrag {};
            class stopDrag {};
        };
    };
};

// Layout grid: 40 x 25 cells, centred, same proportions as the vanilla UI grid.
#define GRID_W (((safezoneW / safezoneH) min 1.2) / 40)
#define GRID_H ((((safezoneW / safezoneH) min 1.2) / 1.2) / 25)
#define GRID_X (safezoneX + (safezoneW - ((safezoneW / safezoneH) min 1.2)) / 2)
#define GRID_Y (safezoneY + (safezoneH - (((safezoneW / safezoneH) min 1.2) / 1.2)) / 2)
// Positions are expressions the game evaluates at runtime, so they are written as strings.
#define QUOTE(var) #var
#define POS_X(n) QUOTE(GRID_X + (n) * GRID_W)
#define POS_Y(n) QUOTE(GRID_Y + (n) * GRID_H)
#define SIZE_W(n) QUOTE((n) * GRID_W)
#define SIZE_H(n) QUOTE((n) * GRID_H)

#define ST_MULTI 16
#define COLOR_PANEL {0.06, 0.065, 0.075, 0.95}
#define COLOR_HEADER {0.1, 0.105, 0.12, 1}
#define COLOR_ACCENT {0.96, 0.58, 0.16, 1}
#define COLOR_CARD {0.13, 0.135, 0.15, 0.9}
#define COLOR_FIELD {0, 0, 0, 0.4}
#define COLOR_MUTED {0.62, 0.64, 0.68, 1}

class RscText;
class RscButton;
class RscEdit;
class RscPicture;
class RscListNBox;
class RscXSliderH;
class RscCheckBox;

class skw_ui_button: RscButton {
    colorBackground[] = {0.18, 0.185, 0.2, 1};
    colorBackgroundActive[] = COLOR_ACCENT;
    colorFocused[] = {0.24, 0.245, 0.26, 1};
    colorText[] = {1, 1, 1, 1};
    sizeEx = SIZE_H(0.9);
};

#include "radio.hpp"

#include "manager.hpp"
