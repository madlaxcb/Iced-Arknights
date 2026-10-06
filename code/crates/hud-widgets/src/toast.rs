//! Toast / Notification 通知队列：自动过期 + 手动关闭 + 右上角堆叠。
//!
//! 按"受控三件套"提供：[`ToastState`]（应用持有的状态）+ [`ToastState::retain_fresh`]
//! （过期清理，由应用的定时订阅驱动）+ [`toast_list`]（渲染）。

use hud_theme::HudTheme;
use hud_tokens::Tokens;
use iced::widget::{button, column, container, row, text};
use iced::{Element, Length, Padding};
use std::time::{Duration, Instant};

/// 通知类型（色条语义同 Badge）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToastKind {
    /// 信息（info）
    Info,
    /// 成功（success）
    Success,
    /// 警告（accent）
    Warning,
    /// 危险（danger）
    Danger,
}

/// 一条通知。
#[derive(Debug, Clone)]
pub struct Toast {
    /// 类型（决定色条颜色）
    pub kind: ToastKind,
    /// 标题
    pub title: String,
    /// 正文（可空字符串）
    pub body: String,
}

impl Toast {
    /// 快捷构造。
    pub fn new(kind: ToastKind, title: impl Into<String>, body: impl Into<String>) -> Self {
        Self {
            kind,
            title: title.into(),
            body: body.into(),
        }
    }
}

/// 通知队列状态（应用持有）。
#[derive(Debug, Default)]
pub struct ToastState {
    items: Vec<(Toast, Instant)>,
}

impl ToastState {
    /// 入队（记录入队时刻）。
    pub fn push(&mut self, toast: Toast, now: Instant) {
        self.items.push((toast, now));
    }

    /// 清理过期项（由应用周期订阅驱动；建议周期 ≤ ttl / 4）。
    pub fn retain_fresh(&mut self, now: Instant, ttl: Duration) {
        self.items
            .retain(|(_, born)| now.duration_since(*born) < ttl);
    }

    /// 手动关闭第 `index` 条。
    pub fn dismiss(&mut self, index: usize) {
        if index < self.items.len() {
            self.items.remove(index);
        }
    }

    /// 当前数量。
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// 是否为空（空时应用应取消定时订阅）。
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

fn kind_color(theme: &HudTheme, kind: ToastKind) -> iced::Color {
    let p = &theme.tokens.palette;
    match kind {
        ToastKind::Info => hud_theme::color(p.info),
        ToastKind::Success => hud_theme::color(p.success),
        ToastKind::Warning => hud_theme::color(p.accent),
        ToastKind::Danger => hud_theme::color(p.danger),
    }
}

/// 渲染通知堆叠（右上角，宽度 300）。
///
/// `on_dismiss` 提供时每条通知带关闭按钮（回调参数为队列下标）。
pub fn toast_list<'a, Message: Clone + 'a>(
    state: &'a ToastState,
    tokens: &Tokens,
    on_dismiss: Option<&'a dyn Fn(usize) -> Message>,
) -> Element<'a, Message, HudTheme> {
    let cards: Vec<Element<'a, Message, HudTheme>> = state
        .items
        .iter()
        .enumerate()
        .map(|(i, (toast, _))| {
            let strip = container(text(""))
                .width(3.0)
                .style(move |theme: &HudTheme| iced::widget::container::Style {
                    background: Some(kind_color(theme, toast.kind).into()),
                    ..Default::default()
                });

            let mut body = column![
                text(toast.title.clone()).size(14),
                text(toast.body.clone()).size(12).style(|theme: &HudTheme| {
                    iced::widget::text::Style {
                        color: Some(hud_theme::color(theme.tokens.palette.text_secondary)),
                    }
                }),
            ]
            .spacing(2);

            if let Some(on_dismiss) = on_dismiss {
                let close = button(text("×").size(12))
                    .padding(2)
                    .on_press(on_dismiss(i))
                    .class(Box::new(|theme: &HudTheme, status: button::Status| {
                        let p = &theme.tokens.palette;
                        let (bg, fg) = match status {
                            button::Status::Hovered => (p.bg2, p.danger),
                            _ => (p.bg1, p.text_secondary),
                        };
                        button::Style {
                            background: Some(hud_theme::color(bg).into()),
                            text_color: hud_theme::color(fg),
                            border: iced::Border {
                                color: hud_theme::color(p.bg1),
                                width: 0.0,
                                radius: 0.0.into(),
                            },
                            ..Default::default()
                        }
                    }) as button::StyleFn<'static, HudTheme>);
                body = body.push(row![text("").width(Length::Fill), close].spacing(0));
            }

            let card = row![
                strip,
                container(body).padding(Padding::from([8, 12]).left(10))
            ]
            .width(Length::Fill);

            container(card)
                .width(300)
                .style(move |theme: &HudTheme| iced::widget::container::Style {
                    background: Some(hud_theme::color(theme.tokens.palette.bg1).into()),
                    border: iced::Border {
                        color: hud_theme::color(theme.tokens.palette.line_base),
                        width: theme.tokens.geometry.line.base,
                        radius: 0.0.into(),
                    },
                    ..Default::default()
                })
                .into()
        })
        .collect();

    iced::widget::Column::<Message, HudTheme, iced::Renderer>::with_children(cards)
        .width(Length::Shrink)
        .spacing(tokens.geometry.space.s)
        .into()
}
