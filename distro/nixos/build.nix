args@{ stdenv, lib, fetchFromGitHub, rustPlatform,
  pkgconfig, cargo, rustc, wayland, mesa,
  # Plugins
  ... }:

let
  self = rustPlatform.buildRustPackage rec {
    name = "wl-zoomy-${version}";
    version = "0.0.1";

    src = fetchFromGitHub {
      owner = "Abdillah";
      repo = "wl-zoomy";
      rev = "bbcdd0bad45ce2e0ef65d2ddfa9f89482a5a8986";
      sha256 = "01ixhvq3wi7d70cm0s5b28yjsgkzlyz9mp99kiix33922hrzacl7";
    };

    nativeBuildInputs = [ pkgconfig rustc cargo ];
    buildInputs = [ wayland mesa ];

    depsSha256 = "0yrjjbi0mapp2shkvb9r4xva84mm558lxajwy7fp5www9xxsapla";

    doCheck = false;
  };
in self

