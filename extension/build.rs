// Copyright (C) 2026 Joncantplay
// SPDX-License-Identifier: GPL-3.0-or-later
//! Builds FAAD2 (AAC / HE-AAC decoder) from the vendored submodule and links it statically.

use std::path::Path;

fn main() {
    let source = Path::new("vendor/faad2");
    if !source.join("CMakeLists.txt").exists() {
        panic!("FAAD2 source missing: run `git submodule update --init` first");
    }

    let dst = cmake::Config::new(source)
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("FAAD_BUILD_CLI", "OFF")
        .define("FAAD_BUNDLED_MODE", "ON")
        // Same static C runtime as the Rust side (see .cargo/config.toml). FAAD2 requires
        // CMake 3.15+, where this variable decides the runtime instead of compiler flags.
        .define("CMAKE_MSVC_RUNTIME_LIBRARY", "MultiThreaded")
        .static_crt(true)
        .build_target("faad")
        .build();

    // Multi-config generators (Visual Studio) put the library into a per-configuration folder.
    let build = dst.join("build");
    println!("cargo:rustc-link-search=native={}", build.display());
    for config in ["Release", "RelWithDebInfo", "MinSizeRel", "Debug"] {
        println!("cargo:rustc-link-search=native={}", build.join(config).display());
    }
    println!("cargo:rustc-link-lib=static=faad");
    println!("cargo:rerun-if-changed=vendor/faad2/libfaad");
    println!("cargo:rerun-if-changed=build.rs");
}
