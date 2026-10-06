//! 切角矩形基础 Widget（R2 自定义 Widget + R1 canvas 路径，见 doc/ADR-002）。
//!
//! - 背景为 45° 切角多边形（canvas::Cache 缓存，仅在尺寸 / 外观变化时重绘）；
//! - 内容保持矩形，自动避让切角（内边距下限为切角的一半）；
//! - 命中测试规则：切角区域属于背景装饰，不参与命中（内容元素自带命中）。

use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer;
use iced::advanced::widget::tree::{self, Tree};
use iced::advanced::{graphics, mouse, Clipboard, Shell, Widget};
use iced::widget::canvas;
use iced::{Color, Element, Length, Padding, Point, Rectangle, Size, Vector};

use crate::geometry::{chamfer_clamp, chamfered_rect_points};

/// 切角盒子的外观参数（由上层从 Token 计算后传入）。
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Appearance {
    /// 切角尺寸（逻辑像素，45°）
    pub chamfer: f32,
    /// 填充色
    pub fill: Color,
    /// 边框色
    pub border_color: Color,
    /// 边框宽
    pub border_width: f32,
}

impl Default for Appearance {
    fn default() -> Self {
        Self {
            chamfer: 8.0,
            fill: Color::TRANSPARENT,
            border_color: Color::WHITE,
            border_width: 1.0,
        }
    }
}

/// 内部状态：缓存几何 + 上次外观（用于失效判断）。
struct State<R: graphics::geometry::Renderer> {
    cache: canvas::Cache<R>,
    appearance: Option<Appearance>,
}

impl<R: graphics::geometry::Renderer> Default for State<R> {
    fn default() -> Self {
        Self {
            cache: canvas::Cache::new(),
            appearance: None,
        }
    }
}

/// 切角盒子：切角背景 + 单个子内容。
pub struct ChamferedBox<'a, Message, Theme, Renderer>
where
    Renderer: graphics::geometry::Renderer,
{
    content: Element<'a, Message, Theme, Renderer>,
    width: Length,
    height: Length,
    padding: Padding,
    appearance: Appearance,
}

impl<'a, Message, Theme, Renderer> ChamferedBox<'a, Message, Theme, Renderer>
where
    Renderer: graphics::geometry::Renderer,
{
    /// 创建切角盒子。
    pub fn new(
        content: impl Into<Element<'a, Message, Theme, Renderer>>,
        appearance: Appearance,
    ) -> Self {
        Self {
            content: content.into(),
            width: Length::Shrink,
            height: Length::Shrink,
            padding: Padding::ZERO,
            appearance,
        }
    }

    /// 设置宽度。
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// 设置高度。
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// 设置内容内边距（实际避让量取 `max(padding, chamfer/2)`）。
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }

    fn content_inset(&self) -> Padding {
        let avoid = chamfer_clamp(f32::INFINITY, f32::INFINITY, self.appearance.chamfer) * 0.5
            + self.appearance.border_width * 0.5;
        let p = self.padding;
        Padding {
            top: p.top.max(avoid),
            bottom: p.bottom.max(avoid),
            left: p.left.max(avoid),
            right: p.right.max(avoid),
        }
    }
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for ChamferedBox<'_, Message, Theme, Renderer>
where
    Renderer: graphics::geometry::Renderer + iced::advanced::text::Renderer + 'static,
    Theme: 'static,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State<Renderer>>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::<Renderer>::default())
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        let state = tree.state.downcast_mut::<State<Renderer>>();
        if state.appearance != Some(self.appearance) {
            state.cache.clear();
            state.appearance = Some(self.appearance);
        }
        self.content.as_widget().diff(&mut tree.children[0]);
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: self.height,
        }
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let inset = self.content_inset();
        let limits = limits.width(self.width).height(self.height).shrink(inset);
        let mut child =
            self.content
                .as_widget_mut()
                .layout(&mut tree.children[0], renderer, &limits);
        child.translate_mut(Vector::new(inset.left, inset.top));
        let size = child.size().expand(inset);
        layout::Node::with_children(size, vec![child])
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn iced::advanced::widget::Operation,
    ) {
        operation.traverse(&mut |operation| {
            self.content.as_widget_mut().operate(
                &mut tree.children[0],
                layout.children().next().unwrap(),
                renderer,
                operation,
            );
        });
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &iced::Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        self.content.as_widget_mut().update(
            &mut tree.children[0],
            event,
            layout.children().next().unwrap(),
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.content.as_widget().mouse_interaction(
            &tree.children[0],
            layout.children().next().unwrap(),
            cursor,
            viewport,
            renderer,
        )
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let state = tree.state.downcast_ref::<State<Renderer>>();
        let appearance = self.appearance;

        let layer = state.cache.draw(&*renderer, bounds.size(), |frame| {
            let pts = chamfered_rect_points(bounds.width, bounds.height, appearance.chamfer);
            let path = canvas::Path::new(|p| {
                let (x0, y0) = pts[0];
                p.move_to(Point::new(x0, y0));
                for (x, y) in pts.iter().skip(1) {
                    p.line_to(Point::new(*x, *y));
                }
                p.close();
            });
            frame.fill(&path, appearance.fill);
            if appearance.border_width > 0.0 {
                frame.stroke(
                    &path,
                    canvas::Stroke::default()
                        .with_color(appearance.border_color)
                        .with_width(appearance.border_width),
                );
            }
        });

        // 平移到自身位置绘制背景（内容不受此平移影响）
        renderer.with_translation(Vector::new(bounds.x, bounds.y), |renderer| {
            renderer.draw_geometry(layer);
        });

        let child_layout = layout.children().next().unwrap();
        self.content.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            child_layout,
            cursor,
            viewport,
        );
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<iced::advanced::overlay::Element<'b, Message, Theme, Renderer>> {
        self.content.as_widget_mut().overlay(
            &mut tree.children[0],
            layout.children().next().unwrap(),
            renderer,
            viewport,
            translation,
        )
    }
}

impl<'a, Message, Theme, Renderer> From<ChamferedBox<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a + 'static,
    Renderer: graphics::geometry::Renderer + iced::advanced::text::Renderer + 'a + 'static,
{
    fn from(box_: ChamferedBox<'a, Message, Theme, Renderer>) -> Self {
        Element::new(box_)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn content_inset_avoids_chamfer() {
        let b = ChamferedBox::<(), iced::Theme, iced::Renderer>::new(
            iced::widget::text("x"),
            Appearance {
                chamfer: 12.0,
                border_width: 1.0,
                ..Default::default()
            },
        );
        let inset = b.content_inset();
        assert!(inset.left >= 6.0, "内容应至少避让切角一半");
        assert!(inset.top >= 6.0);
    }
}
