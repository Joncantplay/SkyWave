/*
 * Returns where the player hears from: the active camera (first person, third person,
 * Zeus or spectator).
 *
 * Return: [position ASL, forward vector, up vector]
 */
private _origin = positionCameraToWorld [0, 0, 0];
private _forward = (positionCameraToWorld [0, 0, 1]) vectorDiff _origin;
private _up = (positionCameraToWorld [0, 1, 0]) vectorDiff _origin;

[AGLToASL _origin, _forward, _up]
