{
    inputs = {
        nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
        zig-overlay.url = "github:mitchellh/zig-overlay";
        zls-overlay.url = "github:zigtools/zls";
    };

    outputs = { self, nixpkgs, zig-overlay, zls-overlay }: let
        system = "x86_64-linux";
        pkgs = import nixpkgs { inherit system; };
        zig = zig-overlay.packages.${system}.master;
        zls = zls-overlay.packages.${system}.zls.overrideAttrs {
            nativeBuildInputs = [ zig ];
        };
    in {
        devShells.${system}.default = pkgs.mkShell {
            packages = with pkgs; [
                zig
                zls
            ];
        };
    };
}