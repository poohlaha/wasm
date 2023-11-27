# wasm
  使用 `rust` 开发 `wasm`

## 项目
```
├── packages                                         
│   ├── wasm-utils                                   // utils
│   └── wasm-tools                                   // tools
├── .gitignore                                       // gitignore文件
├── .rustfmt.toml                                    // 格式化配置文件
├── Cargo.toml                                       // rust程序配置文件
└── README.md                                        // 项目使用说明文件
```

## Usage

* Wasm Utils([README.md](packages/wasm-utils/README.md))
* Wasm Tools([README.md](packages/wasm-tools/README.md))


## 构建

```shell
cargo install wasm-pack
wasm-pack build --target web
```

## 缩小 `wasm` 体积

```shell
cargo install wasm-opt
wasm-opt -Os pkg/wasm_utils_bg.wasm -o pkg/wasm_utils_bg.wasm

cargo install wasm-snip
wasm-snip pkg/wasm_utils_bg.wasm -o pkg/wasm_utils_bg.wasm annoying_space_waster

brew install wabt
wasm2wat pkg/wasm_utils_bg.wasm
```

## 分析工具
用此工具需要添加 `main.rs`

```shell
cargo install cargo-bloat
cargo bloat
```

# License
Apache License, Version 2.0 ([LICENSE](LICENSE) or https://apache.org/licenses/LICENSE-2.0)

