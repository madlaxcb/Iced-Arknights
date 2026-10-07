//! 受控 ContextMenu：应用持有打开状态、菜单位置与选中项。

use hud_theme::HudTheme;
use iced::widget::{button, column, container, rule, text};
use iced::{keyboard, Element, Event, Length, Padding, Subscription};

/// ContextMenu 的一项。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContextMenuItem {
    /// 可执行菜单项。
    Action {
        /// 稳定标识。
        id: usize,
        /// 显示文本。
        label: String,
    },
    /// 禁用菜单项。
    Disabled {
        /// 稳定标识。
        id: usize,
        /// 显示文本。
        label: String,
    },
    /// 分隔线。
    Separator {
        /// 稳定标识。
        id: usize,
    },
}

impl ContextMenuItem {
    /// 创建可执行菜单项。
    pub fn action(id: usize, label: impl Into<String>) -> Self {
        Self::Action {
            id,
            label: label.into(),
        }
    }

    /// 创建禁用菜单项。
    pub fn disabled(id: usize, label: impl Into<String>) -> Self {
        Self::Disabled {
            id,
            label: label.into(),
        }
    }

    /// 创建分隔线。
    pub fn separator(id: usize) -> Self {
        Self::Separator { id }
    }

    fn id(&self) -> usize {
        match self {
            Self::Action { id, .. } | Self::Disabled { id, .. } | Self::Separator { id } => *id,
        }
    }

    fn enabled(&self) -> bool {
        matches!(self, Self::Action { .. })
    }
}

/// ContextMenu 的键盘选中状态。
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ContextMenuState {
    selected: Option<usize>,
}

impl ContextMenuState {
    /// 创建未选中的菜单状态。
    pub fn new() -> Self {
        Self::default()
    }

    /// 返回当前已选中的可执行项 ID。
    pub fn selected(&self, items: &[ContextMenuItem]) -> Option<usize> {
        self.selected
            .filter(|id| items.iter().any(|item| item.id() == *id && item.enabled()))
    }

    /// 向下移动到下一个可执行项。
    pub fn move_next(&mut self, items: &[ContextMenuItem]) {
        self.move_by(items, 1);
    }

    /// 向上移动到上一个可执行项。
    pub fn move_previous(&mut self, items: &[ContextMenuItem]) {
        self.move_by(items, -1);
    }

    /// 激活当前可执行项。
    pub fn activate(&self, items: &[ContextMenuItem]) -> Option<usize> {
        self.selected(items)
    }

    fn move_by(&mut self, items: &[ContextMenuItem], direction: isize) {
        let enabled: Vec<usize> = items
            .iter()
            .filter(|item| item.enabled())
            .map(ContextMenuItem::id)
            .collect();
        if enabled.is_empty() {
            self.selected = None;
            return;
        }
        let current = self
            .selected(items)
            .and_then(|id| enabled.iter().position(|item| *item == id));
        let next = match current {
            Some(index) => (index as isize + direction).rem_euclid(enabled.len() as isize) as usize,
            None => 0,
        };
        self.selected = Some(enabled[next]);
    }
}

/// 渲染受控 ContextMenu。
///
/// 菜单本身不持有打开状态或屏幕坐标；调用方应仅在菜单打开时渲染它，
/// 并通过外层布局将它放置在触发点附近。
pub fn context_menu<'a, Message: Clone + 'a>(
    items: Vec<ContextMenuItem>,
    state: ContextMenuState,
    on_select: &'a dyn Fn(usize) -> Message,
) -> Element<'a, Message, HudTheme> {
    let rows = items.as_slice().iter().map(|item| match item {
        ContextMenuItem::Separator { .. } => rule::horizontal(1.0).into(),
        ContextMenuItem::Action { id, label } | ContextMenuItem::Disabled { id, label } => {
            let id = *id;
            let selected = state.selected(items.as_slice()) == Some(id);
            let enabled = item.enabled();
            let row = button(text(label.clone()).size(14))
                .width(Length::Fill)
                .padding(Padding::from([8, 12]))
                .on_press_maybe(enabled.then(|| on_select(id)))
                .class(Box::new(move |theme: &HudTheme, status: button::Status| {
                    let p = &theme.tokens.palette;
                    let (bg, fg) = if selected {
                        (p.bg2, p.accent)
                    } else if !enabled {
                        (p.bg1, p.text_disabled)
                    } else if matches!(status, button::Status::Hovered) {
                        (p.bg2, p.text_primary)
                    } else {
                        (p.bg1, p.text_primary)
                    };
                    button::Style {
                        background: Some(hud_theme::color(bg).into()),
                        text_color: hud_theme::color(fg),
                        ..Default::default()
                    }
                }) as button::StyleFn<'_, HudTheme>);
            row.into()
        }
    });

    let menu = container(column(rows).spacing(1))
        .width(220.0)
        .padding(4)
        .style(|theme: &HudTheme| container::Style {
            background: Some(hud_theme::color(theme.tokens.palette.bg1).into()),
            border: iced::Border {
                color: hud_theme::color(theme.tokens.palette.line_base),
                width: theme.tokens.geometry.line.base,
                radius: 0.0.into(),
            },
            ..Default::default()
        });

    menu.into()
}

/// ContextMenu 键盘导航消息。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContextMenuKey {
    /// 向上移动。
    Previous,
    /// 向下移动。
    Next,
    /// 激活当前项。
    Activate,
    /// 关闭菜单。
    Dismiss,
}

/// 监听 ContextMenu 的方向键、Enter 和 Esc。
pub fn keyboard_listener() -> Subscription<ContextMenuKey> {
    #[derive(Hash)]
    struct ContextMenuKeyboard;

    iced_futures::subscription::filter_map(ContextMenuKeyboard, |event| {
        let iced_futures::subscription::Event::Interaction { event, status, .. } = event else {
            return None;
        };
        if status != iced::event::Status::Ignored {
            return None;
        }
        match event {
            Event::Keyboard(keyboard::Event::KeyPressed {
                key: keyboard::Key::Named(keyboard::key::Named::ArrowUp),
                ..
            }) => Some(ContextMenuKey::Previous),
            Event::Keyboard(keyboard::Event::KeyPressed {
                key: keyboard::Key::Named(keyboard::key::Named::ArrowDown),
                ..
            }) => Some(ContextMenuKey::Next),
            Event::Keyboard(keyboard::Event::KeyPressed {
                key: keyboard::Key::Named(keyboard::key::Named::Enter),
                ..
            }) => Some(ContextMenuKey::Activate),
            Event::Keyboard(keyboard::Event::KeyPressed {
                key: keyboard::Key::Named(keyboard::key::Named::Escape),
                ..
            }) => Some(ContextMenuKey::Dismiss),
            _ => None,
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keyboard_navigation_skips_separators_and_disabled_items() {
        let items = vec![
            ContextMenuItem::action(1, "Open"),
            ContextMenuItem::separator(2),
            ContextMenuItem::disabled(3, "Unavailable"),
            ContextMenuItem::action(4, "Rename"),
        ];
        let mut state = ContextMenuState::new();

        assert_eq!(state.selected(&items), None);
        state.move_next(&items);
        assert_eq!(state.selected(&items), Some(1));
        state.move_next(&items);
        assert_eq!(state.selected(&items), Some(4));
        state.move_previous(&items);
        assert_eq!(state.selected(&items), Some(1));
    }

    #[test]
    fn activation_returns_only_enabled_action() {
        let items = vec![
            ContextMenuItem::disabled(1, "Unavailable"),
            ContextMenuItem::action(2, "Open"),
        ];
        let mut state = ContextMenuState::new();

        assert_eq!(state.activate(&items), None);
        state.move_next(&items);
        assert_eq!(state.selected(&items), Some(2));
        assert_eq!(state.activate(&items), Some(2));
    }
}
