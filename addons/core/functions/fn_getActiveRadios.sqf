/*
 * Returns all objects whose radio is switched on (maintained by the server).
 *
 * Return: <ARRAY of OBJECTs>
 */
skw_core_radios select {!isNull _x}
