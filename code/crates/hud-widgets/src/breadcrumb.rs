//! HUD 风格的受控面包屑导航。

use hud_theme::HudTheme;
use iced::widget::{button, container, row, text};
use iced::{Element, Length, Padding};

/// 面包屑导航项。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BreadcrumbItem {
    /// 稳定标识。
    pub id: usize,
    /// 显示文本。
    pub label: String,
}

impl BreadcrumbItem {
    /// 创建面包屑导航项。
    pub fn new(id: usize, label: impl Into<String>) -> Self {
        Self {
            id,
            label: label.into(),
        }
    }
}

/// 返回有效的当前项索引；空列表返回 `None`。
pub fn effective_selected(items: &[BreadcrumbItem], selected: usize) -> Option<usize> {
    (!items.is_empty()).then(|| selected.min(items.len() - 1))
}

/// 判断某个面包屑索引是否可点击；最后一项不可点击。
pub fn is_selectable(items: &[BreadcrumbItem], index: usize) -> bool {
    !items.is_empty() && index < items.len().saturating_sub(1)
}

/// 渲染受控面包屑导航。
pub fn breadcrumb<'a, Message: Clone + 'a>(
    items: Vec<BreadcrumbItem>,
    selected: usize,
    on_select: &'a dyn Fn(usize) -> Message,
) -> Element<'a, Message, HudTheme> {
    let current = effective_selected(items.as_slice(), selected);
    let children = items.iter().enumerate().flat_map(|(index, item)| {
        let item_view: Element<'a, Message, HudTheme> = if is_selectable(items.as_slice(), index) {
            let id = item.id;
            button(text(item.label.clone()).size(13))
                .padding(Padding::from([4, 6]))
                .on_press(on_select(id))
                .class(Box::new(|theme: &HudTheme, status: button::Status| {
                    let p = &theme.tokens.palette;
                    let background = match status {
                        button::Status::Hovered => Some(hud_theme::color(p.bg2).into()),
                        _ => None,
                    };
                    button::Style {
                        background,
                        text_color: hud_theme::color(p.text_secondary),
                        ..Default::default()
                    }
                }) as button::StyleFn<'_, HudTheme>)
                .into()
        } else {
            let color = if current == Some(index) {
                items
                    .get(index)
                    .map(|_| hud_theme::color(HudTheme::dark().tokens.palette.accent))
                    .unwrap_or_default()
            } else {
                hud_theme::color(HudTheme::dark().tokens.palette.text_primary)
            };
            text(item.label.clone()).size(13).color(color).into()
        };
        let mut views = vec![item_view];
        if index + 1 < items.len() {
            views.push(
                text("/")
                    .size(13)
                    .color(hud_theme::color(
                        HudTheme::dark().tokens.palette.line_strong,
                    ))
                    .into(),
            );
        }
        views
    });

    container(row(children).spacing(4))
        .width(Length::Fill)
        .padding(Padding::from([4, 0]))
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn last_item_is_not_selectable() {
        let items = vec![
            BreadcrumbItem::new(1, "首页"),
            BreadcrumbItem::new(2, "设置"),
        ];

        assert!(is_selectable(&items, 0));
        assert!(!is_selectable(&items, 1));
    }

    #[test]
    fn selected_index_is_clamped_to_last_item() {
        let items = vec![BreadcrumbItem::new(1, "首页")];

        assert_eq!(effective_selected(&items, 9), Some(0));
        assert_eq!(effective_selected(&[], 9), None);
    }
}
