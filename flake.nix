{
  description = "Mathematics for Machine Learning and Data Science in Rust";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixpkgs-unstable";
    nix-shells.url = "github:Alfablos/nix-shells";
    nix-shells.inputs.nixpkgs.follows = "nixpkgs";
    # oxalica-rust.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      self,
      nixpkgs,
      nix-shells,
      # oxalica-rust,
      ...
    }:
    let
      forAllSystems =
        f:
        nixpkgs.lib.genAttrs
          [
            "x86_64-linux"
            "aarch64-linux"
          ]
          (
            system:
            f (
              import nixpkgs {
                inherit system;
                config.allowUnfree = true;
                # overlays = [ oxalica-rust.overlays.default ];
              }
            )
          );
    in
    {
      devShells = forAllSystems (pkgs: {
        default = self.devShells.${pkgs.system}.rust;
        rust = nix-shells.packages.x86_64-linux.lib.shells.rust {
          rustVersion = "1.86.0";
          withPkgs = with pkgs; [ ];
        };
      });
    };
}
