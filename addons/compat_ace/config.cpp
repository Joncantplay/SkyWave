class CfgPatches {
    class skw_compat_ace {
        name = "SkyWave - ACE compatibility";
        author = "Joncantplay";
        url = "https://github.com/Joncantplay/SkyWave";
        units[] = {};
        weapons[] = {};
        requiredVersion = 2.20;
        requiredAddons[] = {"skw_ui", "skw_objects", "ace_interact_menu"};
        // Loaded only when ACE is present.
        skipWhenMissingDependencies = 1;
    };
};

class Extended_PostInit_EventHandlers {
    class skw_compat_ace {
        init = "call compileScript ['\z\skw\addons\compat_ace\XEH_postInit.sqf']";
    };
};
