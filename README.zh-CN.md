# hud-ui

基于 [iced 0.14](https://github.com/iced-rs/iced) 的「HUD 风格」桌面 UI 组件库
（深色底、细线、45° 切角、克制强调色），目标平台为 Windows 10 22H2 / 11 桌面端；
Linux 可作为开发与演示平台（X11/Wayland + 软件渲染回退）。

> 视觉风格受特定科幻游戏界面语言启发；**不使用**其任何美术资产、Logo、字体与名称。

[English](README.md) | **简体中文**

## 仓库结构

| 目录 | 内容 |
|---|---|
| `code/` | 源码（Cargo workspace：7 个 crate + 3 个演示应用） |
| `doc/` | 设计文档、ADR、进度记录、里程碑截图 |

`dev/`（本机工具链/缓存）与 `dist/`（发布产物）仅为本地目录约定，不入库。

## Workspace 结构

```
code/
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

## 快速上手

环境要求：stable Rust ≥ 1.88（见 `code/rust-toolchain.toml`）。

```bash
cd code

# 构建 & 测试
cargo test --workspace

# 运行陈列馆（Linux 无显示环境时用 Xvfb + 软件渲染）
export DISPLAY=:99 ICED_BACKEND=tiny-skia
cargo run -p gallery
# 示例应用：
cargo run -p sample-app
#   Windows：直接 cargo run -p gallery（默认走 wgpu）

# 主题热更新（仅 debug 构建）：
# gallery 运行中修改 code/assets/theme/dark.toml 即时变色
```

说明：`code/.cargo/config.toml` 将 crates.io 替换为 rsproxy.cn 镜像
（开发环境直连受限）。如不需要镜像，删除该文件中的 `[source.*]` 配置即可。

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

注意（iced 0.14 + 自定义主题，详见 `doc/ADR-001-主题架构.md`）：

- 布局请用 `hud::column! / row! / stack!`（标准宏会把子元素主题钉死为 `iced::Theme`）；
- `.class()` 传入 `Box::new(f) as XxxStyleFn` 或使用库内预置类。

## 组件状态

见 [doc/PROGRESS.md](doc/PROGRESS.md)（M0–M6 已完成，进度与截图齐全）。

## 设计与开发文档

- [设计规范](doc/设计规范.md)：颜色、间距、字体、几何与动效。
- [组件 API](doc/组件API.md)：组件定位、构造入口与状态流。
- [迁移指南](doc/迁移指南.md)：当前 iced 锁版与升级检查步骤。
- [新组件开发指南](doc/新组件开发指南.md)：依赖方向、Token、Cache 与验证约定。

## Sample App

四页业务演示覆盖仪表盘、节点列表 + 详情、设置表单和确认 / 错误 / 加载流程。

| 仪表盘 | 节点详情 | 设置 | 操作流程 |
|---|---|---|---|
| ![Dashboard](doc/m6-sample-dashboard.png) | ![Nodes](doc/m6-sample-nodes.png) | ![Settings](doc/m6-sample-settings.png) | ![Dialog flow](doc/m6-sample-dialog-flow.png) |

> 截图为开发里程碑的存档，可能与当前版本的实际界面略有出入。
> 可在 [`dist/hud-ui-0.1.0/`](dist/hud-ui-0.1.0/) 下载对应平台的演示程序
> （附带 SHA-256 校验和与病毒扫描报告）查看实际效果。

## 质量门

```bash
cd code && ./scripts/check.sh   # fmt --check + clippy -D warnings + test
```

## 许可

本项目基于 [Apache License 2.0](LICENSE) 发布。

分发提示：Linux 构建可能通过 `sctk-adwaita` 内嵌 Cantarell 字体
（SIL Open Font License 1.1）。再分发二进制时请一并保留该字体的版权与许可声明。
