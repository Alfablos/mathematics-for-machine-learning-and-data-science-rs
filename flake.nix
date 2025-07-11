{
  description = "Mathematics for Machine Learning and Data Science in Rust";
  inputs = {
    # nixpkgs.url = "github:nixos/nixpkgs?ref=nixpkgs-unstable";
    nixpkgs.url = "github:nixos/nixpkgs?ref=9b008d60392981ad674e04016d25619281550a9d";
    oxalica-rust.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      self,
      nixpkgs,
      oxalica-rust,
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
                config.cudaSupport = true;
                config.cudaVersion = "12";
                overlays = [ oxalica-rust.overlays.default ];
              }
            )
          );

      rustVersion = "1.86.0";
      pythonMajor = "12";
      
      mkLibraryPath = pkgs: with pkgs;
      lib.makeLibraryPath [
        # add other library packages here if needed
        stdenv.cc.cc  # numpy needs C libraries
        cudaPackages.cuda_nvrtc   # libncrtc.so for cupy
      ];
    in
    {
      devShells = forAllSystems (pkgs: {
        default =
        let
          lib = pkgs.lib;
          oxalica-override = pkgs.rust-bin.stable.${rustVersion}.default.override {
            extensions = [ "rust-src" "clippy" "rust-analyzer" "rustfmt" ];
          };
          python = pkgs."python3${pythonMajor}".withPackages (ppkgs: with ppkgs; [
            matplotlib
          ] ++ lib.optionals (!pkgs.config.cudaSupport) [
            numpy
          ] ++ lib.optionals pkgs.config.cudaSupport [
            cupy
            pytorchWithCuda
          ]);
        in
        pkgs.mkShell {
          packages = [
            pkgs.llvmPackages.bintools
            pkgs.pkg-config
            oxalica-override
            pkgs.nodejs-slim
            
            # Since aliases don't work
            (pkgs.writeShellScriptBin "rustrover" "tmux new -d 'rust-rover .'")
            (pkgs.writeShellScriptBin "pycharm" "tmux new -d 'pycharm-professional .'")
            pkgs.openssl
            python
          ] ++ lib.optionals pkgs.config.cudaSupport ((with pkgs.cudaPackages; [
            cuda_cudart
            # cuda_nvrtc
            cudnn
            cutensor
            nccl
            cusparselt
          ]) ++ (with pkgs; [
            cudatoolkit
            libGLU libGL
          ]));
          
          RUST_SRC_PATH = "${oxalica-override}/lib/rustlib/src/rust";
          
          shellHook = ''
            alias v=nvim
            alias vim=nvim
            
            export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${mkLibraryPath pkgs}:/run/opengl-driver/lib:/run/opengl-driver-32/lib"
            export XLA_FLAGS="--xla_gpu_cuda_data_dir=${pkgs.cudaPackages.cudatoolkit}"                                                   # For tensorflow with GPU support
            export CUDA_PATH=${pkgs.cudaPackages.cudatoolkit}
            export EXTRA_CCFLAGS="-I/usr/include" 
            export RUST_BACKTRACE=full
            
            echo "=== RUST ==="
            echo
            echo "Rust version: $(rustc --version)"
            echo "Cargo version: $(cargo --version)"
            echo "Rust toolchain location: ${oxalica-override}/bin"
            echo "RUST_SRC_PATH (stdlib location): $RUST_SRC_PATH"
            echo
            echo

            echo "=== PYTHON ==="
            echo
            echo "Setting PYTHONPATH to ${python}/${python.sitePackages}"
            export PYTHONPATH="${python}/${python.sitePackages}"
            echo Running $(python --version) @ $(which python) ${if pkgs.config.cudaSupport then "with CUDA support" else ""}
            echo
          '';
          
        };
      });
    };
}
