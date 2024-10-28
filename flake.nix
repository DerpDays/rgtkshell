{
    inputs = {
        nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
        fenix.url = "github:nix-community/fenix/monthly";
        fenix.inputs.nixpkgs.follows = "nixpkgs";
        crane.url = "github:ipetkov/crane";
        flake-utils.url = "github:numtide/flake-utils";
    };

    outputs = {
        # self,
        nixpkgs,
        fenix,
        crane,
        flake-utils,
        ...
    }:
        flake-utils.lib.eachDefaultSystem (system: let
            # pkgs = nixpkgs.legacyPackages.${system};
            pkgs = import nixpkgs {
                inherit system;
                overlays = [fenix.overlays.default];
            };
            craneLib =
                (crane.mkLib pkgs).overrideToolchain
                fenix.packages.${system}.minimal.toolchain;

            rgtkshell = craneLib.buildPackage rec {
                src = craneLib.cleanCargoSource ./.;

                # Add extra inputs here or any other derivation settings
                # doCheck = true;
                buildInputs = [
                    fenix.packages.${pkgs.system}.complete.toolchain
                    pkgs.mold
                    pkgs.pkg-config
                    pkgs.gtk4
                    pkgs.gtk4-layer-shell
                    pkgs.pango
                    pkgs.cairo
                    pkgs.harfbuzz
                    pkgs.gdk-pixbuf

                    pkgs.libGL
                    pkgs.vulkan-headers
                    pkgs.vulkan-loader
                    pkgs.vulkan-tools
                    pkgs.vulkan-tools-lunarg
                    pkgs.vulkan-extension-layer
                    pkgs.vulkan-validation-layers # don't need them *strictly* but immensely helpful
                    pkgs.graphene
                    pkgs.glib
                ];
                # nativeBuildInputs = [];

                shellHook = ''
                    export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${builtins.toString (pkgs.lib.makeLibraryPath buildInputs)}";
                '';
            };
        in {
            packages.default = rgtkshell;
            devShells.default = pkgs.mkShell rec {
                # buildInputs = [rgtkshell];
                buildInputs = [
                    fenix.packages.${pkgs.system}.complete.toolchain
                    pkgs.mold
                    pkgs.pkg-config
                    pkgs.gtk4
                    pkgs.gtk4-layer-shell
                    pkgs.pango
                    pkgs.cairo
                    pkgs.harfbuzz
                    pkgs.gdk-pixbuf

                    pkgs.libGL
                    pkgs.vulkan-headers
                    pkgs.vulkan-loader
                    pkgs.vulkan-tools
                    pkgs.vulkan-tools-lunarg
                    pkgs.vulkan-extension-layer
                    pkgs.vulkan-validation-layers # don't need them *strictly* but immensely helpful
                    pkgs.graphene
                    pkgs.glib
                ];
                # nativeBuildInputs = [];

                shellHook = ''
                    export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${builtins.toString (pkgs.lib.makeLibraryPath buildInputs)}";
                '';
            };
        });
}
