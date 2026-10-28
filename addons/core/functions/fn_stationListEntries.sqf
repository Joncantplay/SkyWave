/*
 * Splits the text of a station list setting into its entries.
 *
 * Format: stations separated by ";", fields separated by "|":
 *     Name | URL | option | option ...
 * Options: "free" (no copyrighted music), "genre=...", "logo=..." (image URL, web page or .paa path).
 *
 * Arguments:
 * 0: Setting text <STRING>
 *
 * Return: [[name, url, genre, logo, copyrightFree], ...]
 */
params [["_text", "", [""]]];

private _entries = [];
{
    private _fields = (_x splitString "|") apply {trim _x};
    if (count _fields >= 2 && {(_fields select 0) != ""} && {(_fields select 1) != ""}) then {
        _fields params ["_name", "_url"];
        private _genre = "";
        private _logo = "";
        private _free = false;
        {
            private _lower = toLower _x;
            switch (true) do {
                case (_lower == "free"): {_free = true};
                case ((_lower select [0, 6]) == "genre="): {_genre = trim (_x select [6, 1000])};
                case ((_lower select [0, 5]) == "logo="): {_logo = trim (_x select [5, 2000])};
            };
        } forEach (_fields select [2, 100]);
        _entries pushBack [_name, _url, _genre, _logo, _free];
    };
} forEach (_text splitString ";");
_entries
