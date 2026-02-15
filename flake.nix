{
  description = "vortix Rust TUI";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    crane.url = "github:ipetkov/crane";
  };

  outputs = { self, nixpkgs, flake-utils, crane }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        craneLib = crane.mkLib pkgs;

        commonArgs = {
          src = craneLib.cleanCargoSource ./.;
          strictDeps = true;
        };

        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        vortix = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;
          # Tests exercise local user state and can fail in Nix build sandboxes.
          doCheck = false;
        });
      in
      {
        packages.default = vortix;
        packages.vortix = vortix;

        apps.default = flake-utils.lib.mkApp { drv = vortix; };

        devShells.default = pkgs.mkShell {
          inputsFrom = [ vortix ];
          packages = with pkgs; [
            cargo
            clippy
            rustc
            rustfmt
          ];
        };
      }
    );
}
