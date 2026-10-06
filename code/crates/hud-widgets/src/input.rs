//! 输入与浮层包装：Select（pick_list）与 Tooltip（均经 HudTheme 主题化）。

use hud_theme::HudTheme;
use iced::widget::pick_list;
use iced::Element;

/// Select 下拉选择（pick_list 包装，统一内边距与宽度）。
pub fn select<'a, T, Message>(
    options: &'a [T],
    selected: Option<T>,
    on_selected: impl Fn(T) -> Message + 'a,
) -> Element<'a, Message, HudTheme>
where
    T: ToString + PartialEq + Clone + 'a,
    Message: Clone + 'a,
{
    pick_list(options, selected, on_selected)
        .padding(6)
        .width(180.0)
        .into()
}

/// Tooltip 悬浮提示（内容 + 提示文本，底部弹出）。
pub fn tooltip<'a, Message: Clone + 'a>(
    content: Element<'a, Message, HudTheme>,
    tip: &'a str,
) -> Element<'a, Message, HudTheme> {
    iced::widget::tooltip::Tooltip::new(content, tip, iced::widget::tooltip::Position::Bottom)
        .into()
}
