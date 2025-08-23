{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
  inputsFrom = with pkgs; [
    llvmPackages.bintools
    rustc
  ];

  buildInputs = with pkgs; [
    llvmPackages.bintools
    # gcc

    # Rust toolchain
    rustc
    cargo
    rustfmt
    clippy

    # Development tools
    cargo-audit
    cargo-tarpaulin
    rust-analyzer
    clippy
    cargo-crev
    # cargo-deb # build deb
    # cargo-deps # dependency graph

    # Shell completions
    installShellFiles

    # Nix tools
    nixpkgs-fmt
    nix-prefetch-github
  ];

  packages = with pkgs; [
    pkg-config
    llvmPackages.bintools
    rustc
  ];

  shellHook = ''
    echo "🔨 Library for SIMKL"
  '';

  # Certain Rust tools won't work without this
  # This can also be fixed by using oxalica/rust-overlay and specifying the rust-src extension
  # See https://discourse.nixos.org/t/rust-src-not-found-and-other-misadventures-of-developing-rust-on-nixos/11570/3?u=samuela. for more details.
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
