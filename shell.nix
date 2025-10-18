{
  pkgs ? import <nixpkgs> { },
  mode ? "dev",
}:
let
  rust = with pkgs; [
    cargo
    rustc

    rustfmt
    clippy

    gdb
  ];

  multiplex = with pkgs; [
    rust-analyzer
    ra-multiplex
    wayland
    pkg-config
  ] ++ rust;

  server_start = ''
    alias kr="pgrep 'ra-multiplex' | xargs kill"
    alias sr="ra-multiplex server & nvim --headless src/*.rs"
    sr
  '';

  aliases = ''
    alias cr="cargo run"
    alias cb="cargo build"
    alias ct="cargo test"
    alias cc="cargo-clippy"
    alias g="rust-gdb -tui ./target/debug/timer"
  '';
in
pkgs.mkShell {
  buildInputs = if mode == "server" then multiplex else rust;

  shellHook = if mode == "server" then server_start else aliases;
}
