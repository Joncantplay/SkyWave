class CfgPatches {
    class skw_stations {
        name = "SkyWave - Stations";
        author = "Joncantplay";
        url = "https://github.com/Joncantplay/SkyWave";
        units[] = {};
        weapons[] = {};
        requiredVersion = 2.20;
        requiredAddons[] = {"skw_core"};
    };
};

// Built-in stations, listed alphabetically in the game. Other mods can add classes here;
// logos are found automatically unless a class sets logo = "image URL, web page or .paa path".
class CfgSkywaveStations {
    // Copyright-free: stay audible in Streamer Mode.
    class skw_ltbr {
        displayName = "LTBR.FM";
        url = "https://stream.ltbr.fm/live";
        fallbackUrls[] = {
            "http://stream.ltbr.fm/live"
        };
        genre = "Eclectic";
        description = "London Tower Block Radio";
        copyrightFree = 1;
        skw_default = 1;
    };
    class skw_ltbr_nodj {
        displayName = "LTBR.FM (no DJ)";
        url = "https://stream.ltbr.fm/live-nodj";
        fallbackUrls[] = {
            "http://stream.ltbr.fm/live-nodj"
        };
        genre = "Eclectic";
        description = "London Tower Block Radio without presenters";
        copyrightFree = 1;
        skw_default = 1;
    };
    class skw_radiomast {
        displayName = "Radio Mast";
        url = "https://streams.radiomast.io/ref-128k-aaclc-stereo";
        fallbackUrls[] = {
            "https://streams.radiomast.io/ref-128k-mp3-stereo"
        };
        genre = "Eclectic";
        copyrightFree = 1;
        skw_default = 1;
    };
    class skw_ncs {
        displayName = "NCS Radio";
        url = "https://stream.laut.fm/ncsradio";
        fallbackUrls[] = {
            "https://ncsradio.stream.laut.fm/ncsradio"
        };
        genre = "Electronic";
        description = "NoCopyrightSounds";
        copyrightFree = 1;
        skw_default = 1;
    };

    // Dance & electronic
    class skw_pulseedm {
        displayName = "PulseEDM";
        url = "http://pulseedm.cdnstream1.com:8124/1373_128";
        genre = "Dance";
        skw_default = 1;
    };
    class skw_technobase {
        displayName = "TechnoBase.FM";
        url = "https://mp3.stream.tb-group.fm/tb.mp3";
        fallbackUrls[] = {
            "https://listener3.mp3.tb-group.fm/tb.mp3"
        };
        genre = "Techno";
        skw_default = 1;
    };
    class skw_hardbase {
        displayName = "HardBase.FM";
        url = "https://mp3.stream.tb-group.fm/hb.mp3";
        fallbackUrls[] = {
            "https://listener1.mp3.tb-group.fm/hb.mp3"
        };
        genre = "Hardstyle";
        skw_default = 1;
    };
    class skw_remixfm {
        displayName = "RemixFM";
        url = "https://stream.laut.fm/remixfm";
        fallbackUrls[] = {
            "https://remixfm.stream.laut.fm/remixfm"
        };
        genre = "Dance";
        skw_default = 1;
    };

    // Chill
    class skw_somafm_groovesalad {
        displayName = "SomaFM Groove Salad";
        url = "https://ice5.somafm.com/groovesalad-128-mp3";
        fallbackUrls[] = {
            "https://ice4.somafm.com/groovesalad-128-mp3"
        };
        genre = "Chill";
        skw_default = 1;
    };
    class skw_somafm_lush {
        displayName = "SomaFM Lush";
        url = "https://ice5.somafm.com/lush-128-mp3";
        fallbackUrls[] = {
            "https://ice4.somafm.com/lush-128-mp3"
        };
        genre = "Chill";
        skw_default = 1;
    };
    class skw_lofi {
        displayName = "Lofi";
        url = "https://stream.laut.fm/lofi";
        fallbackUrls[] = {
            "https://lofi.stream.laut.fm/lofi"
        };
        genre = "Lo-fi";
        skw_default = 1;
    };

    class skw_chilloutfm {
        displayName = "Chill Out FM";
        url = "https://stream.laut.fm/chill-out-fm";
        fallbackUrls[] = {
            "https://chill-out-fm.stream.laut.fm/chill-out-fm"
        };
        genre = "Chill";
        skw_default = 1;
    };

    // Anime & K-Pop
    class skw_animealive {
        displayName = "Anime Alive";
        url = "https://stream.laut.fm/animealive";
        fallbackUrls[] = {
            "https://animealive.stream.laut.fm/animealive"
        };
        genre = "Electronic";
        skw_default = 1;
    };
    class skw_kpop4you {
        displayName = "KPOP4YOU";
        url = "https://stream.laut.fm/kpop4you";
        fallbackUrls[] = {
            "https://kpop4you.stream.laut.fm/kpop4you"
        };
        genre = "K-Pop";
        skw_default = 1;
    };

    // Rock & oldies
    class skw_181_rock {
        displayName = "Rock 181 - 181.FM";
        url = "https://listen.181fm.com/181-rock_128k.mp3";
        fallbackUrls[] = {
            "https://listen.181fm.com/181-rock_64k.aac"
        };
        genre = "Rock";
        skw_default = 1;
    };
    class skw_181_80s {
        displayName = "Awesome 80s - 181.FM";
        url = "https://listen.181fm.com/181-awesome80s_128k.mp3";
        fallbackUrls[] = {
            "https://listen.181fm.com/181-awesome80s_64k.aac"
        };
        genre = "Oldies";
        skw_default = 1;
    };
    class skw_181_90salt {
        displayName = "90s Alternative - 181.FM";
        url = "https://listen.181fm.com/181-90salt_128k.mp3";
        fallbackUrls[] = {
            "https://listen.181fm.com/181-90salt_64k.aac"
        };
        genre = "Alternative";
        skw_default = 1;
    };
    class skw_bob_90srock {
        displayName = "90s Rock - RADIO BOB!";
        url = "https://streams.radiobob.de/bob-90srock/mp3-192/mediaplayer";
        fallbackUrls[] = {
            "https://streams.radiobob.de/bob-90srock/mp3-128/mediaplayer"
        };
        genre = "Rock";
        logo = "https://www.radiobob.de/";
        skw_default = 1;
    };

    // Classical
    class skw_classicfm {
        displayName = "Classic FM";
        url = "https://media-ice.musicradio.com/ClassicFMMP3";
        fallbackUrls[] = {
            "https://media-ssl.musicradio.com/ClassicFMMP3"
        };
        genre = "Classical";
        logo = "https://www.classicfm.com/";
        skw_default = 1;
    };
    class skw_swissclassic {
        displayName = "Radio Swiss Classic";
        url = "https://stream.srg-ssr.ch/m/rsc_de/mp3_128";
        fallbackUrls[] = {
            "https://stream.srg-ssr.ch/m/rsc_de/aacp_96"
        };
        genre = "Classical";
        logo = "https://www.radioswissclassic.ch/";
        skw_default = 1;
    };

    // Urban & country
    class skw_181_rnb {
        displayName = "True R&B - 181.FM";
        url = "https://listen.181fm.com/181-rnb_128k.mp3";
        fallbackUrls[] = {
            "https://listen.181fm.com/181-rnb_64k.aac"
        };
        genre = "R&B";
        skw_default = 1;
    };
    class skw_181_country {
        displayName = "Kickin' Country - 181.FM";
        url = "https://listen.181fm.com/181-kickincountry_128k.mp3";
        fallbackUrls[] = {
            "https://listen.181fm.com/181-kickincountry_64k.aac"
        };
        genre = "Country";
        skw_default = 1;
    };

    // Eclectic & world
    class skw_radioparadise {
        displayName = "Radio Paradise";
        url = "https://stream.radioparadise.com/aac-320";
        fallbackUrls[] = {
            "https://stream-uk1.radioparadise.com/aac-320",
            "https://stream.radioparadise.com/mp3-192"
        };
        genre = "Eclectic";
        skw_default = 1;
    };
    class skw_fip {
        displayName = "FIP";
        url = "https://stream.radiofrance.fr/fip/fip.m3u8?id=radiofrance";
        fallbackUrls[] = {
            "https://icecast.radiofrance.fr/fip-hifi.aac"
        };
        genre = "Eclectic";
        logo = "https://www.radiofrance.fr/fip";
        skw_default = 1;
    };
    class skw_bbc_worldservice {
        displayName = "BBC World Service";
        url = "https://a.files.bbci.co.uk/ms6/live/3441A116-B12E-4D2F-ACA8-C1984642FA4B/audio/simulcast/hls/nonuk/pc_hd_abr_v2/ak/bbc_world_service.m3u8";
        fallbackUrls[] = {
            "https://a.files.bbci.co.uk/ms6/live/3441A116-B12E-4D2F-ACA8-C1984642FA4B/audio/simulcast/hls/nonuk/pc_hd_abr_v2/cf/bbc_world_service.m3u8",
            "https://stream.live.vc.bbcmedia.co.uk/bbc_world_service"
        };
        genre = "News";
        logo = "https://www.bbc.co.uk/sounds";
        skw_default = 1;
    };

    // More dance & electronic
    class skw_181_energy98 {
        displayName = "Energy 98 - 181.FM";
        url = "https://listen.181fm.com/181-energy98_128k.mp3";
        fallbackUrls[] = {
            "https://listen.181fm.com/181-energy98_64k.aac"
        };
        genre = "Dance";
        skw_default = 1;
    };
    class skw_181_energy93 {
        displayName = "Energy 93 - 181.FM";
        url = "https://listen.181fm.com/181-energy93_128k.mp3";
        fallbackUrls[] = {
            "https://listen.181fm.com/181-energy93_64k.aac"
        };
        genre = "Dance";
        skw_default = 1;
    };
    class skw_phonk {
        displayName = "Phonk";
        url = "https://stream.laut.fm/phonk";
        fallbackUrls[] = {
            "https://phonk.stream.laut.fm/phonk"
        };
        genre = "Hip-hop";
        skw_default = 1;
    };
    class skw_tca {
        displayName = "TCA Radio";
        url = "https://stream.laut.fm/tca";
        fallbackUrls[] = {
            "https://tca.stream.laut.fm/tca"
        };
        genre = "Eclectic";
        skw_default = 1;
    };
    class skw_happyfm {
        displayName = "Happy FM";
        url = "https://stream.laut.fm/happyfm";
        fallbackUrls[] = {
            "https://happyfm.stream.laut.fm/happyfm"
        };
        genre = "Pop";
        skw_default = 1;
    };
    class skw_radiogalaxymix {
        displayName = "Radio Galaxy Mix";
        url = "https://stream.laut.fm/radiogalaxymix";
        fallbackUrls[] = {
            "https://radiogalaxymix.stream.laut.fm/radiogalaxymix"
        };
        genre = "Dance";
        skw_default = 1;
    };

    // More chill
    class skw_somafm_deepspaceone {
        displayName = "SomaFM Deep Space One";
        url = "https://ice5.somafm.com/deepspaceone-128-mp3";
        fallbackUrls[] = {
            "https://ice4.somafm.com/deepspaceone-128-mp3"
        };
        genre = "Chill";
        skw_default = 1;
    };
    class skw_somafm_dronezone {
        displayName = "SomaFM Drone Zone";
        url = "https://ice5.somafm.com/dronezone-128-mp3";
        fallbackUrls[] = {
            "https://ice4.somafm.com/dronezone-128-mp3"
        };
        genre = "Chill";
        skw_default = 1;
    };
    class skw_somafm_defcon {
        displayName = "SomaFM DEF CON Radio";
        url = "https://ice5.somafm.com/defcon-128-mp3";
        fallbackUrls[] = {
            "https://ice4.somafm.com/defcon-128-mp3"
        };
        genre = "Electronic";
        skw_default = 1;
    };
    class skw_beatzarea_chill {
        displayName = "BeatzArea Chill";
        url = "https://stream.laut.fm/beatzarea-chill";
        fallbackUrls[] = {
            "https://beatzarea-chill.stream.laut.fm/beatzarea-chill"
        };
        genre = "Chill";
        skw_default = 1;
    };
    class skw_181_chilled {
        displayName = "Chilled Out - 181.FM";
        url = "https://listen.181fm.com/181-chilled_128k.mp3";
        fallbackUrls[] = {
            "https://listen.181fm.com/181-chilled_64k.aac"
        };
        genre = "Chill";
        skw_default = 1;
    };

    // Pop & party
    class skw_181_party {
        displayName = "Party 181 - 181.FM";
        url = "https://listen.181fm.com/181-party_128k.mp3";
        fallbackUrls[] = {
            "https://listen.181fm.com/181-party_64k.aac"
        };
        genre = "Pop";
        skw_default = 1;
    };
    class skw_181_power {
        displayName = "Power 181 - 181.FM";
        url = "https://listen.181fm.com/181-power_128k.mp3";
        fallbackUrls[] = {
            "https://listen.181fm.com/181-power_64k.aac"
        };
        genre = "Pop";
        skw_default = 1;
    };
    class skw_181_themix {
        displayName = "The Mix - 181.FM";
        url = "https://listen.181fm.com/181-themix_128k.mp3";
        fallbackUrls[] = {
            "https://listen.181fm.com/181-themix_64k.aac"
        };
        genre = "Pop";
        skw_default = 1;
    };
    class skw_181_heart {
        displayName = "The Heart - 181.FM";
        url = "https://listen.181fm.com/181-heart_128k.mp3";
        fallbackUrls[] = {
            "https://listen.181fm.com/181-heart_64k.aac"
        };
        genre = "Pop";
        skw_default = 1;
    };
    class skw_181_buzz {
        displayName = "The Buzz - 181.FM";
        url = "https://listen.181fm.com/181-buzz_128k.mp3";
        fallbackUrls[] = {
            "https://listen.181fm.com/181-buzz_64k.aac"
        };
        genre = "Alternative";
        skw_default = 1;
    };
    class skw_181_star90s {
        displayName = "Star 90s - 181.FM";
        url = "https://listen.181fm.com/181-star90s_128k.mp3";
        fallbackUrls[] = {
            "https://listen.181fm.com/181-star90s_64k.aac"
        };
        genre = "Pop";
        skw_default = 1;
    };
    class skw_1000club {
        displayName = "Club 1000";
        url = "https://stream.laut.fm/1000club";
        fallbackUrls[] = {
            "https://1000club.stream.laut.fm/1000club"
        };
        genre = "Pop";
        skw_default = 1;
    };
    class skw_justplay {
        displayName = "Justplay";
        url = "https://stream.laut.fm/justplay";
        fallbackUrls[] = {
            "https://justplay.stream.laut.fm/justplay"
        };
        genre = "J-Pop";
        skw_default = 1;
    };
    class skw_antenneviersen {
        displayName = "Antenne Viersen";
        url = "https://stream.laut.fm/antenne-viersen";
        fallbackUrls[] = {
            "https://antenne-viersen.stream.laut.fm/antenne-viersen"
        };
        genre = "Pop";
        skw_default = 1;
    };
    class skw_eurovision {
        displayName = "Eurovision Radio";
        url = "https://stream.laut.fm/eurovisonradio";
        fallbackUrls[] = {
            "https://eurovisonradio.stream.laut.fm/eurovisonradio"
        };
        genre = "Pop";
        skw_default = 1;
    };
    class skw_querbeat {
        displayName = "Querbeat";
        url = "https://stream.laut.fm/querbeat";
        fallbackUrls[] = {
            "https://querbeat.stream.laut.fm/querbeat"
        };
        genre = "Schlager";
        skw_default = 1;
    };
    class skw_schlager {
        displayName = "Schlager";
        url = "https://stream.laut.fm/schlager";
        fallbackUrls[] = {
            "https://schlager.stream.laut.fm/schlager"
        };
        genre = "Schlager";
        skw_default = 1;
    };
    class skw_traumbaerchen {
        displayName = "Traumbaerchen Radio";
        url = "https://stream.laut.fm/traumbaerchen-radio";
        fallbackUrls[] = {
            "https://traumbaerchen-radio.stream.laut.fm/traumbaerchen-radio"
        };
        genre = "Schlager";
        skw_default = 1;
    };
    class skw_ffm03 {
        displayName = "FFM03";
        url = "https://stream.laut.fm/ffm03";
        fallbackUrls[] = {
            "https://ffm03.stream.laut.fm/ffm03"
        };
        genre = "Dance";
        skw_default = 1;
    };
    class skw_music4gamers {
        displayName = "Music4Gamers";
        url = "https://stream.laut.fm/music4gamers";
        fallbackUrls[] = {
            "https://music4gamers.stream.laut.fm/music4gamers"
        };
        genre = "Dance";
        skw_default = 1;
    };
    class skw_joshysradio {
        displayName = "Joshy's Radio";
        url = "https://stream.laut.fm/joshysradio";
        fallbackUrls[] = {
            "https://joshysradio.stream.laut.fm/joshysradio"
        };
        genre = "Eclectic";
        skw_default = 1;
    };

    // More rock & oldies
    class skw_classicrock109 {
        displayName = "Classic Rock 109";
        url = "http://listen.classicrock109.com:10042/;";
        genre = "Rock";
        skw_default = 1;
    };
    class skw_181_rock40 {
        displayName = "Rock 40 - 181.FM";
        url = "https://listen.181fm.com/181-rock40_128k.mp3";
        fallbackUrls[] = {
            "https://listen.181fm.com/181-rock40_64k.aac"
        };
        genre = "Rock";
        skw_default = 1;
    };
    class skw_radioeuphoria {
        displayName = "Radio Euphoria";
        url = "https://stream.laut.fm/radioeuphoria";
        fallbackUrls[] = {
            "https://radioeuphoria.stream.laut.fm/radioeuphoria"
        };
        genre = "Alternative";
        skw_default = 1;
    };
    class skw_emoscene {
        displayName = "Emo Scene";
        url = "https://stream.laut.fm/emoscene";
        fallbackUrls[] = {
            "https://emoscene.stream.laut.fm/emoscene"
        };
        genre = "Alternative";
        skw_default = 1;
    };
    class skw_181_classichits {
        displayName = "Classic Hits - 181.FM";
        url = "https://listen.181fm.com/181-greatoldies_128k.mp3";
        fallbackUrls[] = {
            "https://listen.181fm.com/181-greatoldies_64k.aac"
        };
        genre = "Oldies";
        skw_default = 1;
    };
    class skw_181_70s {
        displayName = "Super 70s - 181.FM";
        url = "https://listen.181fm.com/181-70s_128k.mp3";
        fallbackUrls[] = {
            "https://listen.181fm.com/181-70s_64k.aac"
        };
        genre = "Oldies";
        skw_default = 1;
    };

    // Hip-hop & R&B
    class skw_181_thebox {
        displayName = "The Box - 181.FM";
        url = "https://listen.181fm.com/181-thebox_128k.mp3";
        fallbackUrls[] = {
            "https://listen.181fm.com/181-thebox_64k.aac"
        };
        genre = "Hip-hop";
        skw_default = 1;
    };
    class skw_181_beat {
        displayName = "The Beat - 181.FM";
        url = "https://listen.181fm.com/181-beat_128k.mp3";
        fallbackUrls[] = {
            "https://listen.181fm.com/181-beat_64k.aac"
        };
        genre = "Hip-hop";
        skw_default = 1;
    };
    class skw_181_oldschool {
        displayName = "Old School HipHop/R&B - 181.FM";
        url = "https://listen.181fm.com/181-oldschool_128k.mp3";
        fallbackUrls[] = {
            "https://listen.181fm.com/181-oldschool_64k.aac"
        };
        genre = "Hip-hop";
        skw_default = 1;
    };
    class skw_181_jammin {
        displayName = "Jammin' 181 - 181.FM";
        url = "https://listen.181fm.com/181-jammin_128k.mp3";
        fallbackUrls[] = {
            "https://listen.181fm.com/181-jammin_64k.aac"
        };
        genre = "Hip-hop";
        skw_default = 1;
    };
    class skw_181_soul {
        displayName = "Soul - 181.FM";
        url = "https://listen.181fm.com/181-soul_128k.mp3";
        fallbackUrls[] = {
            "https://listen.181fm.com/181-soul_64k.aac"
        };
        genre = "R&B";
        skw_default = 1;
    };
    class skw_hiphopforever {
        displayName = "Hiphop Forever";
        url = "https://stream.laut.fm/hiphop-forever";
        fallbackUrls[] = {
            "https://hiphop-forever.stream.laut.fm/hiphop-forever"
        };
        genre = "Hip-hop";
        skw_default = 1;
    };
    class skw_eminemradio {
        displayName = "Eminem Radio";
        url = "https://stream.laut.fm/eminem-radio";
        fallbackUrls[] = {
            "https://eminem-radio.stream.laut.fm/eminem-radio"
        };
        genre = "Hip-hop";
        skw_default = 1;
    };

    // More country & world
    class skw_181_frontporch {
        displayName = "Front Porch - 181.FM";
        url = "https://listen.181fm.com/181-frontporch_128k.mp3";
        fallbackUrls[] = {
            "https://listen.181fm.com/181-frontporch_64k.aac"
        };
        genre = "Country";
        skw_default = 1;
    };
    class skw_liveireland {
        displayName = "Live Ireland";
        url = "http://lynx.prostreaming.net:8058/stream?type=http&nocache=325927";
        genre = "Folk";
        skw_default = 1;
    };

    // Russian
    class skw_nasheradio {
        displayName = "NASHE Radio";
        url = "https://nashe1.hostingradio.ru/nashe-256";
        fallbackUrls[] = {
            "https://nashe1.hostingradio.ru/nashe-128.mp3"
        };
        genre = "Russian";
        skw_default = 1;
    };
    class skw_russkoeradio {
        displayName = "Russkoe Radio";
        url = "https://rusradio.hostingradio.ru/rusradio128.mp3";
        fallbackUrls[] = {
            "https://rusradio.hostingradio.ru/rusradio96.aacp"
        };
        genre = "Russian";
        skw_default = 1;
    };
    class skw_record_russianmix {
        displayName = "Record Russian Mix";
        url = "https://radiorecord.hostingradio.ru/rus96.aacp";
        genre = "Russian";
        skw_default = 1;
    };
    class skw_grazfm {
        displayName = "GRAZ FM";
        url = "https://stream.laut.fm/russianradiograzfm";
        fallbackUrls[] = {
            "https://russianradiograzfm.stream.laut.fm/russianradiograzfm"
        };
        genre = "Russian";
        skw_default = 1;
    };
};
