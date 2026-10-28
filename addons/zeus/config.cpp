class CfgPatches {
    class skw_zeus {
        name = "SkyWave - Zeus";
        author = "Joncantplay";
        url = "https://github.com/Joncantplay/SkyWave";
        units[] = {"skw_zeus_moduleRadio", "skw_zeus_moduleMakeRadio", "skw_zeus_moduleRemoveRadio", "skw_zeus_moduleZeusOnly"};
        weapons[] = {};
        requiredVersion = 2.20;
        requiredAddons[] = {"skw_ui", "skw_objects", "A3_Modules_F", "A3_Modules_F_Curator"};
    };
};

class CfgFactionClasses {
    class NO_CATEGORY;
    class skw_zeus: NO_CATEGORY {
        displayName = "SkyWave";
    };
};

class CfgVehicles {
    class Logic;
    class Module_F: Logic {};
    // Zeus places this on a vehicle or radio to open its radio window.
    class skw_zeus_moduleRadio: Module_F {
        scope = 1;
        scopeCurator = 2;
        displayName = "$STR_skw_zeus_moduleRadio";
        category = "skw_zeus";
        function = "skw_zeus_fnc_moduleRadio";
        functionPriority = 1;
        isGlobal = 1;
        isTriggerActivated = 0;
        curatorCanAttach = 1;
        icon = "\A3\ui_f\data\gui\Rsc\RscDisplayArsenal\radio_ca.paa";
        portrait = "\A3\ui_f\data\gui\Rsc\RscDisplayArsenal\radio_ca.paa";
    };
    // Zeus places this on any object to give it a radio.
    class skw_zeus_moduleMakeRadio: skw_zeus_moduleRadio {
        displayName = "$STR_skw_zeus_moduleMakeRadio";
        function = "skw_zeus_fnc_moduleMakeRadio";
    };
    // Zeus places this on a vehicle or object to remove its radio.
    class skw_zeus_moduleRemoveRadio: skw_zeus_moduleRadio {
        displayName = "$STR_skw_zeus_moduleRemoveRadio";
        function = "skw_zeus_fnc_moduleRemoveRadio";
    };
    // Zeus places this on a radio so only Zeus can operate it (again to unlock).
    class skw_zeus_moduleZeusOnly: skw_zeus_moduleRadio {
        displayName = "$STR_skw_zeus_moduleZeusOnly";
        function = "skw_zeus_fnc_moduleZeusOnly";
    };
};

class CfgFunctions {
    class skw_zeus {
        tag = "skw_zeus";
        class zeus {
            file = "\z\skw\addons\zeus\functions";
            class hint {};
            class moduleMakeRadio {};
            class moduleRadio {};
            class moduleRemoveRadio {};
            class moduleZeusOnly {};
        };
    };
};
