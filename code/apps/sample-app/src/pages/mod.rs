//! 页面模块：导航、页头与四个业务页面。

mod dashboard;
mod dialog_flow;
mod nodes;
mod settings;

pub use dashboard::dashboard;
pub use dialog_flow::dialog_flow;
pub use nodes::nodes;
pub use settings::settings;

use hud_theme::HudTheme;
use hud_widgets::{column, prelude::*};

use crate::{Message, ModalKind, Sample};

/// 页面清单。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    /// 首页仪表盘
    Dashboard,
    /// 节点列表 + 详情
    Nodes,
    /// 设置（表单）
    Settings,
    /// 对话流程（确认 / 错误 / 加载）
    DialogFlow,
}

impl Page {
    const ALL: [Self; 4] = [
        Self::Dashboard,
        Self::Nodes,
        Self::Settings,
        Self::DialogFlow,
    ];

    /// 中文标题。
    fn title(self) -> &'static str {
        match self {
            Self::Dashboard => "监控台 / DASHBOARD",
            Self::Nodes => "节点管理 / NODES",
            Self::Settings => "设置 / SETTINGS",
            Self::DialogFlow => "操作流程 / OPERATIONS",
        }
    }

    /// 英文副标题。
    fn subtitle(self) -> &'static str {
        match self {
            Self::Dashboard => "OVERVIEW",
            Self::Nodes => "LIST & DETAIL",
            Self::Settings => "PREFERENCES",
            Self::DialogFlow => "CONFIRM / ERROR / LOADING",
        }
    }
}

/// 侧边导航。
pub fn nav(s: &Sample) -> iced::Element<'_, Message, HudTheme> {
    let items: Vec<hud_widgets::nav::NavItem> = Page::ALL
        .iter()
        .enumerate()
        .map(|(i, p)| hud_widgets::nav::NavItem {
            id: i,
            label: p.title().split(" /").next().unwrap_or("").to_string(),
        })
        .collect();
    let active = Page::ALL.iter().position(|p| *p == s.page).unwrap_or(0);

    hud_widgets::nav::side_nav(&items, active, &s.theme.tokens, 200.0, &|i| {
        Message::Nav(Page::ALL[i])
    })
}

/// 页头（标题 + 英文副标题 + 强调线）。
pub fn page_title(s: &Sample) -> iced::Element<'_, Message, HudTheme> {
    let tokens = s.theme.tokens;
    column![
        text(s.page.title()).size(24),
        text(s.page.subtitle())
            .size(12)
            .class(hud_widgets::typography::secondary_text()),
        iced::widget::rule::horizontal(1)
    ]
    .spacing(tokens.geometry.space.xs)
    .into()
}

/// 按模态种类构造对话框内容。
pub fn dialog_for(s: &Sample, kind: ModalKind) -> iced::Element<'_, Message, HudTheme> {
    use hud_widgets::button::{class, ButtonVariant};
    use hud_widgets::typography::{card, secondary_text};

    let tokens = s.theme.tokens;
    let actions = column![
        iced::widget::row![
            iced::widget::button(iced::widget::text("取消 / CANCEL").size(14))
                .padding(8)
                .on_press(Message::CloseModal)
                .class(class(ButtonVariant::Secondary)),
            iced::widget::button(iced::widget::text("确认 / CONFIRM").size(14))
                .padding(8)
                .on_press(Message::ConfirmModal)
                .class(class(ButtonVariant::Primary)),
        ]
        .spacing(tokens.geometry.space.m),
        iced::widget::text("Esc 或点击遮罩关闭")
            .size(12)
            .class(secondary_text()),
    ]
    .spacing(tokens.geometry.space.m);

    let (title, title_en, detail) = match kind {
        ModalKind::DisconnectLink => (
            "断开主链路",
            "DISCONNECT",
            String::from("断开后所有节点将走备份链路，延迟显著上升。确定继续？"),
        ),
        ModalKind::RestartNode(i) => {
            let name = s
                .nodes
                .get(i)
                .map(|n| n.name.clone())
                .unwrap_or_else(|| "?".into());
            (
                "重启节点",
                "RESTART NODE",
                format!("即将重启 {name}，期间该节点不可用约 2 秒。"),
            )
        }
    };

    let body: iced::Element<'_, Message, HudTheme> = iced::widget::text(detail).size(14).into();

    card(
        &tokens,
        title,
        title_en,
        column![body, actions]
            .spacing(tokens.geometry.space.m)
            .into(),
    )
}
