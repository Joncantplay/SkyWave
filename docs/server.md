# Running SkyWave on a server

## Installing

1. Install SkyWave and CBA_A3 on the server like any other mod (`-mod=@CBA_A3;@skywave`).
2. Copy `skywave.bikey` from `@skywave\keys\` into the server's `keys` folder if signatures are
   checked (`verifySignatures = 2`). The key stays the same for every 1.x version, so it only has
   to be installed once.
3. Players should use the same SkyWave version as the server. Players with a different version
   can still join; they get a chat message naming both versions, since radios may then not fully
   work.

The dedicated server does not play audio and does not need the extension; it only keeps the
radios' state. Headless clients work the same way.

## BattlEye

SkyWave's audio runs in `skywave_x64.dll` on each player's PC. With BattlEye enabled, Arma
only loads extensions that BattlEye has whitelisted. Until SkyWave's DLL is whitelisted:

- players who start Arma with BattlEye hear no radio (they get a short message once);
- everything else (other players, Zeus, the server) keeps working.

Every new version of the DLL needs to be whitelisted again. The current procedure is described
on BattlEye's website.

## Recommended settings

Server settings are in *Addon Options → SkyWave* on the **Server** tab (as admin or host), or in
the mission's / server's CBA settings file:

| Setting | Recommendation |
|---|---|
| Allow custom stream URLs | Turn off on public servers so players can only pick listed stations. |
| Allow images for custom streams | Turn off on public servers: a stream run by a player could otherwise show any picture to everyone listening. |
| Radio only with engine running | Turn on for more realism; placed radios keep playing. |
| Copyright-free stations only | Turn on for servers that stream or record gameplay. |
| Operate from outside / Driver and commander only | Depends on your group's play style. |
| Server stations | Your own station list, see [stations.md](stations.md). |
| SkyWave default stations | Turn off to offer only your own stations. |

See [settings.md](settings.md) for all settings.

## Remote execution whitelist

SkyWave allows its two network functions in `CfgRemoteExec`: `skw_core_fnc_handleRequest`
(clients → server) and `skw_core_fnc_onRejected` (server → client). Missions that define their
own `CfgRemoteExec` must allow them as well.
