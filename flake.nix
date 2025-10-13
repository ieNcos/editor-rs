{
  # ... 其他 flake 配置 (inputs, outputs 等)
  outputs = { nixpkgs, rust-overlay, ... }:
    # ... 系统枚举
    {
      devShells.default = pkgs.mkShell {
        packages = with pkgs; [
            cargo
        ];
      };
    };
}
