//! TitleBar 自绘标题栏：拖动、双击最大化/还原、最小化 / 最大化 / 关闭。
//!
//! 命令经由 [`TitleBarCommand`] 抛给应用，由应用映射为窗口 Task
//! （`hud_platform::perform`）。四边缩放热区与 DWM 行为属 Windows 真机项
//! （见 doc/ADR-003 清单）。

use hud_theme::HudTheme;
use hud_tokens::Tokens;
use iced::widget::{button, container, mouse_area, row, text};
use iced::{Element, Length};

/// 标题栏命令。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TitleBarCommand {
    /// 按住标题区拖动窗口
    Drag,
    /// 双击标题区：最大化 / 还原
    ToggleMaximize,
    /// 最小化
    Minimize,
    /// 关闭窗口
    Close,
}

/// 渲染标题栏（高度 = 内容 + 上下 padding，均在 Token 口径内）。
///
/// `to_message` 把 [`TitleBarCommand`] 映射为应用消息；
/// 窗口控制按钮的悬停遵循全库规则（关闭键悬停为 danger 底）。
pub fn title_bar<'a, Message: Clone + 'a>(
    title: &str,
    tokens: &Tokens,
    to_message: &'a dyn Fn(TitleBarCommand) -> Message,
) -> Element<'a, Message, HudTheme> {
    let p = &tokens.palette;
    let control_size = 24.0 + tokens.geometry.space.s;

    let drag_area = mouse_area(
        container(text(title.to_string()).size(12))
            .width(Length::Fill)
            .height(control_size)
            .align_y(iced::alignment::Vertical::Center),
    )
    .on_press(to_message(TitleBarCommand::Drag))
    .on_double_click(to_message(TitleBarCommand::ToggleMaximize));

    let control = |glyph: &'static str, command: TitleBarCommand, danger: bool| {
        button(
            container(text(glyph).size(12))
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(iced::alignment::Horizontal::Center)
                .align_y(iced::alignment::Vertical::Center),
        )
        .width(control_size)
        .height(control_size)
        .on_press(to_message(command))
        .class(Box::new(move |theme: &HudTheme, status: button::Status| {
            let pal = &theme.tokens.palette;
            let hovered = matches!(status, button::Status::Hovered);
            let (bg, fg, border) = if danger && hovered {
                (pal.danger, pal.bg0, pal.danger)
            } else if hovered {
                (pal.bg2, pal.text_primary, pal.line_strong)
            } else {
                (pal.bg1, pal.text_secondary, pal.bg1)
            };
            button::Style {
                background: Some(hud_theme::color(bg).into()),
                text_color: hud_theme::color(fg),
                border: iced::Border {
                    color: hud_theme::color(border),
                    width: theme.tokens.geometry.line.base,
                    radius: 0.0.into(),
                },
                ..Default::default()
            }
        }) as button::StyleFn<'static, HudTheme>)
    };

    let bar = row![
        drag_area,
        control("–", TitleBarCommand::Minimize, false),
        control("▢", TitleBarCommand::ToggleMaximize, false),
        control("×", TitleBarCommand::Close, true),
    ]
    .spacing(tokens.geometry.space.xs);

    // 捕获 Token 值（Copy），避免闭包借用 tokens 生命周期
    let bar_bg = hud_theme::color(p.bg1);
    let bar_line = hud_theme::color(p.line_base);
    let bar_line_w = tokens.geometry.line.base;

    container(bar)
        .width(Length::Fill)
        .padding([tokens.geometry.space.xs, tokens.geometry.space.s])
        .style(move |_theme: &HudTheme| iced::widget::container::Style {
            background: Some(bar_bg.into()),
            border: iced::Border {
                color: bar_line,
                width: bar_line_w,
                radius: 0.0.into(),
            },
            ..Default::default()
        })
        .into()
}
