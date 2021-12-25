{
  description = "A very basic flake";

  inputs = {
    flake-utils.url = github:numtide/flake-utils;
    rust-overlay.url = github:oxalica/rust-overlay;
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }: flake-utils.lib.eachDefaultSystem (
    system:
    let
      overlays = [ (import rust-overlay) ];
      pkgs = import nixpkgs { inherit system overlays; };
      rust = pkgs.rust-bin.stable.latest.default.override { extensions = [ "rust-src" ]; };
    in
    {
      devShell = pkgs.mkShell {
        buildInputs = (
          with pkgs; [
            bashInteractive
            rust
            cargo-edit
            (wrapBintoolsWith { bintools = llvmPackages_latest.bintools-unwrapped; })
          ]
        );
        RUSTFLAGS = "-Clink-arg=-fuse-ld=lld";
      };
    }
  );
}
