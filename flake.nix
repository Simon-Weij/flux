/*
  Copyright (c) 2026 Simon-Weij

  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

{
  description = "Development shell for AllStore";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs =
    { self, nixpkgs, ... }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
    in
    {
      devShells."${system}".default = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [
          expect
          pkg-config
          gobject-introspection
          rustc
          cargo
          cargo-tauri
          nodejs
        ];

        buildInputs = with pkgs; [
          at-spi2-atk
          atkmm
          cairo
          gdk-pixbuf
          glib
          gtk3
          harfbuzz
          librsvg
          libsoup_3
          pango
          webkitgtk_4_1
          openssl
        ];

        shellHook = ''
          export RUST_SRC_PATH="${pkgs.rustPlatform.rustLibSrc}";
          echo "Dev environment loaded"
        '';
      };
    };
}
