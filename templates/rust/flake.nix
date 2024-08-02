{
    inputs = {
        nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
        rust-overlay = {
            url = "github:oxalica/rust-overlay";
            inputs.nixpkgs.follows = "nixpkgs";
        };
    };

    outputs = { self, nixpkgs, rust-overlay }: let
        overlays = [ rust-overlay.overlays.default self.overlays.default ];
        system ="x86_64-linux";
        pkgs = import nixpkgs { inherit overlays system; };
    in {
        overlays.default = final: prev: {
            rustToolchain = prev.rust-bin.nightly.latest.default.override {
                extensions = [ "rust-src" "rustfmt" ];
            };
        };

        devShells.${system}.default = pkgs.mkShell {
            packages = with pkgs; [
                rustToolchain
                rust-analyzer
            ];

            env = {
                RUST_SRC_PATH = "${pkgs.rustToolchain}/lib/rustlib/src/rust/library";
            };
        };
    };
}