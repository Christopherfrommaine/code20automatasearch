let
  pkgs = import <nixpkgs> {};
in pkgs.mkShell {
  buildInputs = [
    
    pkgs.rustup
    pkgs.dos2unix
    pkgs.pstree
  
  ];

  shellHook = ''
    export RUST_BACKTRACE=1
    rustup toolchain install stable
    rustup default stable
  '';

}
