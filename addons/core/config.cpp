class CfgPatches {
    class skw_core {
        name = "SkyWave - Core";
        author = "Joncantplay";
        url = "https://github.com/Joncantplay/SkyWave";
        units[] = {};
        weapons[] = {};
        requiredVersion = 2.20;
        requiredAddons[] = {"skw_main"};
    };
};

class Extended_PreInit_EventHandlers {
    class skw_core {
        init = "call compileScript ['\z\skw\addons\core\XEH_preInit.sqf']";
    };
};

class Extended_PostInit_EventHandlers {
    class skw_core {
        init = "call compileScript ['\z\skw\addons\core\XEH_postInit.sqf']";
    };
};

// Allows radio requests on servers that whitelist remote execution.
class CfgRemoteExec {
    class Functions {
        class skw_core_fnc_handleRequest {
            allowedTargets = 2;
        };
        class skw_core_fnc_onRejected {
            allowedTargets = 1;
        };
    };
};

class CfgFunctions {
    class skw_core {
        tag = "skw_core";
        class core {
            file = "\z\skw\addons\core\functions";
            class addStation {};
            class allStations {};
            class applyState {};
            class canControl {};
            class getActiveRadios {};
            class getState {};
            class getStation {};
            class getStations {};
            class handleRequest {};
            class isCopyrightFree {};
            class isRadioCapable {};
            class isStationAvailable {};
            class makeStation {};
            class onRejected {};
            class parseStationList {};
            class readStationConfig {};
            class removeStation {};
            class request {};
            class setRadioEnabled {};
            class setStation {};
            class setVolume {};
            class stationListEntries {};
            class stationListText {};
            class stationUrl {};
            class storeSetting {};
            class syncScriptStations {};
            class turnOff {};
        };
    };
};
