let
  pkgs = (import <nixpkgs> {}).fetchFromGitHub {
    owner = "NixOS";
    repo = "nixpkgs-channels";
    rev = "b5f5c97f7d67a99b67731a8cfd3926f163c11857";
    # nixos-unstable as of 2019-07-24T18:57:18-05:00
    sha256 = "1m9xb3z3jxh0xirdnik11z4hw95bzdz7a4p3ab7y392345jk1wgm";
  };
in
with (import pkgs {});
rustPackages.rustPlatform.buildRustPackage rec {
  name = "rust-gtk-counter-${version}";
  version = "0.1.0";
  src = ./.;
  cargoSha256 = "0i4px1k23ymq7k3jp6y5g7dz0ysjzwrqqxfz4xg399y7zg5wwwhr";
  nativeBuildInputs = [ pkgconfig ];
  buildInputs = [ cairo gtk3 glib pkgconfig ];
}
