//! HUD 风格组件库——组件集合与 prelude。
//!
//! 对外暴露构造函数式 API（计划书 2.3）：`hud::button("确认").variant(...)`。
//! 由于 iced 0.14 的布局宏会把子元素主题钉死为 `iced::Theme`（见 doc/ADR-001），
//! 本 crate 提供钉定 `HudTheme` 的等价宏：`hud::column!` / `hud::row!` / `hud::stack!`。

pub use hud_theme::HudTheme;

pub mod badge;
pub mod breadcrumb;
pub mod button;
pub mod context_menu;
pub mod data_readout;
pub mod decor;
pub mod input;
pub mod list;
pub mod loading;
pub mod modal;
pub mod nav;
pub mod panel;
pub mod progress;
pub mod split_pane;
pub mod status_panel;
pub mod table;
pub mod tabs;
pub mod title_bar;
pub mod toast;
pub mod typography;

/// 库内使用的 iced 重导出（宏路径解析需要）。
pub use iced;

/// 组件使用的元素别名。
pub type Element<'a, Message> = iced::Element<'a, Message, HudTheme>;

/// 常用类型统一导出。
pub mod prelude {
    pub use super::column;
    pub use super::row;
    pub use super::stack;
    pub use super::Element;
    pub use hud_core as core_util;
    pub use hud_theme::HudTheme;
    pub use iced::widget::{
        button, canvas, checkbox, container, pick_list, progress_bar, radio, rule, scrollable,
        slider, text, text_editor, text_input, toggler,
    };
    pub use iced::{Alignment, Center, Color, Length, Task};
}

/// 垂直布局（子元素主题钉定为 [`HudTheme`]）。
#[macro_export]
macro_rules! column {
    () => { $crate::iced::widget::Column::<_, $crate::HudTheme, _>::new() };
    ($($x:expr),+ $(,)?) => {
        $crate::iced::widget::Column::<_, $crate::HudTheme, _>::with_children([
            $($crate::iced::Element::<_, $crate::HudTheme, _>::from($x)),+
        ])
    };
}

/// 水平布局（子元素主题钉定为 [`HudTheme`]）。
#[macro_export]
macro_rules! row {
    () => { $crate::iced::widget::Row::<_, $crate::HudTheme, _>::new() };
    ($($x:expr),+ $(,)?) => {
        $crate::iced::widget::Row::<_, $crate::HudTheme, _>::with_children([
            $($crate::iced::Element::<_, $crate::HudTheme, _>::from($x)),+
        ])
    };
}

/// 叠层布局（子元素主题钉定为 [`HudTheme`]）。
#[macro_export]
macro_rules! stack {
    () => { $crate::iced::widget::Stack::<_, $crate::HudTheme, _>::new() };
    ($($x:expr),+ $(,)?) => {
        $crate::iced::widget::Stack::<_, $crate::HudTheme, _>::with_children([
            $($crate::iced::Element::<_, $crate::HudTheme, _>::from($x)),+
        ])
    };
}
