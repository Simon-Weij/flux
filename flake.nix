/*
Copyright (c) 2026 Simon-Weij

This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/
{
  description = "Development shell for Flux";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = {
    self,
    nixpkgs,
    ...
  }: let
    system = "x86_64-linux";
    pkgs = import nixpkgs {inherit system;};
    rustPlatform = pkgs.rustPlatform;
  in {
    packages."${system}".default = rustPlatform.buildRustPackage (finalAttrs: {
      pname = "flux";
      version = "0.1.0";

      src = ./.;

      cargoLock.lockFile = ./src-tauri/Cargo.lock;

      npmDeps = pkgs.fetchNpmDeps {
        name = "${finalAttrs.pname}-${finalAttrs.version}-npm-deps";
        inherit (finalAttrs) src;
        hash = "sha256-PwbLf2dR8UN7+buqItqllZxzRi1OD1uwFouSIv5b1tY=";
      };

      nativeBuildInputs = with pkgs;
        [
          expect
          pkg-config
          gobject-introspection
          cargo-tauri.hook
          nodejs
          npmHooks.npmConfigHook
          gpu-screen-recorder
          libnotify
          python313
        ]
        ++ lib.optionals stdenv.hostPlatform.isLinux [wrapGAppsHook4];

      buildInputs = with pkgs;
        [
          alsa-lib
          at-spi2-atk
          atkmm
          cairo
          gdk-pixbuf
          glib
          glib.dev
          gtk3
          harfbuzz
          librsvg
          libsoup_3
          pango
          webkitgtk_4_1
          openssl
          poetry
          pkg-config
          gcc
        ]
        ++ lib.optionals stdenv.hostPlatform.isLinux [
          glib-networking
        ];

      desktopItems = [
        (pkgs.makeDesktopItem {
          name = "flux";
          exec = "flux";
          icon = "${finalAttrs.src}/src-tauri/icons/icon.ico";
          desktopName = "Flux";
          genericName = "Flux App";
          categories = ["Utility"];
        })
      ];

      postInstall = ''
        sed -i "s|Icon=flux|Icon=$out/share/icons/hicolor/256x256/apps/flux.ico|" $out/share/applications/flux.desktop
        mkdir -p $out/share/icons/hicolor/256x256/apps/
        cp ${finalAttrs.src}/src-tauri/icons/icon.ico $out/share/icons/hicolor/256x256/apps/flux.ico
      '';

      cargoRoot = "src-tauri";
      buildAndTestSubdir = finalAttrs.cargoRoot;
    });
    devShells."${system}".default = pkgs.mkShell {
      nativeBuildInputs = with pkgs; [
        expect
        pkg-config
        gobject-introspection
        rustc
        cargo
        tauri
        nodejs
        libnotify
        python313
      ];

      buildInputs = with pkgs; [
        alsa-lib
        at-spi2-atk
        atkmm
        cairo
        gdk-pixbuf
        glib
        glib.dev
        gtk3
        harfbuzz
        librsvg
        libsoup_3
        pango
        webkitgtk_4_1
        openssl

        poetry
        pkg-config
        gcc
      ];

      shellHook = ''
        export RUST_SRC_PATH="${pkgs.rustPlatform.rustLibSrc}";
        echo "Dev environment loaded"
      '';
    };
  };
}
