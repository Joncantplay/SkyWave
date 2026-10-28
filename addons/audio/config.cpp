class CfgPatches {
    class skw_audio {
        name = "SkyWave - Audio";
        author = "Joncantplay";
        url = "https://github.com/Joncantplay/SkyWave";
        units[] = {};
        weapons[] = {};
        requiredVersion = 2.20;
        requiredAddons[] = {"skw_core"};
    };
};

class Extended_PreInit_EventHandlers {
    class skw_audio {
        init = "call compileScript ['\z\skw\addons\audio\XEH_preInit.sqf']";
    };
};

class Extended_PostInit_EventHandlers {
    class skw_audio {
        init = "call compileScript ['\z\skw\addons\audio\XEH_postInit.sqf']";
    };
};

class CfgFunctions {
    class skw_audio {
        tag = "skw_audio";
        class audio {
            file = "\z\skw\addons\audio\functions";
            class cabinSpeaker {};
            class cover {};
            class hint {};
            class isClosed {};
            class listener {};
            class logo {};
            class muteReason {};
            class notify {};
            class onCallback {};
            class reset {};
            class retry {};
            class setMuted {};
            class sourceId {};
            class toggleSetting {};
            class update {};
        };
    };
};
