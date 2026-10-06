//! 首页仪表盘：数据读出、链路状态、负载进度与最近事件。

use hud_theme::HudTheme;
use hud_tokens::Tokens;
use hud_widgets::{column, progress::SegmentedProgress, row};

use crate::{Message, Sample};

/// 最近事件（演示数据）。
struct Event {
    time: &'static str,
    node: &'static str,
    level: &'static str,
    detail: &'static str,
}

const EVENTS: [Event; 5] = [
    Event {
        time: "T+00:00",
        node: "NX-01",
        level: "INFO",
        detail: "链路建立，握手完成",
    },
    Event {
        time: "T+03:12",
        node: "NX-04",
        level: "WARN",
        detail: "负载超过 90% 阈值",
    },
    Event {
        time: "T+07:45",
        node: "NX-02",
        level: "INFO",
        detail: "数据同步完成",
    },
    Event {
        time: "T+11:03",
        node: "NX-03",
        level: "INFO",
        detail: "例行自检通过",
    },
    Event {
        time: "T+12:30",
        node: "NX-05",
        level: "ERROR",
        detail: "心跳丢失，转备份链路",
    },
];

fn average_online_load(nodes: &[crate::NodeRecord]) -> f32 {
    let mut count = 0;
    let sum: f32 = nodes
        .iter()
        .filter(|node| node.online)
        .map(|node| {
            count += 1;
            node.load
        })
        .sum();

    if count == 0 {
        0.0
    } else {
        sum / count as f32
    }
}

/// 渲染仪表盘页。
pub fn dashboard(s: &Sample) -> iced::Element<'_, Message, HudTheme> {
    let tokens = s.theme.tokens;

    let online = s.nodes.iter().filter(|n| n.online).count();
    let total = s.nodes.len();
    let avg_load = average_online_load(&s.nodes);
    let avg_latency: u32 = {
        let live: Vec<_> = s.nodes.iter().filter(|n| n.online).collect();
        if live.is_empty() {
            0
        } else {
            live.iter().map(|n| n.latency_ms).sum::<u32>() / live.len() as u32
        }
    };
    let offline = total - online;

    // 顶部数据读出区
    let readouts = row![
        readout("在线节点", &format!("{online}/{total}"), None, &tokens),
        readout(
            "平均负载",
            &format!("{:.0}", avg_load * 100.0),
            Some("%"),
            &tokens
        ),
        readout("平均延迟", &format!("{avg_latency}"), Some("ms"), &tokens),
        readout("离线告警", &format!("{offline}"), None, &tokens),
    ]
    .spacing(tokens.geometry.space.l);

    // 链路状态面板
    let link_panel = hud_widgets::status_panel::status_panel(
        "主链路 / PRIMARY",
        if s.link_online {
            "延迟 24ms · 丢包 0.0% · 全速运行"
        } else {
            "心跳丢失 · 流量已切换至备份链路"
        },
        if s.link_online {
            hud_widgets::status_panel::OpStatus::Online
        } else {
            hud_widgets::status_panel::OpStatus::Offline
        },
        &tokens,
        300.0,
    );
    let backup_panel = hud_widgets::status_panel::status_panel(
        "备份链路 / BACKUP",
        "延迟 96ms · 低带宽 · 待命",
        hud_widgets::status_panel::OpStatus::Warning,
        &tokens,
        300.0,
    );

    // 负载进度（分段）
    let load_section = column![
        hud_widgets::typography::section_header("系统负载", "SYSTEM LOAD"),
        iced::widget::canvas(SegmentedProgress::new(avg_load, 12))
            .width(iced::Length::Fill)
            .height(hud_widgets::progress::recommended_height(&tokens)),
        iced::widget::text(format!("平均负载 {:.0}% · 阈值 90%", avg_load * 100.0))
            .size(12)
            .class(hud_widgets::typography::secondary_text()),
    ]
    .spacing(tokens.geometry.space.s);

    // 最近事件表
    let event_table: iced::Element<'_, Message, HudTheme> = hud_widgets::table::table(
        &[
            hud_widgets::table::TableColumn {
                title: "时间",
                width: 96.0,
                cell: |e: &Event| e.time.to_string(),
            },
            hud_widgets::table::TableColumn {
                title: "节点",
                width: 88.0,
                cell: |e: &Event| e.node.to_string(),
            },
            hud_widgets::table::TableColumn {
                title: "级别",
                width: 72.0,
                cell: |e: &Event| e.level.to_string(),
            },
            hud_widgets::table::TableColumn {
                title: "内容",
                width: 320.0,
                cell: |e: &Event| e.detail.to_string(),
            },
        ],
        &EVENTS,
        &tokens,
        None,
        None,
    );

    column![
        readouts,
        hud_widgets::typography::section_header("链路状态", "LINK STATUS"),
        row![link_panel, backup_panel].spacing(tokens.geometry.space.l),
        load_section,
        hud_widgets::typography::section_header("最近事件", "RECENT EVENTS"),
        event_table,
    ]
    .spacing(tokens.geometry.space.l)
    .into()
}

/// 单个数据读出（含底部基线）。
fn readout(
    label: &str,
    value: &str,
    unit: Option<&str>,
    tokens: &Tokens,
) -> iced::Element<'static, Message, HudTheme> {
    iced::widget::container(hud_widgets::data_readout::data_readout(
        label, value, unit, tokens,
    ))
    .width(200.0)
    .into()
}

#[cfg(test)]
mod tests {
    use super::average_online_load;
    use crate::NodeRecord;

    #[test]
    fn average_load_ignores_offline_nodes() {
        let nodes = [
            NodeRecord {
                name: String::from("A"),
                region: String::new(),
                load: 0.6,
                latency_ms: 10,
                online: true,
            },
            NodeRecord {
                name: String::from("B"),
                region: String::new(),
                load: 0.8,
                latency_ms: 0,
                online: false,
            },
        ];
        assert!((average_online_load(&nodes) - 0.6).abs() < f32::EPSILON);
    }
}
