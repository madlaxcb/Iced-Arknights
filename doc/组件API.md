# 组件 API 导览

组件位于 hud_widgets，元素主题固定为 HudTheme。完整签名与文档注释以源码 rustdoc 为准。

## 基础结构

| 模块 | 入口 | 用途 |
|---|---|---|
| panel | panel(element, tokens, ChamferLevel) | 切角面板 |
| typography | section_header、card、secondary_text | 标题、卡片、文字层级 |
| button | class(ButtonVariant) | Primary / Secondary / Ghost / Danger |
| badge | badge(label, BadgeKind) | 短状态标签 |
| decor | CornerBrackets、TickDivider、GridBackground、SlantedStripes | 装饰原语 |

## 导航与数据

| 模块 | 入口 | 状态归属 |
|---|---|---|
| nav | side_nav(items, active, tokens, width, on_select) | 应用持有当前项 |
| list | list(items, selected, on_select) | 应用持有选中项 |
| tabs | Tabs 构造器 | 应用持有索引 |
| table | table(columns, rows, tokens, selected, on_select) | 行选择回调可选 |
| progress | SegmentedProgress::new / SlantedProgress::new | 应用提供进度 |
| data_readout | data_readout(label, value, unit, tokens) | 只读读数 |
| status_panel | status_panel(label, detail, OpStatus, tokens, width) | 应用提供状态 |

## 表单、反馈与浮层

- input::select(options, selected, on_selected) 为受控下拉；tooltip(content, tip) 包装提示。
- 文本输入、编辑器、checkbox、toggler、slider 由应用持有值并经 HudTheme 主题化。
- ToastState 由应用持有；push 入队，retain_fresh(now, ttl) 清理过期项，dismiss(index) 手动关闭，toast_list 渲染。
- modal(base, dialog, tokens, on_dismiss) 组合遮罩和对话框；仅在 modal 打开时订阅 escape_listener(message)。
- ScanBar::new(phase) 绘制扫描条；调用方仅在可见且活动时推进 phase。
- title_bar(title, tokens, to_message) 产生 TitleBarCommand，由应用转成平台窗口操作。

## 主题与样式

使用 hud_widgets 的 column! / row! / stack!，不要让 iced 标准布局宏把子元素主题推断成 iced::Theme。自定义 iced 0.14 StyleFn 需装箱；颜色、尺寸从 Tokens 读取。业务状态留在应用层，组件接收值与 Message 回调。组合示例见 apps/gallery 与 apps/sample-app。
