{
  description = "monkey";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        projectName = "monkey";
      in {
        packages.${projectName} = pkgs.rustPlatform.buildRustPackage {
          pname = projectName;
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
        };

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
