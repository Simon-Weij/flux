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

    fluxPkg = rustPlatform.buildRustPackage (finalAttrs: {
      pname = "flux";
      version = "0.1.0";

      src = ./.;
      cargoRoot = "src-tauri";
      cargoLock = {lockFile = ./src-tauri/Cargo.lock;};

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
        ++ lib.optionals stdenv.hostPlatform.isLinux [glib-networking];

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

      buildAndTestSubdir = finalAttrs.cargoRoot;
    });
  in {
    packages."${system}".default = fluxPkg;

    devShells."${system}".default = pkgs.mkShell {
      nativeBuildInputs = fluxPkg.nativeBuildInputs;
      buildInputs = fluxPkg.buildInputs;

      shellHook = ''
        export RUST_SRC_PATH="${pkgs.rustPlatform.rustLibSrc}"
        echo "Dev environment loaded"
      '';
    };
  };
}
