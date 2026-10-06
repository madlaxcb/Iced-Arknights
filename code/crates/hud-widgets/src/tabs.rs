//! Tabs 页签：横向选项卡，活动项带 2px（line.strong 档位）强调下划线。

use hud_theme::HudTheme;
use hud_tokens::Tokens;
use iced::widget::{button, column, container, row, text};
use iced::{Element, Length};

/// 渲染页签栏。
///
/// `labels` 为页签文本；`active` 为活动索引；`on_change` 返回切换消息。
/// 下划线粗细取 `tokens.geometry.line.strong`（组件内禁止字面尺寸）。
pub fn tabs<'a, Message: Clone + 'a>(
    labels: &[&str],
    active: usize,
    tokens: &Tokens,
    on_change: &'a dyn Fn(usize) -> Message,
) -> Element<'a, Message, HudTheme> {
    let strong = tokens.geometry.line.strong;

    let tabs: Vec<Element<'a, Message, HudTheme>> = labels
        .iter()
        .enumerate()
        .map(|(i, label)| {
            let is_active = i == active;
            let label = (*label).to_string();
            let btn = button(text(label).size(14))
                .padding([8, 16])
                .on_press(on_change(i))
                .class(Box::new(move |theme: &HudTheme, status: button::Status| {
                    let p = &theme.tokens.palette;
                    let (bg, fg) = if is_active {
                        (p.bg0, p.accent)
                    } else {
                        match status {
                            button::Status::Hovered => (p.bg1, p.text_primary),
                            _ => (p.bg0, p.text_secondary),
                        }
                    };
                    button::Style {
                        background: Some(hud_theme::color(bg).into()),
                        text_color: hud_theme::color(fg),
                        border: iced::Border {
                            color: hud_theme::color(p.bg0),
                            width: 0.0,
                            radius: 0.0.into(),
                        },
                        ..Default::default()
                    }
                }) as button::StyleFn<'_, HudTheme>);

            // 活动下划线（强调线档位；非活动为 0 高不渲染）
            let underline = container(text(""))
                .width(Length::Fill)
                .height(if is_active { strong } else { 0.0 })
                .style(move |theme: &HudTheme| iced::widget::container::Style {
                    background: Some(
                        hud_theme::color(if is_active {
                            theme.tokens.palette.accent
                        } else {
                            theme.tokens.palette.bg0
                        })
                        .into(),
                    ),
                    ..Default::default()
                });

            column![btn, underline].spacing(0).into()
        })
        .collect();

    let bar = row(tabs);
    let divider = container(text(""))
        .width(Length::Fill)
        .height(tokens.geometry.line.base)
        .style(|theme: &HudTheme| iced::widget::container::Style {
            background: Some(hud_theme::color(theme.tokens.palette.line_base).into()),
            ..Default::default()
        });

    column![bar, divider].spacing(0).into()
}
