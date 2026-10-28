/*
 * Stops all radios in the extension and forgets their status.
 */
"skywave" callExtension ["reset", []];
skw_audio_playing = createHashMap;
skw_audio_status = createHashMap;
skw_audio_titles = createHashMap;
skw_audio_history = createHashMap;
