args@{ stdenv, lib, fetchFromGitHub, rustPlatform,
  pkgconfig, cargo, rustc, libdrm,
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
    buildInputs = [ libdrm ];

    depsSha256 = "08riayb1lbqcz2nm2pf5lkb6chi971f4prqzg64hf18f8m4rb889";

    doCheck = false;
  };
in self

