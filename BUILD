load("@crate_index//:defs.bzl", "aliases", "all_crate_deps")
load("@rules_rust//rust:defs.bzl", "rust_binary")

rust_binary(
    name = "music-player",
    srcs = [
      "src/event/events.rs",
      "src/event/key.rs",
      "src/event/mod.rs",
      "src/handlers/album_tracks.rs",
      "src/handlers/library.rs",
      "src/handlers/albums.rs",
      "src/handlers/mod.rs",
      "src/handlers/artist_tracks.rs",
      "src/handlers/play_queue.rs",
      "src/handlers/artists.rs",
      "src/handlers/playbar.rs",
      "src/handlers/common_key_events.rs",
      "src/handlers/playlist.rs",
      "src/handlers/empty.rs",
      "src/handlers/tracks.rs",
      "src/handlers/input.rs",
      "src/ui/mod.rs",
      "src/ui/util.rs",
      "src/app.rs",
      "src/main.rs",
      "src/scan.rs",
      "src/args.rs",
      "src/network.rs",
      "src/user_config.rs",
    ],
    deps = [
      "//addons:music_player_addons",
      "//client:music_player_client",
      "//discovery:music_player_discovery",
      "//entity:music_player_entity",
      "//graphql:music_player_graphql",
      "//migration:migration",
      "//playback:music_player_playback",
      "//scanner:music_player_scanner",
      "//server:music_player_server",
      "//settings:music_player_settings",
      "//storage:music_player_storage",
      "//tracklist:music_player_tracklist",
      "//types:music_player_types",
      "//webui:music_player_webui",
    ] + all_crate_deps(),
)