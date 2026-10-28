#include "script_version.hpp"

#define QUOTE(var) #var
#define EXPAND_QUOTE(var) QUOTE(var)

class CfgPatches {
    class skw_main {
        name = "SkyWave";
        author = "Joncantplay";
        url = "https://github.com/Joncantplay/SkyWave";
        units[] = {};
        weapons[] = {};
        requiredVersion = 2.20;
        requiredAddons[] = {"cba_main"};
        versionStr = EXPAND_QUOTE(MAJOR.MINOR.PATCH.BUILD);
        versionAr[] = {MAJOR, MINOR, PATCH, BUILD};
    };
};
