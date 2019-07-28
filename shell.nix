{ pkgs ? import <nixpkgs> {} }:
with pkgs;
mkShell {
  buildInputs = [ cargo rustc cairo gtk3 glib pkgconfig ];
}
