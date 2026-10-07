//! HUD 风格的 Avatar / Image 框组件。

use hud_theme::HudTheme;
use iced::widget::{container, image, stack, text};
use iced::{Alignment, Element, Padding};

/// Avatar 的在线状态。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AvatarStatus {
    /// 在线。
    Online,
    /// 离线。
    Offline,
    /// 忙碌。
    Busy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StatusColor {
    Online,
    Offline,
    Busy,
}

/// 渲染 Avatar。`image` 为 `None` 时显示名称首字母作为占位符。
pub fn avatar<'a, Message: 'a>(
    name: &str,
    size: f32,
    image: Option<image::Handle>,
    status: Option<AvatarStatus>,
) -> Element<'a, Message, HudTheme> {
    let image_or_initials: Element<'a, Message, HudTheme> = match image {
        Some(handle) => image::Image::new(handle)
            .width(size)
            .height(size)
            .content_fit(iced::ContentFit::Cover)
            .into(),
        None => container(text(initials(name)).size(size * 0.34))
            .width(size)
            .height(size)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .style(|theme: &HudTheme| container::Style {
                background: Some(hud_theme::color(theme.tokens.palette.info).into()),
                text_color: Some(hud_theme::color(theme.tokens.palette.text_primary)),
                ..Default::default()
            })
            .into(),
    };

    let avatar = container(image_or_initials)
        .width(size)
        .height(size)
        .padding(Padding::ZERO)
        .style(move |theme: &HudTheme| container::Style {
            background: Some(hud_theme::color(theme.tokens.palette.bg2).into()),
            border: iced::Border {
                color: hud_theme::color(theme.tokens.palette.line_strong),
                width: theme.tokens.geometry.line.base,
                radius: (size / 2.0).into(),
            },
            ..Default::default()
        });

    let content: Element<'a, Message, HudTheme> = match status {
        Some(status) => stack![avatar, status_dot(status, size)].into(),
        None => avatar.into(),
    };

    container(content).width(size).height(size).into()
}

fn status_dot<'a, Message: 'a>(status: AvatarStatus, size: f32) -> Element<'a, Message, HudTheme> {
    let dot_size = (size * 0.25).max(6.0);
    container(text(""))
        .width(dot_size)
        .height(dot_size)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .style(move |theme: &HudTheme| container::Style {
            background: Some(
                match status_color(status) {
                    StatusColor::Online => hud_theme::color(theme.tokens.palette.success),
                    StatusColor::Offline => hud_theme::color(theme.tokens.palette.line_strong),
                    StatusColor::Busy => hud_theme::color(theme.tokens.palette.accent),
                }
                .into(),
            ),
            border: iced::Border {
                color: hud_theme::color(theme.tokens.palette.bg1),
                width: theme.tokens.geometry.line.base,
                radius: (dot_size / 2.0).into(),
            },
            ..Default::default()
        })
        .into()
}

fn initials(name: &str) -> String {
    let initials: String = name
        .split_whitespace()
        .take(2)
        .filter_map(|word| word.chars().next())
        .flat_map(|character| character.to_uppercase())
        .collect();

    if initials.is_empty() {
        "?".to_string()
    } else {
        initials
    }
}

fn status_color(status: AvatarStatus) -> StatusColor {
    match status {
        AvatarStatus::Online => StatusColor::Online,
        AvatarStatus::Offline => StatusColor::Offline,
        AvatarStatus::Busy => StatusColor::Busy,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initials_use_first_two_non_empty_words() {
        assert_eq!(initials("Lumen"), "L");
        assert_eq!(initials("Lumen Alpha"), "LA");
        assert_eq!(initials("  Lumen   Alpha  "), "LA");
    }

    #[test]
    fn initials_fallback_to_question_mark() {
        assert_eq!(initials(""), "?");
        assert_eq!(initials("   "), "?");
    }

    #[test]
    fn status_color_is_selected_by_state() {
        assert_eq!(status_color(AvatarStatus::Online), StatusColor::Online);
        assert_eq!(status_color(AvatarStatus::Offline), StatusColor::Offline);
        assert_eq!(status_color(AvatarStatus::Busy), StatusColor::Busy);
    }
}
