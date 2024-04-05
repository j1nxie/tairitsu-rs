{ pkgs ? import <nixpkgs> {}}:

pkgs.mkShell {
  packages = with pkgs; [ pkg-config openssl python3 lldb ];
}