//! 对话流程页：确认（模态）、错误（故障注入 + Toast）、加载（扫描条 + Toast）。

use hud_theme::HudTheme;
use hud_widgets::column;

use crate::{Message, Sample};

/// 渲染操作流程页。
pub fn dialog_flow(s: &Sample) -> iced::Element<'_, Message, HudTheme> {
    use hud_widgets::button::{class, ButtonVariant};
    let tokens = s.theme.tokens;

    // 确认流程：断开主链路（模态确认）
    let confirm_section = column![
        hud_widgets::typography::section_header("确认流程", "CONFIRM"),
        iced::widget::text("危险操作前弹出模态对话框；Esc 或点击遮罩取消。").size(13),
        iced::widget::button(iced::widget::text("断开主链路…").size(14))
            .padding(8)
            .on_press(Message::AskDisconnect)
            .class(class(ButtonVariant::Danger)),
    ]
    .spacing(tokens.geometry.space.s);

    // 错误流程：注入故障 → Danger Toast + 主链路离线
    let error_section = column![
        hud_widgets::typography::section_header("错误流程", "ERROR"),
        iced::widget::text("注入链路故障，观察主链路离线状态与通知。").size(13),
        iced::widget::button(iced::widget::text("模拟故障 / FAULT").size(14))
            .padding(8)
            .on_press(Message::DemoError)
            .class(class(ButtonVariant::Secondary)),
    ]
    .spacing(tokens.geometry.space.s);

    // 加载流程：2 秒同步 → Success Toast
    let mut load_section = column![
        hud_widgets::typography::section_header("加载流程", "LOADING"),
        iced::widget::text("重新拉取全部节点数据，期间显示扫描动画。").size(13),
        iced::widget::button(iced::widget::text("重新同步 / SYNC").size(14))
            .padding(8)
            .on_press(Message::DemoLoad)
            .class(class(ButtonVariant::Primary)),
    ]
    .spacing(tokens.geometry.space.s);

    if s.loading.is_some() {
        load_section = load_section.push(
            column![
                iced::widget::canvas(hud_widgets::loading::ScanBar::new(s.loading_phase))
                    .width(280.0)
                    .height(hud_widgets::progress::recommended_height(&tokens)),
                iced::widget::text("正在同步节点数据…")
                    .size(12)
                    .class(hud_widgets::typography::secondary_text()),
            ]
            .spacing(tokens.geometry.space.xs),
        );
    }

    // 链路恢复（断开后可从这里重连）
    let mut flow = column![
        confirm_section,
        error_section,
        load_section,
        hud_widgets::typography::section_header("链路控制", "LINK CONTROL"),
    ]
    .spacing(tokens.geometry.space.l);

    flow = if s.link_online {
        flow.push(
            iced::widget::text("主链路运行中。")
                .size(13)
                .class(hud_widgets::typography::secondary_text()),
        )
    } else {
        flow.push(
            iced::widget::text("主链路离线。")
                .size(13)
                .class(hud_widgets::typography::secondary_text()),
        )
    };

    // 恢复操作也用于清除故障注入状态，因此链路在线时仍提供此操作。
    flow = flow.push(
        iced::widget::button(iced::widget::text("重建主链路 / RESTORE").size(14))
            .padding(8)
            .on_press(Message::RestoreLink)
            .class(class(ButtonVariant::Primary)),
    );

    flow.into()
}
