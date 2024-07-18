{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
  name = "Rust";
  buildInputs = [ pkgs.cargo pkgs.rustc pkgs.rustfmt ];
}
