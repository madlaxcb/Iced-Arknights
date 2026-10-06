# hud-ui

基于 [iced 0.14](https://github.com/iced-rs/iced) 的「HUD 风格」桌面 UI 组件库
（深色底、细线、45° 切角、克制强调色），目标平台为 Windows 10 22H2 / 11 桌面端。

> 视觉风格受特定科幻游戏界面语言启发；**不使用**其任何美术资产、Logo、字体与名称。

## 目录约定（本仓库根为 `code/`，其上级为工作区）

| 目录 | 内容 |
|---|---|
| `dev/` | 依赖与环境（CARGO_HOME、字体等外部资源） |
| `code/` | 源码（本 workspace） |
| `doc/` | 计划书、预研报告、ADR、进度 |
| `dist/` | 出产构建物（release 二进制 / 压缩包） |

## Workspace 结构

```
crates/
  hud-tokens    设计 Token（纯数据，serde，可文件加载）
  hud-core      形状几何 / 像素对齐 / ChamferedBox 自定义 Widget
  hud-theme     HudTheme 与 13 个内置 widget 的 Catalog 样式
  hud-widgets   组件（panel/card/button 变体/decor 装饰/prelude+宏）
  hud-icons     内嵌 SVG 图标（feature 裁剪）
  hud-platform  平台集成（窗口操作；Windows 专有调用隔离于此）
  hud-testkit   测试辅助
apps/
  gallery       组件陈列馆（组件 × 状态走查载体，主题热更新）
  sample-app    示例应用（仪表盘 / 节点管理 / 设置 / 操作流程）
  proto         M0 原型留档（切角 + 自定义 Theme 最小验证）
```

依赖方向单向：`tokens → core → theme → widgets`；`icons` / `platform` 为旁支。

## 快速上手（本机，约 15 分钟内可跑通）

```bash
# 1) 工具链：stable Rust ≥ 1.88（仓库已含 rust-toolchain.toml）

# 2) 依赖镜像（crates.io 直连受限时）：code/.cargo/config.toml 已配置 rsproxy
#    依赖缓存位于上级 dev/cargo（CARGO_HOME），可复用

# 3) 构建 & 测试（在 code/ 内）
export CARGO_HOME="$PWD/../dev/cargo"
cargo test --workspace

# 4) 运行陈列馆（Linux 无显示环境时用 Xvfb + 软件渲染）
export DISPLAY=:99 ICED_BACKEND=tiny-skia
cargo run -p gallery
cargo run -p sample-app
#   Windows：直接 cargo run -p gallery（默认走 wgpu）

# 5) 主题热更新（仅 debug 构建）
#    修改 assets/theme/dark.toml 保存，运行中的 gallery 即时变色
```

## 用法示例

```rust
use hud_widgets::{column, prelude::*, button, ButtonVariant, typography};

fn view(state: &State) -> hud_widgets::Element<'_, Message> {
    column![
        typography::section_header("作战面板", "OPERATIONS"),
        button(text("启动 / START").size(14))
            .padding(10)
            .on_press(Message::Start)
            .class(button::class(ButtonVariant::Primary)),
    ]
    .spacing(16)
    .into()
}

fn main() -> iced::Result {
    iced::application(State::new, State::update, State::view)
        .theme(|_: &State| hud_theme::HudTheme::dark())
        .run()
}
```

注意（iced 0.14 + 自定义主题，详见 `doc/ADR-001`）：

- 布局请用 `hud::column! / row! / stack!`（标准宏会把子元素主题钉死为 `iced::Theme`）；
- `.class()` 传入 `Box::new(f) as XxxStyleFn` 或使用库内预置类。

## 组件状态

见 `doc/PROGRESS.md`（M0–M6 已完成，进度与截图齐全）。

## 质量门

```bash
./scripts/check.sh   # fmt --check + clippy -D warnings + test
```

## 下载与校验

演示程序（Linux / Windows x86_64）通过 GitHub Releases 分发，
本目录在源码仓库中仅保留说明与校验文件：

| 文件 | 说明 |
|---|---|
| [`CHECKSUMS.sha256`](CHECKSUMS.sha256) | 可执行文件 SHA-256 校验和 |
| [`VIRUS-SCAN.md`](VIRUS-SCAN.md) | 可执行文件的病毒扫描报告（ClamAV，结果干净） |

- 下载地址：<https://github.com/madlaxcb/Iced-Arknights/releases/tag/v0.1.0>
- 资产：`hud-ui-0.1.0.zip`（zip 本身的 SHA-256 见 Release 说明）
- 校验方式：解压后在 `hud-ui-0.1.0/` 目录内执行 `sha256sum -c CHECKSUMS.sha256`
- 主题文件 `dark.toml` 在压缩包内，与程序同目录放置生效

> README 中的截图为开发过程中的存档，可能与实际程序界面略有出入，请以实际运行效果为准。

## 许可

本发行包基于 [Apache License 2.0](https://www.apache.org/licenses/LICENSE-2.0) 发布
（源码仓库根目录附有完整许可证文本）。

分发提示：Linux 版本通过 `sctk-adwaita` 内嵌 Cantarell 字体
（SIL Open Font License 1.1），再分发时请一并保留该字体的版权与许可声明。
