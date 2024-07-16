{
  description = "Create a link of a hwmon temp input based on name and label";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      nixpkgs,
      crane,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        craneLib = crane.mkLib nixpkgs.legacyPackages.${system};
        src = craneLib.cleanCargoSource (craneLib.path ./.);
        commonArgs = {
          inherit src;
          inherit (craneLib.crateNameFromCargoToml { cargoToml = ./Cargo.toml; }) pname version;
        };
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;
        temp-linker = craneLib.buildPackage (commonArgs // { inherit cargoArtifacts; });
      in
      {
        packages.default = temp-linker;
      }
    );
}
