# 开发进度记录（PROGRESS）

> 本文件随里程碑更新。里程碑规划遵循内部开发计划（不随本仓库分发）。

## 当前状态：M0～M7 完成，Windows 真机验证待续

| 里程碑 | 状态 | 交付物 / 证据 |
|---|---|---|
| M0 技术预研 | ✅ 完成 | `doc/M0-预研报告.md`、ADR-001/002/003、原型 `apps/proto`、截图 doc/m0-*.png |
| M1 基础架构 | ✅ 完成 | workspace（7 crates + 3 apps）、rustfmt/clippy/deny 配置、Gallery 壳、hud-platform 窗口操作 |
| M2 主题系统 | ✅ 完成 | hud-tokens（serde+对比度单测）、hud-theme（14 个 Catalog）、主题文件加载 + notify 热更新（实测变色）、Gallery Token 页 |
| M3 形状与装饰 | ✅ 完成 | ChamferedBox 自定义 Widget、snap_to_pixel、CornerBrackets / TickDivider / GridBackground / SlantedStripes、几何单测、Gallery 形状页 |
| M4 通用组件 | ✅ P0/P1 完成 | 4.1：Button 四变体、Panel、Card、SectionHeader、Divider；4.2：Slider/Select/TextArea(text_editor)/ScrollArea；4.3：List/ListItem、Tabs、SideNav 组件化、TitleBar（拖动/双击最大化/最小化/关闭，经 window::oldest 绑定主窗口）；4.4：Select/Tooltip 包装、Badge/Tag 色条（5 色）、分段/斜切 Progress、定宽 Table。均入 Gallery 组件页（scrollable 长页），截图 doc/m44-gallery-components.png |
| M5 特色组件 | ✅ 完成 | modal.rs（遮罩点击 + Esc 关闭 + 居中切角对话框，`escape_listener` 以 `iced_futures::subscription::filter_map` 实现——0.14 的 `Subscription::map` 禁止捕获闭包）；toast.rs（四色 ToastState 队列，4s TTL 自动过期 + 手动关闭）；loading.rs（ScanBar 扫描条 canvas::Cache 写法，16ms 帧推进）；status_panel.rs / data_readout.rs；Gallery 页面切换淡入+位移（Animation + window::frames 条件订阅）。Xvfb + openbox 实测：Modal 开/Esc 关/遮罩关、Toast 推送堆叠过期、扫描动画均通过，截图 doc/m5-gallery-{overlays,modal}.png |
| M6 示例与文档 | ✅ 完成 | Sample App 四页业务流程：仪表盘（在线节点 / 在线负载均值 / 链路 / 事件表）、节点列表 + 详情（状态徽标 / 负载 / 重启确认）、设置（输入 / 下拉 / toggler / checkbox / slider）、操作流程（确认 / 故障 Toast / 2s 同步扫描条 / 链路恢复）；设计规范、组件 API、迁移指南、新组件指南；README 快速上手与四页截图。截图 doc/m6-sample-*.png |
| M7 测试与发布 | ✅ 完成 | 视觉回归基线（100%/150%）、workspace 测试与文档、Linux/Windows release 构建、校验和、ClamAV 扫描、GitHub Release v0.1.0 |

## 验证方式

- 每个里程碑均以 **Xvfb + tiny-skia 实跑 + 截图** 验证（doc/m*-gallery-*.png）。
- **键盘事件验证需在 Xvfb 内跑窗口管理器**（已装 openbox）：winit 0.30 仅经 XI2 FocusIn 维护 `active_window`，无 WM 时为 None，所有 KeyPress 在事件处理入口被静默丢弃（winit event_processor.rs `xinput_key_input`）。鼠标事件不受影响——此前 M0～M4 未察觉。
- `cargo test --workspace` 全绿；当前 hud-widgets 测试包含 Avatar、Breadcrumb、ContextMenu、SplitPane 等组件行为测试。
- `cargo fmt --check`、`cargo clippy --workspace --all-targets -- -D warnings` 零输出。
- 主题热更新实测：修改 `assets/theme/dark.toml` → 窗口即时变色（截图像素比对）。
- 交叉编译：`x86_64-pc-windows-gnu` release 构建成功（6m21s）；wine 冒烟可启动渲染，事件循环在 wine/Xvfb 下不稳定退出，**真 Windows 验证列入 ADR-003 清单**。

## 已确认的关键决策（ADR）

1. ADR-001：主题采用方案 B（自定义 HudTheme + Catalog）。
2. ADR-002：ChamferedBox = R2 自定义 Widget + R1 canvas；**强制 canvas::Cache**（Live 几何缺陷规避）。
3. ADR-003：Linux 开发回路（Xvfb + tiny-skia）+ windows-gnu 交叉出产；Windows 真机项清单见 ADR。

## 后续任务

1. Windows 真机验证：wgpu、中文输入法、无边框窗口、DWM/Snap Layouts、系统减少动画。
2. 可选增强：Modal 焦点陷阱、字体子集化与更完整的 rustdoc。
3. 持续维护：随组件变更更新视觉基线和发布资产。

## Windows 真机待验证清单（本机无法覆盖）

- [ ] wgpu（DX12/Vulkan）主渲染路径
- [ ] 中文输入法候选窗位置与组合态（text_input / text_editor）
- [ ] 无边框窗口：DWM 阴影/圆角、Snap Layouts、四边缩放热区
- [ ] 系统"减少动画"查询（hud-platform::reduce_motion 的 windows_impl）
- [ ] 内嵌 CJK 子集字体后的体积与渲染
