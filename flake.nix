{
  description = "monkey";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, crane }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        craneLib = crane.lib.${system};
        projectName = "monkey";
      in {
        packages.${projectName} =
          craneLib.buildPackage { src = craneLib.cleanCargoSource ./.; };

        packages.default = self.packages.${system}.${projectName};

        apps.${projectName} = {
          type = "app";
          program = "${self.packages."${system}".default}/bin/monkey";
        };

        apps.default = self.apps.${system}.monkey;

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            cargo
            rustc
            rustfmt
            clippy
            rust-analyzer
            rustup
            crate2nix
          ];
        };
      });
}
