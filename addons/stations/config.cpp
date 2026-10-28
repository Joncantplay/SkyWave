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
        genre = "Various";
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
        genre = "Various";
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
        genre = "Royalty-free";
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
        genre = "EDM";
        skw_default = 1;
    };
    class skw_technobase {
        displayName = "TechnoBase.FM";
        url = "https://mp3.stream.tb-group.fm/tb.mp3";
        fallbackUrls[] = {
            "https://listener3.mp3.tb-group.fm/tb.mp3"
        };
        genre = "Techno / Dance";
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
        genre = "Remixes";
        skw_default = 1;
    };

    // Chill
    class skw_somafm_groovesalad {
        displayName = "SomaFM Groove Salad";
        url = "https://ice5.somafm.com/groovesalad-128-mp3";
        fallbackUrls[] = {
            "https://ice4.somafm.com/groovesalad-128-mp3"
        };
        genre = "Ambient / Downtempo";
        skw_default = 1;
    };
    class skw_somafm_lush {
        displayName = "SomaFM Lush";
        url = "https://ice5.somafm.com/lush-128-mp3";
        fallbackUrls[] = {
            "https://ice4.somafm.com/lush-128-mp3"
        };
        genre = "Vocal chillout";
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
        genre = "Chillout";
        skw_default = 1;
    };

    // Anime & K-Pop
    class skw_animealive {
        displayName = "Anime Alive";
        url = "https://stream.laut.fm/animealive";
        fallbackUrls[] = {
            "https://animealive.stream.laut.fm/animealive"
        };
        genre = "Anime / J-Pop";
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
        genre = "80s";
        skw_default = 1;
    };
    class skw_181_90salt {
        displayName = "90s Alternative - 181.FM";
        url = "https://listen.181fm.com/181-90salt_128k.mp3";
        fallbackUrls[] = {
            "https://listen.181fm.com/181-90salt_64k.aac"
        };
        genre = "90s Rock / Alternative";
        skw_default = 1;
    };
    class skw_bob_90srock {
        displayName = "90s Rock - RADIO BOB!";
        url = "https://streams.radiobob.de/bob-90srock/mp3-192/mediaplayer";
        fallbackUrls[] = {
            "https://streams.radiobob.de/bob-90srock/mp3-128/mediaplayer"
        };
        genre = "90s Rock";
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
        genre = "Eclectic / Jazz";
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
};
