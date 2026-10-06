//! 设置页：完整表单控件（输入 / 下拉 / 开关 / 复选 / 滑杆）+ 应用反馈。

use hud_theme::HudTheme;
use hud_widgets::{column, row};

use crate::{Message, Sample};

/// 渲染设置页。
pub fn settings(s: &Sample) -> iced::Element<'_, Message, HudTheme> {
    let tokens = s.theme.tokens;
    let form = &s.form;

    let region_option: Option<&'static str> = Sample::REGIONS.get(form.region).copied();

    column![
        hud_widgets::typography::section_header("身份与区域", "IDENTITY"),
        labeled(
            "操作台代号",
            iced::widget::text_input("WATCH-XX", &form.callsign)
                .on_input(Message::CallsignInput)
                .width(280.0),
            &tokens,
        ),
        labeled(
            "默认区域",
            hud_widgets::input::select(&Sample::REGIONS, region_option, Message::RegionSelected,),
            &tokens,
        ),
        hud_widgets::typography::section_header("监控行为", "BEHAVIOR"),
        labeled(
            "断线自动重连",
            iced::widget::toggler(form.auto_reconnect).on_toggle(Message::AutoReconnect),
            &tokens,
        ),
        labeled(
            "告警通知",
            iced::widget::checkbox(form.notify).on_toggle(Message::Notify),
            &tokens,
        ),
        labeled(
            "数据刷新间隔",
            row![
                iced::widget::slider(1.0..=30.0, form.refresh_secs, Message::RefreshChanged)
                    .width(220.0),
                iced::widget::text(format!("{:.0}s", form.refresh_secs))
                    .size(13)
                    .class(hud_widgets::typography::secondary_text()),
            ]
            .spacing(tokens.geometry.space.m)
            .align_y(iced::alignment::Vertical::Center),
            &tokens,
        ),
        iced::widget::button(iced::widget::text("应用设置 / APPLY").size(14))
            .padding(10)
            .on_press(Message::ApplySettings)
            .class(hud_widgets::button::class(
                hud_widgets::button::ButtonVariant::Primary,
            )),
    ]
    .spacing(tokens.geometry.space.m)
    .into()
}

/// 标签 + 控件一行。
fn labeled<'a>(
    label: &str,
    control: impl Into<iced::Element<'a, Message, HudTheme>>,
    tokens: &hud_tokens::Tokens,
) -> iced::Element<'a, Message, HudTheme> {
    row![
        iced::widget::container(iced::widget::text(label.to_string()).size(14)).width(160.0),
        control.into(),
    ]
    .spacing(tokens.geometry.space.m)
    .align_y(iced::alignment::Vertical::Center)
    .into()
}
