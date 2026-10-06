//! 文字与分隔：SectionHeader（中文主标题 + 英文小字副标题）与 Divider。

use hud_theme::HudTheme;
use iced::widget::{column, rule, text};

/// 次级文字样式类（装箱）。
pub fn secondary_text() -> text::StyleFn<'static, HudTheme> {
    Box::new(|theme: &HudTheme| text::Style {
        color: Some(hud_theme::color(theme.tokens.palette.text_secondary)),
    })
}

/// 强调文字样式类（accent 色，用于关键数值 / 当前项）。
pub fn accent_text() -> text::StyleFn<'static, HudTheme> {
    Box::new(|theme: &HudTheme| text::Style {
        color: Some(hud_theme::color(theme.tokens.palette.accent)),
    })
}

/// 区块标题：中文主标题 + 英文大写小字副标题（计划书 1.2）。
pub fn section_header<'a, Message: 'a>(
    zh: &'a str,
    en: &'a str,
) -> iced::Element<'a, Message, HudTheme> {
    column![text(zh).size(20), text(en).size(12).class(secondary_text()),]
        .spacing(2)
        .into()
}

/// 分隔线（1px，Token 颜色经主题 Catalog）。
pub fn divider<Message: 'static>() -> iced::Element<'static, Message, HudTheme> {
    rule::horizontal(1.0).into()
}

/// 卡片：标题区 + 内容区 + 切角容器（计划书 1.2 Card P0）。
pub fn card<'a, Message>(
    tokens: &hud_tokens::Tokens,
    title: &'a str,
    title_en: &'a str,
    body: iced::Element<'a, Message, HudTheme>,
) -> iced::Element<'a, Message, HudTheme>
where
    Message: 'a + 'static,
{
    let header = section_header::<Message>(title, title_en);
    let content = column![header, divider(), body].spacing(12);
    crate::panel::panel(content, tokens, crate::panel::ChamferLevel::M)
        .padding(16)
        .into()
}
