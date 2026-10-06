//! Table 定宽列表格（表头 + 数据行；不做虚拟化，计划书 1.2 P1 边界）。

use hud_theme::HudTheme;
use hud_tokens::Tokens;
use iced::widget::{button, column, container, row, text};
use iced::{Element, Length, Padding};

/// 表格列定义。
pub struct TableColumn<'a, T> {
    /// 表头文本
    pub title: &'a str,
    /// 列宽（逻辑像素，定宽）
    pub width: f32,
    /// 单元格渲染（返回文本）
    pub cell: fn(&T) -> String,
}

/// 渲染表格。
///
/// `on_select` 为 `Some` 时行可点击（悬停点亮 / 选中强调）。
pub fn table<'a, T, Message: Clone + 'a>(
    columns: &[TableColumn<'_, T>],
    rows: &[T],
    tokens: &Tokens,
    selected: Option<usize>,
    on_select: Option<&'a dyn Fn(usize) -> Message>,
) -> Element<'a, Message, HudTheme> {
    let header = header_row::<T, Message>(columns);
    let body: Vec<Element<'a, Message, HudTheme>> = rows
        .iter()
        .enumerate()
        .map(|(i, data)| match on_select {
            Some(on_select) => row_button(columns, data, i, selected, tokens, on_select).into(),
            None => plain_row(columns, data, i, selected),
        })
        .collect();

    column![header, column(body).padding(0)].spacing(0).into()
}

fn cells<'a, T, Message: Clone + 'a>(
    columns: &[TableColumn<'_, T>],
    data: &T,
) -> Vec<Element<'a, Message, HudTheme>> {
    columns
        .iter()
        .map(|col| {
            container(text((col.cell)(data)).size(13))
                .width(col.width)
                .align_x(iced::alignment::Horizontal::Left)
                .into()
        })
        .collect()
}

fn header_row<'a, T, Message: Clone + 'a>(
    columns: &[TableColumn<'_, T>],
) -> Element<'a, Message, HudTheme> {
    let cells: Vec<Element<'a, Message, HudTheme>> = columns
        .iter()
        .map(|col| {
            container(text(col.title.to_string()).size(12))
                .width(col.width)
                .align_x(iced::alignment::Horizontal::Left)
                .into()
        })
        .collect();

    container(row(cells).padding(Padding::from([6, 8])))
        .width(Length::Fill)
        .style(|theme: &HudTheme| {
            let p = &theme.tokens.palette;
            iced::widget::container::Style {
                text_color: Some(hud_theme::color(p.text_secondary)),
                background: Some(hud_theme::color(p.bg2).into()),
                border: iced::Border {
                    color: hud_theme::color(p.line_base),
                    width: theme.tokens.geometry.line.base,
                    radius: 0.0.into(),
                },
                ..Default::default()
            }
        })
        .into()
}

fn row_button<'a, T, Message: Clone + 'a>(
    columns: &[TableColumn<'_, T>],
    data: &T,
    index: usize,
    selected: Option<usize>,
    _tokens: &Tokens,
    on_select: &'a dyn Fn(usize) -> Message,
) -> button::Button<'a, Message, HudTheme> {
    let is_selected = selected == Some(index);
    let content = row(cells::<T, Message>(columns, data))
        .padding(Padding::from([6, 8]))
        .spacing(0);

    button(content)
        .width(Length::Fill)
        .on_press(on_select(index))
        .class(Box::new(move |theme: &HudTheme, status: button::Status| {
            let p = &theme.tokens.palette;
            let (bg, fg) = if is_selected {
                (p.bg2, p.accent)
            } else {
                match status {
                    button::Status::Hovered => (p.bg1, p.text_primary),
                    _ => (p.bg0, p.text_primary),
                }
            };
            button::Style {
                background: Some(hud_theme::color(bg).into()),
                text_color: hud_theme::color(fg),
                border: iced::Border {
                    color: hud_theme::color(p.bg0),
                    width: 0.0,
                    radius: 0.0.into(),
                },
                ..Default::default()
            }
        }) as button::StyleFn<'_, HudTheme>)
}

fn plain_row<'a, T, Message: Clone + 'a>(
    columns: &[TableColumn<'_, T>],
    data: &T,
    index: usize,
    selected: Option<usize>,
) -> Element<'a, Message, HudTheme> {
    let is_selected = selected == Some(index);
    let content = row(cells::<T, Message>(columns, data))
        .padding(Padding::from([6, 8]))
        .spacing(0);

    container(content)
        .width(Length::Fill)
        .style(move |theme: &HudTheme| {
            let p = &theme.tokens.palette;
            iced::widget::container::Style {
                text_color: Some(hud_theme::color(if is_selected {
                    p.accent
                } else {
                    p.text_primary
                })),
                background: Some(hud_theme::color(if is_selected { p.bg2 } else { p.bg0 }).into()),
                ..Default::default()
            }
        })
        .into()
}
