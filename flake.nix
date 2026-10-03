{
  description = "wlr - a wallust theme and wallpaper manager";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    # wlr is written against wallust 4 (not yet in nixpkgs)
    wallust = {
      url = "git+https://codeberg.org/explosion-mental/wallust/?rev=1f36e1546d31a42fccce9764240d21436f5d29c4";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, wallust }:
    let
      systems = [ "x86_64-linux" "aarch64-linux" ];
      forAllSystems = f: nixpkgs.lib.genAttrs systems (system: f nixpkgs.legacyPackages.${system});
      wallustFor = pkgs: wallust.packages.${pkgs.stdenv.hostPlatform.system}.default;
    in
    {
      packages = forAllSystems (pkgs: {
        default = pkgs.rustPlatform.buildRustPackage {
          pname = "wlr";
          version = "1.0.0";
          # Only the crate, so README/screenshot edits don't trigger a rebuild
          src = pkgs.lib.fileset.toSource {
            root = ./.;
            fileset = pkgs.lib.fileset.unions [ ./Cargo.toml ./Cargo.lock ./src ];
          };
          cargoLock.lockFile = ./Cargo.lock;

          nativeBuildInputs = [ pkgs.installShellFiles pkgs.makeWrapper ];

          postInstall = ''
            installShellCompletion --cmd wlr \
              --bash <($out/bin/wlr completions bash) \
              --zsh <($out/bin/wlr completions zsh) \
              --fish <($out/bin/wlr completions fish)
            # Fallback only: a wallust already on PATH takes precedence
            wrapProgram $out/bin/wlr --suffix PATH : ${pkgs.lib.makeBinPath [ (wallustFor pkgs) ]}
          '';

          meta.mainProgram = "wlr";
        };
      });

      devShells = forAllSystems (pkgs: {
        default = pkgs.mkShell {
          packages = [ pkgs.cargo pkgs.rustc pkgs.rust-analyzer pkgs.clippy pkgs.rustfmt (wallustFor pkgs) ];
        };
      });
    };
}
