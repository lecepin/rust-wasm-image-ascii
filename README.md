# Rust wasm image to ascii

```
██████████████████████████████████████████████████
████████████████████    ██████████████████████████
███████████████████   █  █████████████████████████
██████████████████  █████  ███████████████████████
█████████████████   █████    █████████████████████
████████████████   █████  █    ███████████████████
██████████████    █████  █  ██    ████████████████
████████████      ███  █  ██████   ███████████████
███████████  █    ██  █  ██████  █  ██████████████
██████████  ██    █  ██ █████   ███  █████████████
██████████          ███       ████     ███████████
█████████     █    █████      ███   █  ███████████
█████████  █   █  ██      ███  █      ████████████
██████████ ███    █  ███████   █     █████████████
████████     █    █  █       ██     ██████████████
███████    █   █  ███     ████     ███████████████
██████     ██ ███  ███████████   █████████████████
██████ █    █           ████      ████████████████
██████               ██            ███████████████
██████                       █████  ██████████████
█████                              ███████████████
██████ ███   █      心中有光        ████████████████
███████      █                  ██████████████████
███████████                     ██████████████████
███████████████               ████████████████████
███████████████                ███████████████████
███████████                     ██████████████████
█████████  ██                    █████████████████
████████                           ███████████████
████████          ███████      ██   ██████████████
███████          ██████████           ████████████
██████          █████████████         ████████████
████   ██      ███████████████      ██   █████████
███         ██████████████████████ █   ███████████
██████████████████████████████████████████████████
██████████████████████████████████████████████████
```

### 灰度算法

灰度算法对比：

[https://lecepin.github.io/rust-wasm-image-ascii/gray.html](https://lecepin.github.io/rust-wasm-image-ascii/gray.html)

![image](https://user-images.githubusercontent.com/11046969/185297221-e4441e32-5802-418c-a3bf-e9f9900966e3.png)


这里直接使用的 `image` crate 的内置算法，上图中的第三种：

```rust
// luminance formula credits: https://stackoverflow.com/a/596243
// >>> Luminance = 0.2126*R + 0.7152*G + 0.0722*B <<<
// calculate RGB values to get luminance of the pixel
pub fn get_luminance(r: u8, g: u8, b: u8) -> f32 {
    let r = 0.2126 * (r as f32);
    let g = 0.7152 * (g as f32);
    let b = 0.0722 * (b as f32);
    r + g + b
}
```

### 简单版本

简单版本只做了一种效果，访问地址： [https://lecepin.github.io/rust-wasm-image-ascii/test.html](https://lecepin.github.io/rust-wasm-image-ascii/test.html)

![](./docs/02.webp)

### Tai 版本

看到一个支持 ASCII 种类挺多的 Rust 项目 https://github.com/MustafaSalih1993/tai ，于是将这个项目的 IO 部分进行了修改，适配 WASM 进行了编译处理。

![](./docs/03.webp)

## 安装&使用

```html
<script type="module">
  import initWasm, {
    get_gray_image,
    get_ascii_by_image,
    get_ascii_by_image_tai,
  } from "./pkg/rust_wasm_image_ascii.js";

  initWasm()
    .then(() => {});
</script>
```

可以直接使用仓库中 `pkg/` 目录中的文件，也可以使用 upkg 的资源 https://unpkg.com/browse/rust-wasm-image-ascii/ ，也可以 `npm install rust-wasm-image-ascii` 使用。

接口描述参考这里：[pkg/rust_wasm_image_ascii.d.ts](./pkg/rust_wasm_image_ascii.d.ts)

