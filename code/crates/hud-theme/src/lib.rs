//! HUD 风格组件库——主题：`HudTheme`、各内置 widget 的 Catalog 实现、
//! 样式函数与主题文件加载。
//!
//! 样式函数全部为 `fn(&HudTheme, Status) -> Style` 纯函数，可独立单测
//! （计划书 2.4 / 5.8）。组件内部只读 Token，禁止字面颜色与尺寸。

use hud_tokens::Tokens;
use iced::widget::overlay::menu;
use iced::widget::{
    button, checkbox, container, pick_list, progress_bar, radio, rule, scrollable, slider, text,
    text_editor, text_input, toggler,
};
use iced::{theme, Border, Color};
use serde::de::DeserializeOwned;

/// HUD 主题（方案 B，见 doc/ADR-001）。
#[derive(Debug, Clone, PartialEq)]
pub struct HudTheme {
    /// 设计 Token（可整体替换以支持热更新）。
    pub tokens: Tokens,
}

impl Default for HudTheme {
    fn default() -> Self {
        Self::dark()
    }
}

impl HudTheme {
    /// 内置暗色主题（首版唯一主题，计划书 6.1）。
    pub fn dark() -> Self {
        Self {
            tokens: Tokens::dark(),
        }
    }

    /// 从 TOML 字符串加载 Token 构建主题。
    pub fn from_toml_str(s: &str) -> Result<Self, Error> {
        Ok(Self {
            tokens: toml::from_str::<Tokens>(s)?,
        })
    }

    /// 从 TOML 文件加载主题。
    pub fn from_path<P: AsRef<std::path::Path>>(path: P) -> Result<Self, Error> {
        let s = std::fs::read_to_string(path)?;
        Self::from_toml_str(&s)
    }

    /// 反序列化任意 TOML 配置（供 testkit / gallery 使用）。
    pub fn parse_toml<T: DeserializeOwned>(s: &str) -> Result<T, Error> {
        Ok(toml::from_str(s)?)
    }
}

/// 主题加载错误。
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// IO 错误
    #[error(transparent)]
    Io(#[from] std::io::Error),
    /// TOML 解析错误
    #[error(transparent)]
    Toml(#[from] toml::de::Error),
}

impl theme::Base for HudTheme {
    fn default(_preference: theme::Mode) -> Self {
        Self::dark()
    }

    fn mode(&self) -> theme::Mode {
        theme::Mode::Dark
    }

    fn base(&self) -> theme::Style {
        let p = &self.tokens.palette;
        theme::Style {
            background_color: color(p.bg0),
            text_color: color(p.text_primary),
        }
    }

    fn palette(&self) -> Option<theme::Palette> {
        let p = &self.tokens.palette;
        Some(theme::Palette {
            background: color(p.bg0),
            text: color(p.text_primary),
            primary: color(p.accent),
            success: color(p.success),
            warning: color(p.accent),
            danger: color(p.danger),
        })
    }

    fn name(&self) -> &str {
        "hud-dark"
    }
}

/// Token 颜色 → iced 颜色。
pub fn color(c: hud_tokens::Color) -> Color {
    Color::from_rgb8(c.r, c.g, c.b)
}

/// 无圆角 1px 边框（HUD 线条语言的基本单位）。
fn border(c: hud_tokens::Color, width: f32) -> Border {
    Border {
        color: color(c),
        width,
        radius: 0.0.into(),
    }
}

// ---------------------------------------------------------------------------
// 各 widget 样式函数（纯函数，公开供 testkit 快照与单测使用）
// ---------------------------------------------------------------------------

/// container 样式（注意：0.14 的 container Catalog 无 Status）。
pub fn style_container(theme: &HudTheme, _status: ()) -> container::Style {
    let p = &theme.tokens.palette;
    container::Style {
        text_color: Some(color(p.text_primary)),
        background: Some(color(p.bg0).into()),
        border: border(p.line_base, 0.0),
        ..Default::default()
    }
}

/// button 样式：底色加深 + 边框点亮 + 强调色文字（计划书 1.4-7）。
pub fn style_button(theme: &HudTheme, status: button::Status) -> button::Style {
    let p = &theme.tokens.palette;
    let (bg, line, fg) = match status {
        button::Status::Active => (p.bg1, p.line_base, p.text_primary),
        button::Status::Hovered => (p.bg2, p.line_strong, p.accent),
        button::Status::Pressed => (p.bg0, p.line_strong, p.accent),
        button::Status::Disabled => (p.bg1, p.line_dim, p.text_disabled),
    };
    button::Style {
        background: Some(color(bg).into()),
        text_color: color(fg),
        border: border(line, theme.tokens.geometry.line.base),
        ..Default::default()
    }
}

/// text_input 样式：聚焦时边框点亮。
pub fn style_text_input(theme: &HudTheme, status: text_input::Status) -> text_input::Style {
    let p = &theme.tokens.palette;
    let line = match status {
        text_input::Status::Focused { .. } | text_input::Status::Hovered => p.line_strong,
        text_input::Status::Active | text_input::Status::Disabled => p.line_base,
    };
    let value = match status {
        text_input::Status::Disabled => p.text_disabled,
        _ => p.text_primary,
    };
    text_input::Style {
        background: color(p.bg1).into(),
        border: border(line, theme.tokens.geometry.line.base),
        icon: color(p.text_secondary),
        placeholder: color(p.text_secondary),
        value: color(value),
        selection: color(p.accent).scale_alpha(0.3),
    }
}

/// text 默认继承父级前景色（次级文字请用 text::Style 显式覆盖）。
pub fn style_text(_theme: &HudTheme) -> text::Style {
    text::Style { color: None }
}

/// checkbox 样式：选中时以强调色填充。
pub fn style_checkbox(theme: &HudTheme, status: checkbox::Status) -> checkbox::Style {
    let p = &theme.tokens.palette;
    let (checked, hovered) = match status {
        checkbox::Status::Active { is_checked } => (is_checked, false),
        checkbox::Status::Hovered { is_checked } => (is_checked, true),
        checkbox::Status::Disabled { .. } => (false, false),
    };
    let is_disabled = matches!(status, checkbox::Status::Disabled { .. });
    let line = if hovered || checked {
        p.line_strong
    } else {
        p.line_base
    };
    let icon = if is_disabled {
        p.text_disabled
    } else if checked {
        p.bg0
    } else {
        p.text_secondary
    };
    let bg = if is_disabled {
        p.bg2
    } else if checked {
        p.accent
    } else {
        p.bg1
    };
    checkbox::Style {
        background: color(bg).into(),
        icon_color: color(icon),
        border: border(line, theme.tokens.geometry.line.base),
        text_color: Some(color(if is_disabled {
            p.text_disabled
        } else {
            p.text_primary
        })),
    }
}

/// toggler 样式：方形滑块，开态强调色。
pub fn style_toggler(theme: &HudTheme, status: toggler::Status) -> toggler::Style {
    let p = &theme.tokens.palette;
    let on = matches!(
        status,
        toggler::Status::Active { is_toggled: true }
            | toggler::Status::Hovered { is_toggled: true }
    );
    let disabled = matches!(status, toggler::Status::Disabled { .. });
    let line = match status {
        toggler::Status::Active { .. } => p.line_base,
        toggler::Status::Hovered { .. } => p.line_strong,
        toggler::Status::Disabled { .. } => p.line_dim,
    };
    toggler::Style {
        background: color(if disabled {
            p.bg1
        } else if on {
            p.accent
        } else {
            p.bg2
        })
        .into(),
        background_border_width: theme.tokens.geometry.line.base,
        background_border_color: color(line),
        foreground: color(if disabled {
            p.text_disabled
        } else if on {
            p.bg0
        } else {
            p.text_secondary
        })
        .into(),
        foreground_border_width: 0.0,
        foreground_border_color: color(p.bg0),
        text_color: Some(color(if disabled {
            p.text_disabled
        } else {
            p.text_primary
        })),
        // HUD 几何语言：方形，不做圆角
        border_radius: Some(0.0.into()),
        padding_ratio: 0.25,
    }
}

/// radio 样式。
pub fn style_radio(theme: &HudTheme, status: radio::Status) -> radio::Style {
    let p = &theme.tokens.palette;
    let (selected, hovered) = match status {
        radio::Status::Active { is_selected } => (is_selected, false),
        radio::Status::Hovered { is_selected } => (is_selected, true),
    };
    radio::Style {
        background: color(p.bg1).into(),
        dot_color: color(if selected { p.accent } else { p.text_secondary }),
        border_width: theme.tokens.geometry.line.base,
        border_color: color(if hovered { p.line_strong } else { p.line_base }),
        text_color: Some(color(p.text_primary)),
    }
}

/// slider 样式：细线轨道 + 方形滑块（计划书 1.2）。
pub fn style_slider(theme: &HudTheme, status: slider::Status) -> slider::Style {
    let p = &theme.tokens.palette;
    let rail_color = match status {
        slider::Status::Active => p.line_base,
        slider::Status::Hovered | slider::Status::Dragged => p.line_strong,
    };
    slider::Style {
        rail: slider::Rail {
            backgrounds: (color(p.line_dim).into(), color(rail_color).into()),
            width: theme.tokens.geometry.line.strong,
            border: border(p.line_dim, 0.0),
        },
        handle: slider::Handle {
            shape: slider::HandleShape::Rectangle {
                width: 8,
                border_radius: 0.0.into(),
            },
            background: color(p.text_primary).into(),
            border_width: theme.tokens.geometry.line.base,
            border_color: color(p.line_strong),
        },
    }
}

/// progress_bar 样式：暗轨 + 强调色进度。
pub fn style_progress_bar(theme: &HudTheme) -> progress_bar::Style {
    let p = &theme.tokens.palette;
    progress_bar::Style {
        background: color(p.bg2).into(),
        bar: color(p.accent).into(),
        border: border(p.line_base, theme.tokens.geometry.line.base),
    }
}

/// scrollable 样式：主题化细滚动条。
pub fn style_scrollable(theme: &HudTheme, _status: scrollable::Status) -> scrollable::Style {
    let p = &theme.tokens.palette;
    scrollable::Style {
        container: container::Style::default(),
        vertical_rail: scrollable::Rail {
            background: Some(color(p.bg1).into()),
            border: border(p.line_dim, 0.0),
            scroller: scrollable::Scroller {
                background: color(p.line_base).into(),
                border: border(p.line_strong, 0.0),
            },
        },
        horizontal_rail: scrollable::Rail {
            background: Some(color(p.bg1).into()),
            border: border(p.line_dim, 0.0),
            scroller: scrollable::Scroller {
                background: color(p.line_base).into(),
                border: border(p.line_strong, 0.0),
            },
        },
        gap: None,
        auto_scroll: scrollable::AutoScroll {
            background: color(p.bg2).into(),
            border: border(p.line_base, theme.tokens.geometry.line.base),
            shadow: Default::default(),
            icon: color(p.text_secondary),
        },
    }
}

/// text_editor（多行编辑器）样式：与 text_input 同一视觉语言。
pub fn style_text_editor(theme: &HudTheme, status: text_editor::Status) -> text_editor::Style {
    let p = &theme.tokens.palette;
    let line = match status {
        text_editor::Status::Focused { .. } | text_editor::Status::Hovered => p.line_strong,
        text_editor::Status::Active | text_editor::Status::Disabled => p.line_base,
    };
    text_editor::Style {
        background: color(p.bg1).into(),
        border: border(line, theme.tokens.geometry.line.base),
        placeholder: color(p.text_secondary),
        value: color(p.text_primary),
        selection: color(p.accent).scale_alpha(0.3),
    }
}

/// rule（分隔线）样式。
pub fn style_rule(theme: &HudTheme) -> rule::Style {
    rule::Style {
        color: color(theme.tokens.palette.line_dim),
        radius: 0.0.into(),
        fill_mode: rule::FillMode::Full,
        snap: true,
    }
}

/// pick_list 样式。
pub fn style_pick_list(theme: &HudTheme, status: pick_list::Status) -> pick_list::Style {
    let p = &theme.tokens.palette;
    let line = match status {
        pick_list::Status::Active => p.line_base,
        pick_list::Status::Hovered | pick_list::Status::Opened { .. } => p.line_strong,
    };
    pick_list::Style {
        text_color: color(p.text_primary),
        placeholder_color: color(p.text_secondary),
        handle_color: color(p.text_secondary),
        background: color(p.bg1).into(),
        border: border(line, theme.tokens.geometry.line.base),
    }
}

/// pick_list 下拉菜单样式。
pub fn style_pick_list_menu(theme: &HudTheme) -> menu::Style {
    let p = &theme.tokens.palette;
    menu::Style {
        background: color(p.bg1).into(),
        border: border(p.line_base, theme.tokens.geometry.line.base),
        text_color: color(p.text_primary),
        selected_text_color: color(p.accent),
        selected_background: color(p.bg2).into(),
        shadow: Default::default(),
    }
}

// ---------------------------------------------------------------------------
// Catalog 实现
// ---------------------------------------------------------------------------

impl container::Catalog for HudTheme {
    type Class<'a> = container::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(|t| style_container(t, ()))
    }

    fn style(&self, class: &Self::Class<'_>) -> container::Style {
        class(self)
    }
}

impl button::Catalog for HudTheme {
    type Class<'a> = button::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(style_button)
    }

    fn style(&self, class: &Self::Class<'_>, status: button::Status) -> button::Style {
        class(self, status)
    }
}

impl text_input::Catalog for HudTheme {
    type Class<'a> = text_input::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(style_text_input)
    }

    fn style(&self, class: &Self::Class<'_>, status: text_input::Status) -> text_input::Style {
        class(self, status)
    }
}

impl text::Catalog for HudTheme {
    type Class<'a> = text::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(style_text)
    }

    fn style(&self, class: &Self::Class<'_>) -> text::Style {
        class(self)
    }
}

impl checkbox::Catalog for HudTheme {
    type Class<'a> = checkbox::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(style_checkbox)
    }

    fn style(&self, class: &Self::Class<'_>, status: checkbox::Status) -> checkbox::Style {
        class(self, status)
    }
}

impl toggler::Catalog for HudTheme {
    type Class<'a> = toggler::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(style_toggler)
    }

    fn style(&self, class: &Self::Class<'_>, status: toggler::Status) -> toggler::Style {
        class(self, status)
    }
}

impl radio::Catalog for HudTheme {
    type Class<'a> = radio::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(style_radio)
    }

    fn style(&self, class: &Self::Class<'_>, status: radio::Status) -> radio::Style {
        class(self, status)
    }
}

impl slider::Catalog for HudTheme {
    type Class<'a> = slider::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(style_slider)
    }

    fn style(&self, class: &Self::Class<'_>, status: slider::Status) -> slider::Style {
        class(self, status)
    }
}

impl progress_bar::Catalog for HudTheme {
    type Class<'a> = progress_bar::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(style_progress_bar)
    }

    fn style(&self, class: &Self::Class<'_>) -> progress_bar::Style {
        class(self)
    }
}

impl scrollable::Catalog for HudTheme {
    type Class<'a> = scrollable::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(style_scrollable)
    }

    fn style(&self, class: &Self::Class<'_>, status: scrollable::Status) -> scrollable::Style {
        class(self, status)
    }
}

impl text_editor::Catalog for HudTheme {
    type Class<'a> = text_editor::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(style_text_editor)
    }

    fn style(&self, class: &Self::Class<'_>, status: text_editor::Status) -> text_editor::Style {
        class(self, status)
    }
}

impl rule::Catalog for HudTheme {
    type Class<'a> = rule::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(style_rule)
    }

    fn style(&self, class: &Self::Class<'_>) -> rule::Style {
        class(self)
    }
}

impl menu::Catalog for HudTheme {
    type Class<'a> = menu::StyleFn<'a, Self>;

    fn default<'a>() -> <Self as menu::Catalog>::Class<'a> {
        Box::new(style_pick_list_menu)
    }

    fn style(&self, class: &<Self as menu::Catalog>::Class<'_>) -> menu::Style {
        class(self)
    }
}

impl pick_list::Catalog for HudTheme {
    type Class<'a> = pick_list::StyleFn<'a, Self>;

    fn default<'a>() -> <Self as pick_list::Catalog>::Class<'a> {
        Box::new(style_pick_list)
    }

    fn style(
        &self,
        class: &<Self as pick_list::Catalog>::Class<'_>,
        status: pick_list::Status,
    ) -> pick_list::Style {
        class(self, status)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn theme_loads_from_toml_with_section_fallback() {
        // 完整 palette 可加载
        let p = Tokens::dark().palette;
        let c = |x: hud_tokens::Color| format!("{{ r = {}, g = {}, b = {} }}", x.r, x.g, x.b);
        let theme = HudTheme::from_toml_str(&format!(
            "[palette]\nbg0 = {}\nbg1 = {}\nbg2 = {}\nline_dim = {}\nline_base = {}\nline_strong = {}\ntext_primary = {}\ntext_secondary = {}\ntext_disabled = {}\naccent = {}\ninfo = {}\ndanger = {}\nsuccess = {}\n",
            c(p.bg0), c(p.bg1), c(p.bg2), c(p.line_dim), c(p.line_base), c(p.line_strong),
            c(p.text_primary), c(p.text_secondary), c(p.text_disabled), c(p.accent), c(p.info), c(p.danger), c(p.success),
        ))
        .expect("加载完整主题");
        assert_eq!(theme.tokens.palette, p);

        // 缺失整个 section 时回退默认（serde default）
        let fallback = HudTheme::from_toml_str("").expect("空配置");
        assert_eq!(fallback.tokens, Tokens::dark());
    }

    #[test]
    fn text_style_leaves_color_inheritable() {
        assert_eq!(style_text(&HudTheme::dark()).color, None);
    }

    #[test]
    fn style_functions_are_total_over_status() {
        let theme = HudTheme::dark();
        for s in [
            button::Status::Active,
            button::Status::Hovered,
            button::Status::Pressed,
            button::Status::Disabled,
        ] {
            let _ = style_button(&theme, s);
        }
        for s in [
            text_input::Status::Active,
            text_input::Status::Hovered,
            text_input::Status::Focused { is_hovered: true },
            text_input::Status::Focused { is_hovered: false },
            text_input::Status::Disabled,
        ] {
            let _ = style_text_input(&theme, s);
        }
        let _ = style_scrollable(
            &theme,
            scrollable::Status::Active {
                is_horizontal_scrollbar_disabled: true,
                is_vertical_scrollbar_disabled: false,
            },
        );
    }
}
