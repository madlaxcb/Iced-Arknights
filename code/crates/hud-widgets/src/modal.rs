//! Modal / Dialog 模态层：遮罩 + 居中对话框 + Esc 关闭。
//!
//! 结构上"底层内容 → 遮罩 → 对话框"三层叠放（`stack`），遮罩点击与 Esc
//! 均映射到调用方给定的 dismiss 消息；对话框在上层自然拦截事件。
//! 焦点顺序（Tab 循环）由应用用 `operation::focus_next` 处理（见 Gallery 演示）。

use hud_theme::HudTheme;
use hud_tokens::Tokens;
use iced::widget::{container, mouse_area, text};
use iced::{keyboard, Element, Event, Length, Subscription};

/// 渲染模态层。
///
/// `base` 为被模态覆盖的页面内容；`dialog` 为对话框内容（建议用
/// [`crate::panel::panel`] 或 [`crate::typography::card`] 包裹）；
/// `on_dismiss` 是遮罩点击时发出的消息（Esc 由 [`escape_listener`] 提供）。
pub fn modal<'a, Message: Clone + 'a>(
    base: Element<'a, Message, HudTheme>,
    dialog: Element<'a, Message, HudTheme>,
    tokens: &Tokens,
    on_dismiss: &'a dyn Fn() -> Message,
) -> Element<'a, Message, HudTheme> {
    let overlay_bg = hud_theme::color(tokens.palette.bg0).scale_alpha(0.6);

    let scrim = container(text(""))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_theme: &HudTheme| iced::widget::container::Style {
            background: Some(overlay_bg.into()),
            ..Default::default()
        });

    let scrim = mouse_area(scrim.center(Length::Fill)).on_press(on_dismiss());

    let dialog = container(dialog)
        .width(Length::Fill)
        .height(Length::Fill)
        .center(Length::Fill)
        .padding(tokens.geometry.space.xl);

    iced::widget::Stack::<Message, HudTheme, iced::Renderer>::with_children(vec![
        base,
        scrim.into(),
        dialog.into(),
    ])
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

/// Esc 键监听：按下 Esc（且未被控件捕获）时发 `on_escape`。
///
/// 仅在存在模态时订阅（计划书 2.5：无动画 / 无浮层时不订阅）。
/// 注意：`Subscription::map` 的闭包禁止捕获（编译期零尺寸检查），
/// 因此这里用自由函数 `subscription::filter_map`，以类型为身份、
/// 消息通过闭包捕获传入（与 `event::listen_with` 同一模式）。
pub fn escape_listener<Message: Clone + Send + 'static>(
    on_escape: Message,
) -> Subscription<Message> {
    #[derive(Hash)]
    struct EscapeListener;

    iced_futures::subscription::filter_map(EscapeListener, move |subscription_event| {
        let iced_futures::subscription::Event::Interaction { event, status, .. } =
            subscription_event
        else {
            return None;
        };
        match event {
            Event::Keyboard(keyboard::Event::KeyPressed {
                key: keyboard::Key::Named(keyboard::key::Named::Escape),
                ..
            }) if status == iced::event::Status::Ignored => Some(on_escape.clone()),
            _ => None,
        }
    })
}
