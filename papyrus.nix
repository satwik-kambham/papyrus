let
  pkgs = import <nixpkgs> {};
in
pkgs.appimageTools.wrapType2 {
  name = "papyrus";
  src = pkgs.fetchurl {
    url = "https://github.com/satwik-kambham/papyrus/releases/download/app-v0.0.0/papyrus_0.1.0_amd64.AppImage";
    hash = "sha256-YuetFX7AmIkqY31hAHLhGYfitU83cvIo19BF7nTSpG4=";
  };
  extraPkgs = pkgs: with pkgs; [];
}
