//! SideNav 侧边导航（组件化：Gallery 与业务应用共用）。

use hud_theme::HudTheme;
use hud_tokens::Tokens;
use iced::widget::{button, text};
use iced::{Element, Length};

/// 导航项数据。
pub struct NavItem {
    /// 稳定标识（回调返回值）
    pub id: usize,
    /// 文本
    pub label: String,
}

/// 渲染侧边导航（活动项：bg2 底 + 强调线边框 + accent 文字）。
pub fn side_nav<'a, Message: Clone + 'a>(
    items: &[NavItem],
    active: usize,
    tokens: &Tokens,
    width: f32,
    on_select: &'a dyn Fn(usize) -> Message,
) -> Element<'a, Message, HudTheme> {
    let children: Vec<Element<'a, Message, HudTheme>> = items
        .iter()
        .map(|item| {
            let id = item.id;
            let is_active = id == active;
            button(text(item.label.clone()).size(14))
                .width(Length::Fill)
                .padding(10)
                .on_press(on_select(id))
                .class(Box::new(move |theme: &HudTheme, status: button::Status| {
                    let p = &theme.tokens.palette;
                    let (bg, line, fg) = if is_active {
                        (p.bg2, p.line_strong, p.accent)
                    } else {
                        match status {
                            button::Status::Hovered => (p.bg1, p.line_strong, p.text_primary),
                            _ => (p.bg0, p.bg0, p.text_secondary),
                        }
                    };
                    button::Style {
                        background: Some(hud_theme::color(bg).into()),
                        text_color: hud_theme::color(fg),
                        border: iced::Border {
                            color: hud_theme::color(line),
                            width: theme.tokens.geometry.line.base,
                            radius: 0.0.into(),
                        },
                        ..Default::default()
                    }
                }) as button::StyleFn<'_, HudTheme>)
                .into()
        })
        .collect();

    iced::widget::Column::<Message, HudTheme, iced::Renderer>::with_children(children)
        .width(width)
        .height(Length::Fill)
        .padding(tokens.geometry.space.m)
        .spacing(tokens.geometry.space.xs)
        .into()
}
