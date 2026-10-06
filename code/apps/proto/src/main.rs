//! M0 技术预研原型（计划书 4-M0）
//!
//! P1 切角外形：用 canvas 路径画 45° 切角面板，验证布局缩放与内容叠加；
//! P5 主题架构：自定义 Theme 类型（方案 B），实现 theme::Base 与各 widget 的
//! Catalog，应用以 `Element<'a, Msg, ProtoTheme>` 运行。
//!
//! 0.14 API 备忘（M0 实测结论，供 ADR-001 引用）：
//! - `StyleFn<'a, Theme> = Box<dyn Fn(&Theme, Status) -> Style + 'a>`，
//!   `.class()` 需要 `Box::new(f) as StyleFn` 显式装箱（std 无 `From<fn>` for `Box<dyn Fn>`）；
//! - `column!/row!/stack!` 宏在子元素处调用 `Element::from`，会把 Theme 钉死为
//!   `iced::Theme`，自定义主题必须用 `Column::<Msg, Theme>::with_children` 显式构建；
//! - `container::Catalog` 已无 Status 参数（0.14 移除）；
//! - `text_input::Status::Focused { is_hovered }` 为结构体变体；
//! - `application(boot, update, view)` 首参是 BootFn（初始状态），标题用 `.title()`。

use iced::theme::{Mode, Palette};
use iced::widget::{button, canvas, container, text, text_input};
use iced::{
    application, mouse, Border, Center, Color, Element, Length, Point, Rectangle, Renderer, Size,
    Task,
};

fn main() -> iced::Result {
    application(State::new, State::update, State::view)
        .title("hud proto (M0)")
        .theme(|_: &State| ProtoTheme)
        .window_size((900.0, 640.0))
        .run()
}

// ---------------------------------------------------------------------------
// P5：自定义 Theme（方案 B 最小实现）
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, Default, PartialEq)]
struct ProtoTheme;

// Token 起步参考值（计划书附录 B，M2 会移入 hud-tokens）
const BG_WINDOW: Color = Color::from_rgb8(0x0E, 0x0F, 0x11);
const BG_PANEL: Color = Color::from_rgb8(0x16, 0x18, 0x1C);
const BG_PANEL_HOVER: Color = Color::from_rgb8(0x1F, 0x22, 0x28);
const LINE_BASE: Color = Color::from_rgb8(0x3A, 0x3F, 0x47);
const LINE_STRONG: Color = Color::from_rgb8(0x5A, 0x60, 0x6B);
const TEXT_PRIMARY: Color = Color::from_rgb8(0xE6, 0xE8, 0xEB);
const TEXT_SECONDARY: Color = Color::from_rgb8(0x8A, 0x90, 0x99);
const ACCENT: Color = Color::from_rgb8(0xFF, 0xD4, 0x00);

impl iced::theme::Base for ProtoTheme {
    fn default(_preference: Mode) -> Self {
        Self
    }

    fn mode(&self) -> Mode {
        Mode::Dark
    }

    fn base(&self) -> iced::theme::Style {
        iced::theme::Style {
            background_color: BG_WINDOW,
            text_color: TEXT_PRIMARY,
        }
    }

    fn palette(&self) -> Option<Palette> {
        Some(Palette {
            background: BG_WINDOW,
            text: TEXT_PRIMARY,
            primary: ACCENT,
            success: Color::from_rgb8(0x4A, 0xDE, 0x80),
            warning: ACCENT,
            danger: Color::from_rgb8(0xFF, 0x4D, 0x4F),
        })
    }

    fn name(&self) -> &str {
        "Proto"
    }
}

impl container::Catalog for ProtoTheme {
    type Class<'a> = container::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(|_theme: &ProtoTheme| container::Style {
            text_color: Some(TEXT_PRIMARY),
            background: Some(BG_WINDOW.into()),
            border: Border {
                color: LINE_BASE,
                width: 0.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        })
    }

    fn style(&self, class: &Self::Class<'_>) -> container::Style {
        class(self)
    }
}

impl button::Catalog for ProtoTheme {
    type Class<'a> = button::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(button_style)
    }

    fn style(&self, class: &Self::Class<'_>, status: button::Status) -> button::Style {
        class(self, status)
    }
}

impl text_input::Catalog for ProtoTheme {
    type Class<'a> = text_input::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(text_input_style)
    }

    fn style(&self, class: &Self::Class<'_>, status: text_input::Status) -> text_input::Style {
        class(self, status)
    }
}

impl text::Catalog for ProtoTheme {
    type Class<'a> = text::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(|_theme: &ProtoTheme| text::Style {
            color: Some(TEXT_PRIMARY),
        })
    }

    fn style(&self, class: &Self::Class<'_>) -> text::Style {
        class(self)
    }
}

// ---------------------------------------------------------------------------
// 状态
// ---------------------------------------------------------------------------

struct State {
    value: String,
    clicks: u32,
}

impl State {
    fn new() -> Self {
        Self {
            value: String::new(),
            clicks: 0,
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    Input(String),
    Pressed,
}

impl State {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Input(v) => self.value = v,
            Message::Pressed => self.clicks += 1,
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message, ProtoTheme> {
        // 内容层（文字永远保持矩形，用内边距避开切角——计划书 1.4-5）
        let content: Element<'_, Message, ProtoTheme> =
            iced::widget::Column::<Message, ProtoTheme, Renderer>::with_children(vec![
                text("HUD 风格面板 / CHAMFER PANEL").size(20).into(),
                text("45° chamfer · canvas path · custom theme")
                    .size(12)
                    .class(Box::new(secondary_text) as text::StyleFn<'_, ProtoTheme>)
                    .into(),
                text_input("在此输入…", &self.value)
                    .on_input(Message::Input)
                    .padding(8)
                    .width(280)
                    .class(Box::new(text_input_style) as text_input::StyleFn<'_, ProtoTheme>)
                    .into(),
                button(text("确认 / CONFIRM").size(14))
                    .on_press(Message::Pressed)
                    .padding(10)
                    .class(Box::new(button_style) as button::StyleFn<'_, ProtoTheme>)
                    .into(),
                text(format!("点击次数: {}", self.clicks)).size(14).into(),
            ])
            .spacing(14)
            .align_x(Center)
            .into();

        let content = container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Center)
            .align_y(Center)
            .class(Box::new(hud_layer_style) as container::StyleFn<'_, ProtoTheme>);

        // 形状层：大面板 12px 切角 + 小面板 8px 切角（P1，Cache 缓存绘制）
        // P6 对照实验（计划书 v0.2）：同屏 4 画布，Cache 与 Live 写法各 2 个
        fn cell<'a>(
            el: Element<'a, Message, ProtoTheme>,
            label: &'static str,
        ) -> Element<'a, Message, ProtoTheme> {
            iced::widget::Column::<Message, ProtoTheme, Renderer>::with_children(vec![
                el,
                text(label).size(12).into(),
            ])
            .spacing(4)
            .into()
        }

        let c1 = cell(
            canvas(CacheShape {
                chamfer: 12.0,
                accent: true,
                cache: canvas::Cache::new(),
            })
            .width(Length::Fill)
            .height(150)
            .into(),
            "C1 · Cache 多图元",
        );
        let c2 = cell(
            canvas(CacheShape {
                chamfer: 8.0,
                accent: false,
                cache: canvas::Cache::new(),
            })
            .width(Length::Fill)
            .height(150)
            .into(),
            "C2 · Cache 多图元",
        );
        let l1 = cell(
            canvas(LiveShape {
                chamfer: 12.0,
                accent: true,
            })
            .width(Length::Fill)
            .height(150)
            .into(),
            "L1 · Live 多图元",
        );
        let l2 = cell(
            canvas(LiveShape {
                chamfer: 8.0,
                accent: false,
            })
            .width(Length::Fill)
            .height(150)
            .into(),
            "L2 · Live 多图元",
        );

        let shapes: Element<'_, Message, ProtoTheme> =
            iced::widget::Column::<Message, ProtoTheme, Renderer>::with_children(vec![
                iced::widget::Row::<Message, ProtoTheme, Renderer>::with_children(vec![c1, c2])
                    .spacing(12)
                    .into(),
                iced::widget::Row::<Message, ProtoTheme, Renderer>::with_children(vec![l1, l2])
                    .spacing(12)
                    .into(),
            ])
            .spacing(12)
            .width(Length::Fill)
            .into();

        iced::widget::Stack::<Message, ProtoTheme, Renderer>::with_children(vec![
            shapes,
            content.into(),
        ])
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}

fn secondary_text(_theme: &ProtoTheme) -> text::Style {
    text::Style {
        color: Some(TEXT_SECONDARY),
    }
}

fn hud_layer_style(_theme: &ProtoTheme) -> container::Style {
    container::Style {
        text_color: Some(TEXT_PRIMARY),
        ..Default::default()
    }
}

fn text_input_style(_theme: &ProtoTheme, status: text_input::Status) -> text_input::Style {
    let border = match status {
        text_input::Status::Focused { .. } | text_input::Status::Hovered => LINE_STRONG,
        text_input::Status::Active | text_input::Status::Disabled => LINE_BASE,
    };
    text_input::Style {
        background: BG_PANEL.into(),
        border: Border {
            color: border,
            width: 1.0,
            radius: 0.0.into(),
        },
        icon: TEXT_SECONDARY,
        placeholder: TEXT_SECONDARY,
        value: TEXT_PRIMARY,
        selection: ACCENT.scale_alpha(0.3),
    }
}

fn button_style(_theme: &ProtoTheme, status: button::Status) -> button::Style {
    let border = match status {
        button::Status::Hovered | button::Status::Pressed => LINE_STRONG,
        button::Status::Active | button::Status::Disabled => LINE_BASE,
    };
    let background = match status {
        button::Status::Hovered => BG_PANEL_HOVER,
        button::Status::Pressed => BG_WINDOW,
        button::Status::Active | button::Status::Disabled => BG_PANEL,
    };
    button::Style {
        background: Some(background.into()),
        text_color: match status {
            button::Status::Hovered => ACCENT,
            button::Status::Disabled => TEXT_SECONDARY,
            button::Status::Active | button::Status::Pressed => TEXT_PRIMARY,
        },
        border: Border {
            color: border,
            width: 1.0,
            radius: 0.0.into(),
        },
        ..Default::default()
    }
}

// ---------------------------------------------------------------------------
// P1：切角外形（R1 路线：canvas 路径填充 + 描边）
// P6：Cache 写法（CacheShape）与 Live 写法（LiveShape）对照
// ---------------------------------------------------------------------------

struct CacheShape {
    chamfer: f32,
    accent: bool,
    cache: canvas::Cache,
}

/// 生成 45° 切角矩形路径（后续沉淀到 hud-core，并配几何单测）
fn chamfered_rect_path(rect: Rectangle, chamfer: f32) -> canvas::Path {
    let c = chamfer.clamp(0.0, rect.width.min(rect.height) / 2.0);
    let x = rect.x;
    let y = rect.y;
    let w = rect.width;
    let h = rect.height;
    canvas::Path::new(|p| {
        p.move_to(Point::new(x + c, y));
        p.line_to(Point::new(x + w - c, y));
        p.line_to(Point::new(x + w, y + c));
        p.line_to(Point::new(x + w, y + h - c));
        p.line_to(Point::new(x + w - c, y + h));
        p.line_to(Point::new(x + c, y + h));
        p.line_to(Point::new(x, y + h - c));
        p.line_to(Point::new(x, y + c));
        p.close();
    })
}

/// 两种写法共用的图元序列（切角填充 + 描边 + 可选顶条），
/// 确保 Cache / Live 对照只有缓存方式一个变量。
fn draw_shape(frame: &mut canvas::Frame, size: Size, chamfer: f32, accent: bool) {
    let path = chamfered_rect_path(Rectangle::with_size(size), chamfer);

    frame.fill(&path, BG_PANEL);
    frame.stroke(
        &path,
        canvas::Stroke::default()
            .with_color(LINE_BASE)
            .with_width(1.0),
    );

    if accent {
        // 顶边 2px 强调线（切角范围内收缩）
        let c = chamfer;
        let accent = canvas::Path::new(|p| {
            p.move_to(Point::new(c, 0.5));
            p.line_to(Point::new(size.width - c, 0.5));
            p.line_to(Point::new(size.width - c, 2.5));
            p.line_to(Point::new(c, 2.5));
            p.close();
        });
        frame.fill(&accent, ACCENT);
    }
}

impl<Message> canvas::Program<Message, ProtoTheme> for CacheShape {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &ProtoTheme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        // Cache 写法（官方推荐模式）
        let frame = self.cache.draw(renderer, bounds.size(), |frame| {
            draw_shape(frame, bounds.size(), self.chamfer, self.accent);
        });

        vec![frame]
    }
}

/// Live 写法：每次绘制直接构建 Frame（不经 Cache）。
struct LiveShape {
    chamfer: f32,
    accent: bool,
}

impl<Message> canvas::Program<Message, ProtoTheme> for LiveShape {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &ProtoTheme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());
        draw_shape(&mut frame, bounds.size(), self.chamfer, self.accent);
        vec![frame.into_geometry()]
    }
}
