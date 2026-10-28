/*
 * Shown to a player whose radio request the server refused.
 *
 * Arguments:
 * 0: Stringtable key with the reason <STRING>
 */
params [["_key", "", [""]]];

if (!hasInterface || {!isLocalized _key}) exitWith {};
hint localize _key;
