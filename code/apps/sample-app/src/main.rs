#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

//! 示例应用「链路监控台」：以真实业务流程走通组件库（计划书 M6）。
//!
//! 四页面：首页仪表盘、节点列表 + 详情、设置（表单）、对话流程（确认 / 错误 / 加载）。
//! 与 Gallery（组件 × 状态走查）不同，本应用展示组件在业务语境下的组合方式。

mod pages;

use std::time::{Duration, Instant};

use hud_theme::HudTheme;
use hud_widgets::{
    column,
    toast::{Toast, ToastKind, ToastState},
};

fn main() -> iced::Result {
    iced::application(Sample::new, Sample::update, Sample::view)
        .title(|_: &Sample| String::from("hud-ui sample-app"))
        .theme(|s: &Sample| s.theme.clone())
        .subscription(Sample::subscription)
        .window_size((1080.0, 720.0))
        .run()
}

/// 受监控节点记录（演示数据）。
pub struct NodeRecord {
    /// 节点代号
    pub name: String,
    /// 部署区域
    pub region: String,
    /// 负载 0.0..=1.0
    pub load: f32,
    /// 链路延迟（毫秒）
    pub latency_ms: u32,
    /// 是否在线
    pub online: bool,
}

/// 设置页表单值。
pub struct SettingsForm {
    /// 操作台代号
    pub callsign: String,
    /// 默认区域（索引指向 [`Sample::REGIONS`]）
    pub region: usize,
    /// 断线自动重连
    pub auto_reconnect: bool,
    /// 告警通知
    pub notify: bool,
    /// 数据刷新间隔（秒）
    pub refresh_secs: f32,
}

/// 模态对话框种类。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModalKind {
    /// 确认断开主链路
    DisconnectLink,
    /// 确认重启节点（携带节点索引）
    RestartNode(usize),
}

/// 应用消息。
#[derive(Debug, Clone, PartialEq)]
pub enum Message {
    /// 侧边导航切换
    Nav(pages::Page),
    /// 节点列表选中
    NodeSelect(usize),
    /// 请求断开主链路（打开确认模态）
    AskDisconnect,
    /// 请求重启节点（打开确认模态）
    AskRestart(usize),
    /// 确认当前模态动作
    ConfirmModal,
    /// 关闭模态（遮罩 / Esc / 取消）
    CloseModal,
    /// 模拟链路故障（错误流程演示）
    DemoError,
    /// 重建主链路（错误/断开后的恢复动作）
    RestoreLink,
    /// 重新同步（加载流程演示）
    DemoLoad,
    /// 加载流程推进（16ms）
    LoadTick,
    /// 选择节点上的 Breadcrumb
    BreadcrumbSelect(usize),
    /// 调整节点列表与详情的分栏比例
    SplitPaneRatio(f32),
    /// 打开节点 ContextMenu
    OpenNodeContextMenu,
    /// 关闭节点 ContextMenu
    CloseNodeContextMenu,
    /// 执行节点 ContextMenu 项
    NodeContextMenuSelect(usize),
    /// ContextMenu 键盘导航
    NodeContextMenuKey(hud_widgets::context_menu::ContextMenuKey),
    /// Toast 过期清理（250ms）
    ToastTick,
    /// 手动关闭第 index 条 Toast
    ToastDismiss(usize),
    /// 表单：代号输入
    CallsignInput(String),
    /// 表单：区域选择
    RegionSelected(&'static str),
    /// 表单：自动重连
    AutoReconnect(bool),
    /// 表单：告警通知
    Notify(bool),
    /// 表单：刷新间隔
    RefreshChanged(f32),
    /// 表单：应用设置
    ApplySettings,
}

/// 应用状态。
pub struct Sample {
    /// 主题（固定 dark；字段化以便渲染层取 Token）
    pub theme: HudTheme,
    /// 当前页面
    pub page: pages::Page,
    /// 主链路是否在线（对话流程会切换）
    pub link_online: bool,
    /// 节点数据
    pub nodes: Vec<NodeRecord>,
    /// 列表选中节点
    pub selected: Option<usize>,
    /// Breadcrumb 当前节点路径索引
    pub breadcrumb_selected: usize,
    /// 节点列表与详情的分栏比例
    pub split_pane_state: hud_widgets::split_pane::SplitPaneState,
    /// 节点 ContextMenu 是否打开
    pub node_context_menu_open: bool,
    /// 节点 ContextMenu 键盘状态
    pub node_context_menu_state: hud_widgets::context_menu::ContextMenuState,
    /// 设置表单
    pub form: SettingsForm,
    /// Toast 队列
    pub toasts: ToastState,
    /// 打开的模态
    pub modal: Option<ModalKind>,
    /// 加载流程起点（Some 表示加载中）
    pub loading: Option<Instant>,
    /// 扫描条相位（16ms 推进，驱动重绘）
    pub loading_phase: f32,
}

impl Sample {
    /// 区域选项（设置页下拉）。
    pub const REGIONS: [&'static str; 4] = ["扇区 A", "扇区 B", "边界站", "深空中继"];

    /// Toast 存活时长。
    pub const TOAST_TTL: Duration = Duration::from_secs(4);

    fn new() -> (Self, iced::Task<Message>) {
        let nodes = vec![
            NodeRecord {
                name: String::from("NX-01"),
                region: String::from("扇区 A"),
                load: 0.62,
                latency_ms: 24,
                online: true,
            },
            NodeRecord {
                name: String::from("NX-02"),
                region: String::from("扇区 A"),
                load: 0.35,
                latency_ms: 31,
                online: true,
            },
            NodeRecord {
                name: String::from("NX-03"),
                region: String::from("扇区 B"),
                load: 0.81,
                latency_ms: 47,
                online: true,
            },
            NodeRecord {
                name: String::from("NX-04"),
                region: String::from("边界站"),
                load: 0.94,
                latency_ms: 88,
                online: true,
            },
            NodeRecord {
                name: String::from("NX-05"),
                region: String::from("深空中继"),
                load: 0.0,
                latency_ms: 0,
                online: false,
            },
        ];

        let state = Self {
            theme: HudTheme::dark(),
            page: pages::Page::Dashboard,
            link_online: true,
            selected: Some(0),
            breadcrumb_selected: 1,
            split_pane_state: hud_widgets::split_pane::SplitPaneState::default(),
            node_context_menu_open: false,
            node_context_menu_state: hud_widgets::context_menu::ContextMenuState::new(),
            nodes,
            form: SettingsForm {
                callsign: String::from("WATCH-07"),
                region: 0,
                auto_reconnect: true,
                notify: true,
                refresh_secs: 5.0,
            },
            toasts: ToastState::default(),
            modal: None,
            loading: None,
            loading_phase: 0.0,
        };
        (state, iced::Task::none())
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        use std::time::Duration;
        let mut subs: Vec<iced::Subscription<Message>> = Vec::new();

        // 计划书 2.5：仅在需要时订阅
        if self.modal.is_some() {
            subs.push(hud_widgets::modal::escape_listener(Message::CloseModal));
        }
        if !self.toasts.is_empty() {
            subs.push(iced::time::every(Duration::from_millis(250)).map(|_| Message::ToastTick));
        }
        if self.loading.is_some() {
            subs.push(iced::time::every(Duration::from_millis(16)).map(|_| Message::LoadTick));
        }
        if self.node_context_menu_open {
            subs.push(
                hud_widgets::context_menu::keyboard_listener().map(Message::NodeContextMenuKey),
            );
        }

        iced::Subscription::batch(subs)
    }

    fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::Nav(page) => self.page = page,
            Message::NodeSelect(i) => self.selected = Some(i),
            Message::BreadcrumbSelect(i) => self.breadcrumb_selected = i,
            Message::SplitPaneRatio(ratio) => {
                self.split_pane_state.ratio = hud_widgets::split_pane::clamp_ratio(ratio);
            }
            Message::OpenNodeContextMenu => {
                self.node_context_menu_open = true;
                self.node_context_menu_state = hud_widgets::context_menu::ContextMenuState::new();
            }
            Message::CloseNodeContextMenu => self.node_context_menu_open = false,
            Message::NodeContextMenuSelect(i) => {
                self.node_context_menu_open = false;
                self.push_toast(
                    ToastKind::Info,
                    "节点操作",
                    &format!("已执行菜单项 {i}。"),
                    Instant::now(),
                );
            }
            Message::NodeContextMenuKey(key) => {
                let items = [
                    hud_widgets::context_menu::ContextMenuItem::action(1, "查看详情"),
                    hud_widgets::context_menu::ContextMenuItem::action(2, "复制节点 ID"),
                ];
                match key {
                    hud_widgets::context_menu::ContextMenuKey::Previous => {
                        self.node_context_menu_state.move_previous(&items)
                    }
                    hud_widgets::context_menu::ContextMenuKey::Next => {
                        self.node_context_menu_state.move_next(&items)
                    }
                    hud_widgets::context_menu::ContextMenuKey::Activate => {
                        if let Some(i) = self.node_context_menu_state.activate(&items) {
                            return self.update(Message::NodeContextMenuSelect(i));
                        }
                    }
                    hud_widgets::context_menu::ContextMenuKey::Dismiss => {
                        self.node_context_menu_open = false
                    }
                }
            }
            Message::AskDisconnect => self.modal = Some(ModalKind::DisconnectLink),
            Message::AskRestart(i) => self.modal = Some(ModalKind::RestartNode(i)),
            Message::ConfirmModal => {
                let now = Instant::now();
                match self.modal {
                    Some(ModalKind::DisconnectLink) => {
                        self.link_online = false;
                        self.push_toast(
                            ToastKind::Warning,
                            "链路已断开",
                            "主链路已按确认指令断开。",
                            now,
                        );
                    }
                    Some(ModalKind::RestartNode(i)) => {
                        if let Some(node) = self.nodes.get_mut(i) {
                            node.load = 0.0;
                            node.online = true;
                            node.latency_ms = node.latency_ms.max(18);
                        }
                        self.push_toast(
                            ToastKind::Success,
                            "重启完成",
                            "节点已恢复，负载归零。",
                            now,
                        );
                    }
                    None => {}
                }
                self.modal = None;
            }
            Message::CloseModal => self.modal = None,
            Message::DemoError => {
                self.link_online = false;
                self.push_toast(
                    ToastKind::Danger,
                    "链路故障",
                    "心跳丢失，主链路已标记为离线。",
                    Instant::now(),
                );
            }
            Message::RestoreLink => {
                self.link_online = true;
                self.push_toast(
                    ToastKind::Success,
                    "链路恢复",
                    "主链路已重建，流量切回主用路由。",
                    Instant::now(),
                );
            }
            Message::DemoLoad => {
                if self.loading.is_none() {
                    self.loading = Some(Instant::now());
                }
            }
            Message::LoadTick => {
                self.loading_phase = (self.loading_phase + 0.02).fract();
                if let Some(started) = self.loading {
                    if started.elapsed() >= Duration::from_secs(2) {
                        self.loading = None;
                        self.push_toast(
                            ToastKind::Success,
                            "同步完成",
                            "全部节点数据已更新。",
                            Instant::now(),
                        );
                    }
                }
            }
            Message::ToastTick => {
                self.toasts.retain_fresh(Instant::now(), Self::TOAST_TTL);
            }
            Message::ToastDismiss(i) => self.toasts.dismiss(i),
            Message::CallsignInput(v) => self.form.callsign = v,
            Message::RegionSelected(region) => {
                self.form.region = Self::REGIONS.iter().position(|r| *r == region).unwrap_or(0);
            }
            Message::AutoReconnect(v) => self.form.auto_reconnect = v,
            Message::Notify(v) => self.form.notify = v,
            Message::RefreshChanged(v) => self.form.refresh_secs = v,
            Message::ApplySettings => {
                self.push_toast(
                    ToastKind::Success,
                    "设置已保存",
                    "新的监控参数已下发到全部节点。",
                    Instant::now(),
                );
            }
        }
        iced::Task::none()
    }

    fn push_toast(&mut self, kind: ToastKind, title: &str, body: &str, now: Instant) {
        self.toasts.push(Toast::new(kind, title, body), now);
    }

    fn view(&self) -> iced::Element<'_, Message, HudTheme> {
        let nav = pages::nav(self);
        let body: iced::Element<'_, Message, HudTheme> = match self.page {
            pages::Page::Dashboard => pages::dashboard(self),
            pages::Page::Nodes => pages::nodes(self),
            pages::Page::Settings => pages::settings(self),
            pages::Page::DialogFlow => pages::dialog_flow(self),
        };
        let mut page_content = column![pages::page_title(self)];
        if !self.toasts.is_empty() {
            page_content = page_content.push(hud_widgets::toast::toast_list(
                &self.toasts,
                &self.theme.tokens,
                Some(&Message::ToastDismiss),
            ));
        }
        page_content = page_content
            .push(body)
            .spacing(24)
            .padding(24)
            .max_width(920);

        let page = iced::widget::scrollable(page_content)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill);

        let base: iced::Element<'_, Message, HudTheme> = iced::widget::row![nav, page]
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into();

        // 模态层（对话框打开时叠加遮罩与对话框）
        match self.modal {
            Some(kind) => {
                let dialog = pages::dialog_for(self, kind);
                hud_widgets::modal::modal(base, dialog, &self.theme.tokens, &|| Message::CloseModal)
            }
            None => base,
        }
    }
}
