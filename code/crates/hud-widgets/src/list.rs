//! List / ListItem：可选中、可悬停的列表（受控组件，状态由应用持有）。

use hud_theme::HudTheme;
use iced::widget::{button, column, row, text};
use iced::{Element, Length, Padding};

use crate::typography::secondary_text;

/// 列表项数据。
pub struct ListItem {
    /// 稳定标识（回调返回值）
    pub id: usize,
    /// 主文本
    pub label: String,
    /// 右侧次要文本（可空）
    pub meta: Option<String>,
}

/// 渲染列表（选中项强调，悬停点亮；键盘可达性依赖 button 焦点链）。
pub fn list<'a, Message: Clone + 'a>(
    items: &[ListItem],
    selected: Option<usize>,
    on_select: &'a dyn Fn(usize) -> Message,
) -> Element<'a, Message, HudTheme> {
    let rows: Vec<Element<'a, Message, HudTheme>> = items
        .iter()
        .map(|item| {
            let id = item.id;
            let is_selected = selected == Some(id);
            let meta = item.meta.clone().unwrap_or_default();
            let row = row![
                text(item.label.clone()).size(14),
                text(meta).size(12).class(secondary_text()),
            ]
            .spacing(8);

            button(row)
                .width(Length::Fill)
                .padding(Padding::from([8, 12]))
                .on_press(on_select(id))
                .class(Box::new(move |theme: &HudTheme, status: button::Status| {
                    let p = &theme.tokens.palette;
                    let (bg, line, fg) = if is_selected {
                        (p.bg2, p.line_strong, p.accent)
                    } else {
                        match status {
                            button::Status::Hovered => (p.bg1, p.line_dim, p.text_primary),
                            _ => (p.bg0, p.bg0, p.text_primary),
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

    column(rows).padding(0).into()
}
