{
  description = "A Rust development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    # 引入 rust-overlay 来管理 Rust 工具链
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        # 覆盖 nixpkgs，加入 rust-overlay
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };
        # 定义 Rust 工具链，这里使用 2024-05-02 的 nightly 版本，并添加一些扩展
        rustToolchain = pkgs.rust-bin.nightly."2024-05-02".minimal.override {
          extensions = [
            "rust-src"    # 用于 Rust 分析，rust-analyzer 需要
            "rust-analyzer"
            "clippy"
          ];
          # 如果需要交叉编译，可以在这里指定目标平台
          # targets = [ "riscv64gc-unknown-none-elf" ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          # 定义开发环境的依赖
          buildInputs = with pkgs; [
            rustToolchain   # Rust 工具链
            cargo-binutils # cargo-binutils 提供了像 cargo-binutils 这样的工具
            python3        # 一些构建脚本可能会用到
            gdb            # 调试器
            # 添加其他你需要的依赖，例如 openssl, pkg-config 等
          ];
          
          # 可选：在进入 shell 时执行的命令，例如显示版本信息
          shellHook = ''
            rustc --version
            cargo --version
          '';
        };
      }
    );
}
