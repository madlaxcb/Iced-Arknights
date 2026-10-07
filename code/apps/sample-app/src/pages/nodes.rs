//! 节点列表 + 详情：左列表右详情的主从布局，重启走确认模态。

use hud_theme::HudTheme;
use hud_widgets::{column, row};

use crate::{Message, Sample};

/// 渲染节点管理页。
pub fn nodes(s: &Sample) -> iced::Element<'_, Message, HudTheme> {
    let tokens = s.theme.tokens;
    let breadcrumb_items = vec![
        hud_widgets::breadcrumb::BreadcrumbItem::new(0, "节点管理"),
        hud_widgets::breadcrumb::BreadcrumbItem::new(1, "在线节点"),
        hud_widgets::breadcrumb::BreadcrumbItem::new(2, "节点详情"),
    ];
    let breadcrumbs = hud_widgets::breadcrumb::breadcrumb(
        breadcrumb_items,
        s.breadcrumb_selected,
        &Message::BreadcrumbSelect,
    );

    let items: Vec<hud_widgets::list::ListItem> = s
        .nodes
        .iter()
        .enumerate()
        .map(|(i, n)| hud_widgets::list::ListItem {
            id: i,
            label: n.name.clone(),
            meta: Some(if n.online {
                format!("{}ms", n.latency_ms)
            } else {
                String::from("离线")
            }),
        })
        .collect();

    let list_panel = column![
        hud_widgets::typography::section_header("节点清单", "NODES"),
        hud_widgets::list::list(&items, s.selected, &Message::NodeSelect),
    ]
    .spacing(tokens.geometry.space.m);

    let detail: iced::Element<'_, Message, HudTheme> = match s.selected.and_then(|i| s.nodes.get(i))
    {
        None => iced::widget::text("未选中节点")
            .size(14)
            .class(hud_widgets::typography::secondary_text())
            .into(),
        Some(n) => {
            let status_badge = if n.online {
                hud_widgets::badge::badge("在线 / ONLINE", hud_widgets::badge::BadgeKind::Success)
            } else {
                hud_widgets::badge::badge("离线 / OFFLINE", hud_widgets::badge::BadgeKind::Danger)
            };

            let mut detail_col = column![
                iced::widget::row![iced::widget::text(n.name.clone()).size(20), status_badge,]
                    .spacing(tokens.geometry.space.m)
                    .align_y(iced::alignment::Vertical::Center),
                kv(String::from("区域"), n.region.clone(), &tokens),
                kv(
                    String::from("延迟"),
                    format!("{} ms", n.latency_ms),
                    &tokens
                ),
                kv(
                    String::from("负载"),
                    format!("{:.0}%", n.load * 100.0),
                    &tokens
                ),
                iced::widget::canvas(hud_widgets::progress::SlantedProgress::new(n.load))
                    .width(iced::Length::Fill)
                    .height(hud_widgets::progress::recommended_height(&tokens)),
            ]
            .spacing(tokens.geometry.space.m);

            // 只有在线节点允许重启（离线节点先走链路恢复流程）
            if n.online {
                if let Some(i) = s.selected {
                    detail_col = detail_col.push(
                        iced::widget::button(iced::widget::text("重启节点 / RESTART").size(14))
                            .padding(8)
                            .on_press(Message::AskRestart(i))
                            .class(hud_widgets::button::class(
                                hud_widgets::button::ButtonVariant::Danger,
                            )),
                    );
                }
            }

            hud_widgets::panel::panel(detail_col, &tokens, hud_widgets::panel::ChamferLevel::M)
                .padding(16)
                .into()
        }
    };

    hud_widgets::column![
        breadcrumbs,
        row![
            iced::widget::button(iced::widget::text(if s.node_context_menu_open {
                "关闭节点菜单 / CLOSE MENU"
            } else {
                "节点操作 / NODE ACTIONS"
            }))
            .padding(8)
            .on_press(if s.node_context_menu_open {
                Message::CloseNodeContextMenu
            } else {
                Message::OpenNodeContextMenu
            }),
            if s.node_context_menu_open {
                hud_widgets::context_menu::context_menu(
                    vec![
                        hud_widgets::context_menu::ContextMenuItem::action(1, "查看详情"),
                        hud_widgets::context_menu::ContextMenuItem::action(2, "复制节点 ID"),
                    ],
                    s.node_context_menu_state,
                    &Message::NodeContextMenuSelect,
                )
            } else {
                iced::widget::text("菜单关闭").size(12).into()
            },
        ]
        .spacing(tokens.geometry.space.s),
        hud_widgets::split_pane::split_pane(
            &s.split_pane_state,
            iced::widget::container(list_panel).padding(12).into(),
            detail,
            Message::SplitPaneRatio,
        ),
    ]
    .spacing(tokens.geometry.space.m)
    .into()
}

/// 键值对一行。
fn kv(
    key: String,
    value: String,
    tokens: &hud_tokens::Tokens,
) -> iced::Element<'static, Message, HudTheme> {
    iced::widget::row![
        iced::widget::container(iced::widget::text(key).size(13)).width(72.0),
        iced::widget::text(value)
            .size(13)
            .class(hud_widgets::typography::secondary_text()),
    ]
    .spacing(tokens.geometry.space.s)
    .into()
}
