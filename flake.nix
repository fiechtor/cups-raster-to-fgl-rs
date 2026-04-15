{
  description = "Rust flake";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = {
    self,
    nixpkgs,
    naersk,
    ...
  } @ inputs: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};

    naersk' = pkgs.callPackage naersk {};
  in {
    devShells.${system}.default = pkgs.mkShell {
      packages = with pkgs; [rustc cargo rustfmt rust-analyzer cups chafa];
      buildInputs = with pkgs; [
        chafa
      ];
    };
    nixConfig = {
      extra-substituters = [
        "https://nix-community.cachix.org"
      ];
      extra-trusted-public-keys = [
        "nix-community.cachix.org-1:mB9FSh9qf2dCimDSUo8Zy7bkq5CX+/rkCWyvRCYg3Fs="
      ];
    };
    packages."${system}".default = naersk'.buildPackage {
      release = true; # Build as non-debug mode
      copyBins = true; # auto install binary to system
      src = ./.;

      buildInputs = with pkgs; [rustc cargo rustfmt rust-analyzer];
    };
  };
}
