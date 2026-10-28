class CfgPatches {
    class skw_objects {
        name = "SkyWave - Objects";
        author = "Joncantplay";
        url = "https://github.com/Joncantplay/SkyWave";
        units[] = {"skw_radio"};
        weapons[] = {};
        requiredVersion = 2.20;
        requiredAddons[] = {"skw_core", "A3_Structures_F_Items_Electronics", "3DEN"};
    };
};

class CfgEditorSubcategories {
    class skw_radios {
        displayName = "SkyWave";
    };
};

class CfgVehicles {
    class Land_FMradio_F;
    // A radio to place in missions or as Zeus, using the radio model from the base game.
    class skw_radio: Land_FMradio_F {
        scope = 2;
        scopeCurator = 2;
        displayName = "$STR_skw_objects_radio";
        editorCategory = "EdCat_Things";
        editorSubcategory = "skw_radios";
        skw_radio = 1;
    };
};

class CfgFunctions {
    class skw_objects {
        tag = "skw_objects";
        class objects {
            file = "\z\skw\addons\objects\functions";
            class edenStations {};
        };
    };
};

// Eden editor: starting station and volume for vehicles and radios.
class Cfg3DEN {
    class Attributes {
        class Combo;
        // Drop-down of the built-in stations (filled by script, so stations from other mods
        // show up too).
        class skw_stationCombo: Combo {
            attributeLoad = "[_this controlsGroupCtrl 100, _value] call skw_objects_fnc_edenStations";
            attributeSave = "private _combo = _this controlsGroupCtrl 100; _combo lbData lbCurSel _combo";
        };
    };
    class Object {
        class AttributeCategories {
            class skw_attributes {
                displayName = "SkyWave";
                collapsed = 1;
                class Attributes {
                    class skw_station {
                        displayName = "$STR_skw_objects_attributeStation";
                        tooltip = "$STR_skw_objects_attributeStation_tooltip";
                        property = "skw_station";
                        control = "skw_stationCombo";
                        typeName = "STRING";
                        defaultValue = "''";
                        // Next frame, so the volume attribute below has been stored already.
                        expression = "if (isServer && {_value != ''}) then {[{params ['_object', '_station']; [_object, _station, _object getVariable ['skw_objects_volume', 0.7]] call skw_core_fnc_applyState}, [_this, _value]] call CBA_fnc_execNextFrame}";
                        condition = "1";
                        validate = "none";
                        unique = 0;
                    };
                    class skw_volume {
                        displayName = "$STR_skw_objects_attributeVolume";
                        tooltip = "$STR_skw_objects_attributeVolume_tooltip";
                        property = "skw_volume";
                        control = "Slider";
                        typeName = "NUMBER";
                        defaultValue = "0.7";
                        expression = "_this setVariable ['skw_objects_volume', _value]";
                        condition = "1";
                        unique = 0;
                    };
                };
            };
        };
    };
};
