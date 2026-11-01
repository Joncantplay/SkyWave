/*
 * Reads the setting with switched-off built-in stations into class names.
 *
 * Arguments:
 * 0: Setting text, class names separated by commas <STRING>
 *
 * Return: Class names <ARRAY of STRINGs>
 */
params [["_text", "", [""]]];

((_text splitString ",; ") apply {trim _x}) select {_x != ""}
