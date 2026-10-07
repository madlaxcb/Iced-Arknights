#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

//! 组件陈列馆（Gallery）：每个组件 × 每个状态的展示与走查载体。
//!
//! 布局：左侧 SideNav + 右侧内容区；debug 构建支持主题文件热更新。

use hud_theme::HudTheme;
use hud_widgets::{column, prelude::*, row};

mod snapshot;

fn main() -> iced::Result {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|arg| arg == "--snapshot") {
        return snapshot::run();
    }
    if args.iter().any(|arg| arg == "--compare-snapshot") {
        return snapshot::compare();
    }

    iced::application(Gallery::new, Gallery::update, Gallery::view)
        .title(|_: &Gallery| String::from("hud-ui gallery"))
        .theme(|g: &Gallery| g.theme.clone())
        .subscription(Gallery::subscription)
        .window_size((1100.0, 720.0))
        .run()
}

/// debug 构建下监听 assets/theme 目录，文件变化即发 ThemeFileChanged。
#[cfg(debug_assertions)]
fn theme_watcher() -> iced::Subscription<Message> {
    iced::Subscription::run(theme_stream).map(|()| Message::ThemeFileChanged)
}

/// 主题文件 → 空消息流（std 线程内跑 notify watcher，经 futures mpsc 桥接）。
#[cfg(debug_assertions)]
fn theme_stream() -> impl iced::futures::Stream<Item = ()> {
    use iced::futures::channel::mpsc;
    use notify::Watcher;

    let (mut tx, rx) = mpsc::channel::<()>(16);
    std::thread::spawn(move || {
        let (evt_tx, evt_rx) = std::sync::mpsc::channel::<()>();
        let watcher =
            notify::recommended_watcher(move |res: Result<notify::Event, notify::Error>| {
                if res.is_ok() {
                    let _ = evt_tx.send(());
                }
            });
        let mut watcher = match watcher {
            Ok(w) => w,
            Err(_) => return, // 监听不可用时静默退出（无热更新）
        };
        let dir = std::path::Path::new("assets/theme");
        if watcher
            .watch(dir, notify::RecursiveMode::NonRecursive)
            .is_err()
        {
            return;
        }
        while evt_rx.recv().is_ok() {
            if tx.try_send(()).is_err() {
                break; // 接收端已关闭（窗口退出）
            }
        }
    });
    rx
}

/// 页面清单。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Page {
    /// 总览
    Overview,
    /// Token（色板 / 字号 / 几何）
    Tokens,
    /// 形状与装饰原语（M3）
    Shapes,
    /// 通用组件（M4）
    Components,
    /// 反馈与浮层（M5）
    Overlays,
    /// 关于
    About,
}

impl Page {
    const ALL: [Self; 6] = [
        Self::Overview,
        Self::Tokens,
        Self::Shapes,
        Self::Components,
        Self::Overlays,
        Self::About,
    ];

    fn title(self) -> &'static str {
        match self {
            Self::Overview => "总览 / OVERVIEW",
            Self::Tokens => "Token / DESIGN TOKENS",
            Self::Shapes => "形状 / SHAPES",
            Self::Components => "组件 / COMPONENTS",
            Self::Overlays => "浮层 / OVERLAYS",
            Self::About => "关于 / ABOUT",
        }
    }
}

struct Gallery {
    page: Page,
    theme: HudTheme,
    theme_status: String,
    input_value: String,
    checked: bool,
    tab: usize,
    list_sel: Option<usize>,
    table_sel: Option<usize>,
    slider_v: f32,
    choice: Option<DemoChoice>,
    editor: iced::widget::text_editor::Content,
    window: Option<iced::window::Id>,
    // M5：浮层 / 动效
    modal_open: Option<usize>,
    context_menu_open: bool,
    context_menu_state: hud_widgets::context_menu::ContextMenuState,
    breadcrumb_selected: usize,
    split_pane_state: hud_widgets::split_pane::SplitPaneState,
    toasts: hud_widgets::toast::ToastState,
    loading_on: bool,
    loading_phase: f32,
    page_anim: iced::animation::Animation<f32>,
}

/// Select 演示选项。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DemoChoice {
    /// 选项 A
    Alpha,
    /// 选项 B
    Bravo,
    /// 选项 C
    Charlie,
}

impl std::fmt::Display for DemoChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Alpha => write!(f, "模式 A / ALPHA"),
            Self::Bravo => write!(f, "模式 B / BRAVO"),
            Self::Charlie => write!(f, "模式 C / CHARLIE"),
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    /// 切换页面
    Go(Page),
    /// 主题文件变化（notify 订阅触发）
    ThemeFileChanged,
    /// 输入框内容变化
    Input(String),
    /// 复选框 / 开关切换
    Toggled(bool),
    /// 页签切换
    TabsGo(usize),
    /// 列表选中
    ListSel(usize),
    /// 表格选中
    TableSel(usize),
    /// 滑杆变化
    SliderChanged(f32),
    /// 下拉选择
    Picked(DemoChoice),
    /// 多行编辑器动作
    Editor(iced::widget::text_editor::Action),
    /// 标题栏命令
    TitleBar(hud_widgets::title_bar::TitleBarCommand),
    /// 主窗口 Id 就绪（启动时查询）
    WindowFound(Option<iced::window::Id>),
    /// 无操作（Task 映射占位）
    Ignore,
    /// 打开第 i 个演示对话框
    OpenModal(usize),
    /// 关闭对话框（遮罩 / Esc / 按钮共用）
    CloseModal,
    /// 推送一条演示通知
    ToastPush(hud_widgets::toast::ToastKind),
    /// 通知过期清理（周期订阅）
    ToastTick,
    /// 手动关闭第 i 条通知
    ToastDismiss(usize),
    /// 扫描条推进
    LoadingTick,
    /// 切换扫描条开关
    ToggleLoading,
    /// 动画帧（页面切换淡入期间驱动重绘）
    Frame,
    /// 打开 ContextMenu
    OpenContextMenu,
    /// 关闭 ContextMenu
    CloseContextMenu,
    /// 执行 ContextMenu 项
    ContextMenuSelect(usize),
    /// ContextMenu 键盘导航
    ContextMenuKey(hud_widgets::context_menu::ContextMenuKey),
    /// 切换 Breadcrumb 项
    BreadcrumbSelect(usize),
    /// 调整 SplitPane 比例
    SplitPaneRatio(f32),
}

impl Gallery {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                page: Page::Overview,
                theme: HudTheme::dark(),
                theme_status: String::from("内置暗色主题"),
                input_value: String::new(),
                checked: false,
                tab: 0,
                list_sel: Some(1),
                table_sel: None,
                slider_v: 60.0,
                choice: Some(DemoChoice::Alpha),
                editor: iced::widget::text_editor::Content::new(),
                window: None,
                modal_open: None,
                context_menu_open: false,
                context_menu_state: hud_widgets::context_menu::ContextMenuState::new(),
                breadcrumb_selected: 2,
                split_pane_state: hud_widgets::split_pane::SplitPaneState::default(),
                toasts: hud_widgets::toast::ToastState::default(),
                loading_on: true,
                loading_phase: 0.0,
                page_anim: iced::animation::Animation::new(1.0),
            },
            // 查询主窗口 Id（单窗口应用：最老的窗口即主窗口）
            iced::window::oldest().map(Message::WindowFound),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Go(page) => {
                if self.page != page {
                    self.page = page;
                    // 简易页面切换：淡入 + 位移（120ms / ease-out，计划书 M5）
                    self.page_anim = iced::animation::Animation::new(0.0)
                        .easing(iced::animation::Easing::EaseOut)
                        .duration(std::time::Duration::from_millis(
                            self.theme.tokens.motion.feedback_ms,
                        ))
                        .go(1.0, std::time::Instant::now());
                }
            }
            Message::ThemeFileChanged => match HudTheme::from_path("assets/theme/dark.toml") {
                Ok(t) => {
                    self.theme = t;
                    self.theme_status = String::from("已从 assets/theme/dark.toml 热更新");
                }
                Err(e) => {
                    self.theme_status = format!("主题文件加载失败：{e}（继续使用上一次配置）");
                }
            },
            Message::Input(v) => self.input_value = v,
            Message::Toggled(v) => self.checked = v,
            Message::TabsGo(i) => self.tab = i,
            Message::ListSel(i) => {
                self.list_sel = if self.list_sel == Some(i) {
                    None
                } else {
                    Some(i)
                }
            }
            Message::TableSel(i) => self.table_sel = Some(i),
            Message::SliderChanged(v) => self.slider_v = v,
            Message::Picked(c) => self.choice = Some(c),
            Message::Editor(action) => self.editor.perform(action),
            Message::WindowFound(id) => self.window = id,
            Message::Ignore => {}
            Message::OpenModal(i) => self.modal_open = Some(i),
            Message::CloseModal => self.modal_open = None,
            Message::ToastPush(kind) => self.toasts.push(
                hud_widgets::toast::Toast::new(
                    kind,
                    match kind {
                        hud_widgets::toast::ToastKind::Info => "情报更新",
                        hud_widgets::toast::ToastKind::Success => "部署完成",
                        hud_widgets::toast::ToastKind::Warning => "资源阈值",
                        hud_widgets::toast::ToastKind::Danger => "链路故障",
                    },
                    "这是 Toast 通知的正文示例文本，4 秒后自动消失。",
                ),
                std::time::Instant::now(),
            ),
            Message::ToastTick => {
                self.toasts
                    .retain_fresh(std::time::Instant::now(), std::time::Duration::from_secs(4));
            }
            Message::ToastDismiss(i) => self.toasts.dismiss(i),
            Message::LoadingTick => self.loading_phase = (self.loading_phase + 0.02).fract(),
            Message::ToggleLoading => self.loading_on = !self.loading_on,
            Message::Frame => {}
            Message::OpenContextMenu => {
                self.context_menu_open = true;
                self.context_menu_state = hud_widgets::context_menu::ContextMenuState::new();
            }
            Message::CloseContextMenu => self.context_menu_open = false,
            Message::ContextMenuSelect(id) => {
                self.context_menu_open = false;
                self.theme_status = format!("ContextMenu 项 {id} 已执行");
            }
            Message::BreadcrumbSelect(id) => {
                self.breadcrumb_selected = id;
            }
            Message::SplitPaneRatio(ratio) => {
                self.split_pane_state.ratio = hud_widgets::split_pane::clamp_ratio(ratio);
            }
            Message::ContextMenuKey(key) => match key {
                hud_widgets::context_menu::ContextMenuKey::Previous => {
                    self.context_menu_state.move_previous(&[
                        hud_widgets::context_menu::ContextMenuItem::action(1, "打开 / OPEN"),
                        hud_widgets::context_menu::ContextMenuItem::action(2, "重命名 / RENAME"),
                    ]);
                }
                hud_widgets::context_menu::ContextMenuKey::Next => {
                    self.context_menu_state.move_next(&[
                        hud_widgets::context_menu::ContextMenuItem::action(1, "打开 / OPEN"),
                        hud_widgets::context_menu::ContextMenuItem::action(2, "重命名 / RENAME"),
                    ]);
                }
                hud_widgets::context_menu::ContextMenuKey::Activate => {
                    if let Some(id) = self.context_menu_state.activate(&[
                        hud_widgets::context_menu::ContextMenuItem::action(1, "打开 / OPEN"),
                        hud_widgets::context_menu::ContextMenuItem::action(2, "重命名 / RENAME"),
                    ]) {
                        return self.update(Message::ContextMenuSelect(id));
                    }
                }
                hud_widgets::context_menu::ContextMenuKey::Dismiss => {
                    self.context_menu_open = false;
                }
            },
            Message::TitleBar(cmd) => {
                let Some(id) = self.window else {
                    return Task::none();
                };
                let window_cmd = match cmd {
                    hud_widgets::title_bar::TitleBarCommand::Drag => {
                        hud_platform::WindowCommand::Drag
                    }
                    hud_widgets::title_bar::TitleBarCommand::ToggleMaximize => {
                        hud_platform::WindowCommand::ToggleMaximize
                    }
                    hud_widgets::title_bar::TitleBarCommand::Minimize => {
                        hud_platform::WindowCommand::Minimize
                    }
                    hud_widgets::title_bar::TitleBarCommand::Close => {
                        hud_platform::WindowCommand::Close
                    }
                };
                return hud_platform::perform(id, window_cmd).map(|_| Message::Ignore);
            }
        }
        Task::none()
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        use std::time::Duration;
        let mut subs: Vec<iced::Subscription<Message>> = Vec::new();

        // 主题热更新仅在 debug 构建开启（计划书 M2）；release 订阅为空以省电
        #[cfg(debug_assertions)]
        subs.push(theme_watcher());

        // M5：仅在需要时订阅（计划书 2.5）
        if self.modal_open.is_some() {
            subs.push(hud_widgets::modal::escape_listener(Message::CloseModal));
        }
        if self.context_menu_open {
            subs.push(hud_widgets::context_menu::keyboard_listener().map(Message::ContextMenuKey));
        }
        if !self.toasts.is_empty() {
            subs.push(iced::time::every(Duration::from_millis(250)).map(|_| Message::ToastTick));
        }
        if self.loading_on {
            subs.push(iced::time::every(Duration::from_millis(16)).map(|_| Message::LoadingTick));
        }
        if self.page_anim.is_animating(std::time::Instant::now()) {
            subs.push(iced::window::frames().map(|_| Message::Frame));
        }

        iced::Subscription::batch(subs)
    }

    fn view(&self) -> Element<'_, Message> {
        let nav = self.side_nav();

        // 简易页面切换：淡入（遮罩淡出）+ 位移（padding 8→0），计划书 M5
        let now = std::time::Instant::now();
        let alpha = self.page_anim.interpolate_with(|v| v, now);
        let dy = (1.0 - alpha) * 8.0;
        let fade_scrim = hud_theme::color(self.theme.tokens.palette.bg0).scale_alpha(1.0 - alpha);

        let content = column![self.page_title(), self.page_body()]
            .spacing(24)
            .padding(24)
            .max_width(960);

        let content =
            iced::widget::Stack::<Message, HudTheme, iced::Renderer>::with_children(vec![
                container(content)
                    .width(Length::Fill)
                    .height(Length::Shrink)
                    .padding(iced::Padding {
                        top: dy,
                        right: 0.0,
                        bottom: 0.0,
                        left: 0.0,
                    })
                    .into(),
                container(text(""))
                    .width(Length::Fill)
                    .height(Length::Shrink)
                    .style(move |_theme: &HudTheme| iced::widget::container::Style {
                        background: Some(fade_scrim.into()),
                        ..Default::default()
                    })
                    .into(),
            ]);

        let base = row![nav, content]
            .width(Length::Fill)
            .height(Length::Fill)
            .into();

        // M5：模态层（对话框打开时叠加遮罩与对话框）
        match self.modal_open {
            Some(i) => {
                let dialog = self.demo_dialog(i);
                hud_widgets::modal::modal(base, dialog, &self.theme.tokens, &|| Message::CloseModal)
            }
            None => base,
        }
    }

    /// 演示对话框内容（0：确认；1：表单）。
    fn demo_dialog(&self, kind: usize) -> Element<'_, Message> {
        use hud_widgets::button::{class, ButtonVariant};
        use hud_widgets::typography::{card, secondary_text};

        let tokens = self.theme.tokens;
        let (title, title_en) = if kind == 0 {
            ("确认操作", "CONFIRM")
        } else {
            ("输入代号", "INPUT CALLSIGN")
        };

        let body: Element<'_, Message> = if kind == 0 {
            column![
                text("即将断开当前链路，是否继续？").size(14),
                row![
                    button(text("取消 / CANCEL").size(14))
                        .padding(8)
                        .on_press(Message::CloseModal)
                        .class(class(ButtonVariant::Secondary)),
                    button(text("确认 / CONFIRM").size(14))
                        .padding(8)
                        .on_press(Message::CloseModal)
                        .class(class(ButtonVariant::Primary)),
                ]
                .spacing(12),
            ]
            .spacing(16)
            .into()
        } else {
            column![
                text_input("CALLSIGN", &self.input_value)
                    .on_input(Message::Input)
                    .padding(8)
                    .width(240),
                button(text("提交 / SUBMIT").size(14))
                    .padding(8)
                    .on_press(Message::CloseModal)
                    .class(class(ButtonVariant::Primary)),
            ]
            .spacing(16)
            .into()
        };

        card(
            &tokens,
            title,
            title_en,
            column![
                body,
                text("Esc 或点击遮罩关闭").size(12).class(secondary_text())
            ]
            .spacing(8)
            .into(),
        )
    }

    fn side_nav(&self) -> Element<'_, Message> {
        let items: Vec<hud_widgets::nav::NavItem> = Page::ALL
            .iter()
            .map(|p| hud_widgets::nav::NavItem {
                id: 0,
                label: p.title().to_string(),
            })
            .enumerate()
            .map(|(id, mut item)| {
                item.id = id;
                item
            })
            .collect();
        let active = Page::ALL.iter().position(|p| *p == self.page).unwrap_or(0);
        hud_widgets::nav::side_nav(&items, active, &self.theme.tokens, 220.0, &|i| {
            Message::Go(Page::ALL[i])
        })
    }

    fn page_title(&self) -> Element<'_, Message> {
        column![
            text(self.page.title()).size(28),
            text(self.theme_status.as_str()).size(12),
        ]
        .spacing(4)
        .into()
    }

    fn page_body(&self) -> Element<'_, Message> {
        match self.page {
            Page::Overview => self.overview_page(),
            Page::Tokens => self.tokens_page(),
            Page::Shapes => self.shapes_page(),
            Page::Components => self.components_page(),
            Page::Overlays => self.overlays_page(),
            Page::About => self.about_page(),
        }
    }

    fn shapes_page(&self) -> Element<'_, Message> {
        use hud_widgets::decor::{CornerBrackets, GridBackground, SlantedStripes, TickDivider};
        use hud_widgets::panel::ChamferLevel;

        let tokens = self.theme.tokens;

        fn cell<'a>(label: &'static str, inner: Element<'a, Message>) -> Element<'a, Message> {
            column![inner, text(label).size(12)].spacing(6).into()
        }

        // 三个切角档位面板
        let chamfer_panel = |level: ChamferLevel, label: String| {
            hud_widgets::panel::panel(text(label).size(14), &tokens, level)
                .padding(16)
                .width(150)
                .height(72)
        };

        let brackets: Element<'_, Message> = container(
            canvas(CornerBrackets::new().accent())
                .width(Length::Fill)
                .height(Length::Fill),
        )
        .width(150)
        .height(72)
        .into();

        let grid: Element<'_, Message> = container(
            canvas(GridBackground::default())
                .width(Length::Fill)
                .height(Length::Fill),
        )
        .width(230)
        .height(72)
        .into();

        let stripes: Element<'_, Message> = container(
            canvas(SlantedStripes::default().accent())
                .width(Length::Fill)
                .height(Length::Fill),
        )
        .width(150)
        .height(72)
        .into();

        let divider: Element<'_, Message> = canvas(TickDivider::default())
            .width(Length::Fill)
            .height(16)
            .into();

        column![
            text("切角档位 / CHAMFER SIZES").size(12),
            row![
                chamfer_panel(ChamferLevel::S, "4px".into()),
                chamfer_panel(ChamferLevel::M, "8px".into()),
                chamfer_panel(ChamferLevel::L, "12px".into()),
            ]
            .spacing(12),
            rule::horizontal(1),
            text("角标 / CORNER BRACKETS").size(12),
            row![
                cell("accent", brackets),
                cell(
                    "常规",
                    container(canvas(CornerBrackets::default()).width(Length::Fill).height(Length::Fill))
                        .width(150)
                        .height(72)
                        .into()
                ),
                grid,
                stripes,
            ]
            .spacing(12),
            rule::horizontal(1),
            text("刻度分隔线 / TICK DIVIDER").size(12),
            divider,
            rule::horizontal(1),
            text("像素对齐 / PIXEL SNAP").size(12),
            text(format!(
                "snap_to_pixel(10.3, 1.25) = {:.2}（物理像素对齐工具，几何单测覆盖 1.0/1.25/1.5/2.0）",
                hud_core::snap_to_pixel(10.3, 1.25)
            ))
            .size(14),
        ]
        .spacing(16)
        .into()
    }

    fn components_page(&self) -> Element<'_, Message> {
        use hud_widgets::breadcrumb::{breadcrumb, BreadcrumbItem};
        use hud_widgets::button::{class, ButtonVariant};
        use hud_widgets::context_menu::{context_menu, ContextMenuItem};
        use hud_widgets::decor::{CornerBrackets, SlantedStripes};
        use hud_widgets::input::{select, tooltip};
        use hud_widgets::list::{list, ListItem};
        use hud_widgets::progress::{SegmentedProgress, SlantedProgress};
        use hud_widgets::table::{table, TableColumn};
        use hud_widgets::tabs::tabs;
        use hud_widgets::title_bar::title_bar;
        use hud_widgets::typography::{card, secondary_text, section_header};

        let tokens = self.theme.tokens;
        let breadcrumb_items = [
            BreadcrumbItem::new(0, "首页 / HOME"),
            BreadcrumbItem::new(1, "系统 / SYSTEM"),
            BreadcrumbItem::new(2, "组件 / COMPONENTS"),
        ];
        let breadcrumb_demo = breadcrumb(
            breadcrumb_items.to_vec(),
            self.breadcrumb_selected,
            &Message::BreadcrumbSelect,
        );
        let context_items = [
            ContextMenuItem::action(1, "打开 / OPEN"),
            ContextMenuItem::action(2, "重命名 / RENAME"),
            ContextMenuItem::separator(3),
            ContextMenuItem::disabled(4, "删除 / DELETE（禁用）"),
        ];
        let split_pane_demo = hud_widgets::split_pane::split_pane(
            &self.split_pane_state,
            container(column![
                text("左侧 / PRIMARY").size(13),
                text("节点列表区域").size(12),
            ])
            .padding(12)
            .into(),
            container(column![
                text("右侧 / DETAIL").size(13),
                text("详情与操作区域").size(12),
            ])
            .padding(12)
            .into(),
            Message::SplitPaneRatio,
        );
        let split_pane_demo = column![
            text(format!(
                "SplitPane / 分栏 {:.0}%",
                self.split_pane_state.ratio * 100.0
            ))
            .size(14),
            container(split_pane_demo).height(120),
        ]
        .spacing(8);
        let context_menu_demo = column![
            text("ContextMenu / 受控上下文菜单").size(14),
            button(text(if self.context_menu_open {
                "关闭菜单 / CLOSE MENU"
            } else {
                "右键或点击打开 / OPEN MENU"
            }))
            .padding(8)
            .on_press(if self.context_menu_open {
                Message::CloseContextMenu
            } else {
                Message::OpenContextMenu
            }),
            if self.context_menu_open {
                context_menu(
                    context_items.to_vec(),
                    self.context_menu_state,
                    &Message::ContextMenuSelect,
                )
            } else {
                text("菜单关闭时不渲染浮层").size(12).into()
            },
        ]
        .spacing(8);

        // ---- M4.3 TitleBar（作用于本窗口；无边框窗口场景见组件文档） ----
        let title_bar_demo = title_bar(
            "hud-ui gallery — 标题栏演示（按钮真实作用于本窗口）",
            &tokens,
            &Message::TitleBar,
        );

        // ---- M4.1 按钮 ----
        let variant_buttons = |variant: ButtonVariant, label: &'static str| {
            button(text(label).size(14))
                .padding(10)
                .on_press(Message::Go(self.page))
                .class(class(variant))
        };
        let buttons = row![
            variant_buttons(ButtonVariant::Primary, "主操作 / PRIMARY"),
            variant_buttons(ButtonVariant::Secondary, "次要 / SECONDARY"),
            button(text("幽灵 / GHOST").size(14))
                .padding(10)
                .class(class(ButtonVariant::Ghost)),
            variant_buttons(ButtonVariant::Danger, "危险 / DANGER"),
        ]
        .spacing(12);

        // ---- M4.2 输入类 ----
        let slider_value = text(format!("音量 {:.0}", self.slider_v)).size(12);
        let inputs = column![
            text_input("单行输入 / TEXT INPUT", &self.input_value)
                .on_input(Message::Input)
                .padding(8)
                .width(280),
            row![
                checkbox(self.checked)
                    .label("复选框 / CHECKBOX")
                    .on_toggle(Message::Toggled),
                toggler(self.checked).on_toggle(Message::Toggled),
                text("开关 / TOGGLER").size(14),
            ]
            .spacing(12)
            .align_y(iced::alignment::Vertical::Center),
            row![
                slider(0.0..=100.0, self.slider_v, Message::SliderChanged).width(180),
                slider_value,
            ]
            .spacing(12)
            .align_y(iced::alignment::Vertical::Center),
            select(
                &[DemoChoice::Alpha, DemoChoice::Bravo, DemoChoice::Charlie],
                self.choice,
                Message::Picked,
            ),
            tooltip(
                button(text("悬停我 / HOVER").size(14)).padding(8).into(),
                "Tooltip 底部弹出提示",
            ),
        ]
        .spacing(12);

        let editor = text_editor(&self.editor)
            .height(90)
            .on_action(Message::Editor);

        // ---- M4.3 Tabs / List ----
        let tab_labels = ["概览 / INFO", "日志 / LOG", "设置 / CFG"];
        let tab_body: Element<'_, Message> = match self.tab {
            0 => text("页签内容 A — 概览信息").size(14).into(),
            1 => text("页签内容 B — 日志输出").size(14).into(),
            _ => text("页签内容 C — 配置项").size(14).into(),
        };
        let tabs_demo = column![
            tabs(&tab_labels, self.tab, &tokens, &Message::TabsGo),
            container(tab_body).padding(12),
        ]
        .spacing(0);

        let list_items = [
            ListItem {
                id: 0,
                label: String::from("01 — 前线观测站"),
                meta: Some(String::from("在线")),
            },
            ListItem {
                id: 1,
                label: String::from("02 — 后勤补给线"),
                meta: Some(String::from("在线")),
            },
            ListItem {
                id: 2,
                label: String::from("03 — 通讯中继"),
                meta: Some(String::from("警告")),
            },
            ListItem {
                id: 3,
                label: String::from("04 — 电力网格"),
                meta: Some(String::from("离线")),
            },
        ];
        let list_demo = list(&list_items, self.list_sel, &Message::ListSel);

        // ---- M4.4 Table / Badge / Progress ----
        struct Task {
            code: &'static str,
            name: &'static str,
            status: &'static str,
            load: f32,
        }
        let tasks = [
            Task {
                code: "T-101",
                name: "资源回收",
                status: "进行中",
                load: 0.72,
            },
            Task {
                code: "T-102",
                name: "补给装载",
                status: "等待",
                load: 0.30,
            },
            Task {
                code: "T-103",
                name: "设施检修",
                status: "完成",
                load: 1.00,
            },
        ];
        let columns = [
            TableColumn {
                title: "编号 / CODE",
                width: 90.0,
                cell: |t: &Task| t.code.to_string(),
            },
            TableColumn {
                title: "任务 / TASK",
                width: 130.0,
                cell: |t: &Task| t.name.to_string(),
            },
            TableColumn {
                title: "状态 / STATUS",
                width: 90.0,
                cell: |t: &Task| t.status.to_string(),
            },
            TableColumn {
                title: "负载 / LOAD",
                width: 80.0,
                cell: |t: &Task| format!("{:.0}%", t.load * 100.0),
            },
        ];
        let table_demo = table(
            &columns,
            &tasks,
            &tokens,
            self.table_sel,
            Some(&Message::TableSel),
        );

        let badges = row![
            hud_widgets::badge::badge("常规 / COMMON", hud_widgets::badge::BadgeKind::Neutral),
            hud_widgets::badge::badge("信息 / INFO", hud_widgets::badge::BadgeKind::Info),
            hud_widgets::badge::badge("完成 / DONE", hud_widgets::badge::BadgeKind::Success),
            hud_widgets::badge::badge("警告 / WARN", hud_widgets::badge::BadgeKind::Warning),
            hud_widgets::badge::badge("故障 / FAULT", hud_widgets::badge::BadgeKind::Danger),
        ]
        .spacing(12);

        let v = self.slider_v / 100.0;
        let thin_bar = |value: f32| {
            container(iced::widget::progress_bar(0.0..=1.0, value))
                .height(tokens.geometry.space.s)
                .width(Length::Fill)
        };
        let progress_demo = column![
            thin_bar(v),
            canvas(SegmentedProgress::new(v, 10))
                .width(Length::Fill)
                .height(tokens.geometry.space.s),
            canvas(SlantedProgress::new(v))
                .width(Length::Fill)
                .height(tokens.geometry.space.s),
        ]
        .spacing(8);

        let progress_card_body = column![
            thin_bar(v),
            text(format!("当前演示进度 {:.0}%", v * 100.0))
                .size(12)
                .class(secondary_text()),
        ]
        .spacing(4)
        .into();

        // 卡片 + 装饰角标
        let decorated_card: Element<'_, Message> = {
            let inner = column![
                section_header("装饰卡片", "DECORATED CARD"),
                text("角标 + 斜纹条组合示例（装饰可整体关闭）。").size(13),
            ]
            .spacing(8);
            hud_widgets::panel::panel(inner, &tokens, hud_widgets::panel::ChamferLevel::M)
                .padding(16)
                .width(320)
                .into()
        };
        let decorated: Element<'_, Message> = container(stack![
            decorated_card,
            canvas(CornerBrackets::new().accent()).width(320).height(96),
        ])
        .width(320)
        .into();

        let _ = SlantedStripes::default(); // 斜纹条已在形状页展示

        // 总装：外层 scrollable（M4.2 ScrollArea 演示兼页面容器）
        scrollable(
            column![
                title_bar_demo,
                section_header("导航", "NAVIGATION"),
                breadcrumb_demo,
                rule::horizontal(1),
                section_header("基础组件", "CORE COMPONENTS"),
                buttons,
                rule::horizontal(1),
                section_header("输入", "INPUTS"),
                inputs,
                editor,
                rule::horizontal(1),
                section_header("上下文菜单", "CONTEXT MENU"),
                context_menu_demo,
                rule::horizontal(1),
                section_header("分栏", "SPLIT PANE"),
                split_pane_demo,
                rule::horizontal(1),
                section_header("页签", "TABS"),
                tabs_demo,
                rule::horizontal(1),
                section_header("列表", "LIST"),
                list_demo,
                rule::horizontal(1),
                section_header("表格", "TABLE"),
                table_demo,
                rule::horizontal(1),
                section_header("徽标", "BADGE / TAG"),
                badges,
                rule::horizontal(1),
                section_header("进度", "PROGRESS"),
                progress_demo,
                rule::horizontal(1),
                section_header("卡片", "CARD"),
                row![
                    card(&tokens, "卡片", "CARD / WITH CHAMFER", progress_card_body),
                    decorated,
                ]
                .spacing(16),
            ]
            .spacing(20)
            .padding(0),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    fn overlays_page(&self) -> Element<'_, Message> {
        use hud_widgets::loading::ScanBar;
        use hud_widgets::status_panel::{status_panel, OpStatus};
        use hud_widgets::toast::{toast_list, ToastKind};
        use hud_widgets::typography::{secondary_text, section_header};

        let tokens = self.theme.tokens;

        // Modal：两个打开按钮（真实作用于本页顶层）
        let modal_demo = row![
            button(text("确认对话框 / CONFIRM").size(14))
                .padding(10)
                .on_press(Message::OpenModal(0)),
            button(text("表单对话框 / FORM").size(14))
                .padding(10)
                .on_press(Message::OpenModal(1)),
        ]
        .spacing(12);

        // Toast：四类推送 + 当前队列
        let toast_push = row![
            button(text("信息 / INFO").size(14))
                .padding(8)
                .on_press(Message::ToastPush(ToastKind::Info)),
            button(text("成功 / SUCCESS").size(14))
                .padding(8)
                .on_press(Message::ToastPush(ToastKind::Success)),
            button(text("警告 / WARNING").size(14))
                .padding(8)
                .on_press(Message::ToastPush(ToastKind::Warning)),
            button(text("故障 / DANGER").size(14))
                .padding(8)
                .on_press(Message::ToastPush(ToastKind::Danger)),
        ]
        .spacing(12);
        let toast_queue: Element<'_, Message> = if self.toasts.is_empty() {
            text("队列为空（推送后 4 秒自动消失，可手动关闭）")
                .size(12)
                .class(secondary_text())
                .into()
        } else {
            toast_list(&self.toasts, &tokens, Some(&Message::ToastDismiss))
        };

        // Loading：扫描条（唯一循环动效，仅可见时订阅）
        let loading_toggle = button(
            text(if self.loading_on {
                "停止 / STOP"
            } else {
                "开始 / START"
            })
            .size(14),
        )
        .padding(8)
        .on_press(Message::ToggleLoading);
        let loading_demo = column![
            row![
                loading_toggle,
                text(if self.loading_on {
                    "扫描中（16ms 帧推进）"
                } else {
                    "已停止（无订阅，零开销）"
                })
                .size(12)
                .class(secondary_text()),
            ]
            .spacing(12)
            .align_y(iced::alignment::Vertical::Center),
            canvas(ScanBar::new(self.loading_phase))
                .width(Length::Fill)
                .height(tokens.geometry.space.m),
        ]
        .spacing(8);

        // StatusPanel：三种运行状态
        let panels = row![
            status_panel(
                "链路 A / LINK-A",
                "延迟 12ms · 丢包 0%",
                OpStatus::Online,
                &tokens,
                200.0
            ),
            status_panel(
                "链路 B / LINK-B",
                "延迟 180ms · 抖动",
                OpStatus::Warning,
                &tokens,
                200.0
            ),
            status_panel(
                "链路 C / LINK-C",
                "无响应 · 已重试 3 次",
                OpStatus::Offline,
                &tokens,
                200.0
            ),
        ]
        .spacing(12);

        // DataReadout：等宽数字读数
        let readouts = row![
            hud_widgets::data_readout::data_readout("能源 / POWER", "87.4", Some("%"), &tokens),
            hud_widgets::data_readout::data_readout("温度 / TEMP", "41.2", Some("°C"), &tokens),
            hud_widgets::data_readout::data_readout("信号 / SIGNAL", "1024", Some("dBm"), &tokens),
        ]
        .spacing(24);

        scrollable(
            column![
                section_header("模态对话框", "MODAL / DIALOG"),
                text("Esc 或点击遮罩关闭；键盘 Tab 循环由应用处理。")
                    .size(12)
                    .class(secondary_text()),
                modal_demo,
                rule::horizontal(1),
                section_header("通知队列", "TOAST / NOTIFICATION"),
                toast_push,
                toast_queue,
                rule::horizontal(1),
                section_header("加载扫描条", "LOADING"),
                loading_demo,
                rule::horizontal(1),
                section_header("状态面板", "STATUS PANEL"),
                panels,
                rule::horizontal(1),
                section_header("数据读数", "DATA READOUT"),
                readouts,
            ]
            .spacing(20),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    fn overview_page(&self) -> Element<'_, Message> {
        column![
            text("hud-ui 组件陈列馆").size(20),
            text("左侧导航切换页面；本窗口同时是视觉回归与走查的载体。").size(14),
        ]
        .spacing(12)
        .into()
    }

    fn tokens_page(&self) -> Element<'_, Message> {
        let p = &self.theme.tokens.palette;
        let swatches: Vec<Element<'_, Message>> = [
            ("bg.0", p.bg0),
            ("bg.1", p.bg1),
            ("bg.2", p.bg2),
            ("line.dim", p.line_dim),
            ("line.base", p.line_base),
            ("line.strong", p.line_strong),
            ("text.primary", p.text_primary),
            ("text.secondary", p.text_secondary),
            ("text.disabled", p.text_disabled),
            ("accent", p.accent),
            ("info", p.info),
            ("danger", p.danger),
            ("success", p.success),
        ]
        .iter()
        .map(|(name, c)| {
            let hex = format!("#{:02X}{:02X}{:02X}", c.r, c.g, c.b);
            let fill = *c;
            column![
                container(text(""))
                    .width(96)
                    .height(40)
                    .class(Box::new(move |t: &HudTheme| {
                        container::Style {
                            background: Some(hud_theme::color(fill).into()),
                            border: iced::Border {
                                color: hud_theme::color(t.tokens.palette.line_base),
                                width: t.tokens.geometry.line.base,
                                radius: 0.0.into(),
                            },
                            ..Default::default()
                        }
                    }) as container::StyleFn<'_, HudTheme>),
                text(*name).size(12),
                text(hex).size(12),
            ]
            .spacing(2)
            .into()
        })
        .collect();

        let typography = column![
            text("字号阶梯 / TYPE SCALE").size(12),
            text("12px 辅助小字 — THE QUICK BROWN FOX").size(12),
            text("14px 正文 — 快速的棕色狐狸跳过懒惰的狗").size(14),
            text("16px 强调正文 — HUD 风格界面语言").size(16),
            text("20px 小标题 — HUD 风格界面语言").size(20),
            text("28px 页面标题").size(28),
        ]
        .spacing(8);

        let geo = &self.theme.tokens.geometry;
        let geometry_info = column![
            text("几何 / GEOMETRY").size(12),
            text(format!(
                "间距 {} / {} / {} / {} / {} / {}",
                geo.space.xs, geo.space.s, geo.space.m, geo.space.l, geo.space.xl, geo.space.xxl
            ))
            .size(14),
            text(format!(
                "切角 {} / {} / {}",
                geo.chamfer.s, geo.chamfer.m, geo.chamfer.l
            ))
            .size(14),
            text(format!("线宽 {} / {}", geo.line.base, geo.line.strong)).size(14),
            text(format!(
                "动效 {} / {} / {} ms",
                self.theme.tokens.motion.feedback_ms,
                self.theme.tokens.motion.transition_ms,
                self.theme.tokens.motion.overlay_ms
            ))
            .size(14),
        ]
        .spacing(6);

        column![
            text("色板 / PALETTE").size(12),
            iced::widget::Row::<Message, HudTheme, iced::Renderer>::with_children(swatches)
                .spacing(8)
                .wrap(),
            rule::horizontal(1),
            typography,
            rule::horizontal(1),
            geometry_info,
        ]
        .spacing(16)
        .into()
    }

    fn about_page(&self) -> Element<'_, Message> {
        column![
            text("基于 iced 0.14 的 HUD 风格组件库").size(16),
            text("视觉语言：深色底、细线、45° 切角、克制强调。").size(14),
            text("受特定科幻游戏界面风格启发，不使用其任何美术资产、Logo 与字体。").size(12),
        ]
        .spacing(10)
        .into()
    }
}
