//! HUD 风格的受控左右分栏组件。

use hud_theme::HudTheme;
use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer;
use iced::advanced::widget::tree::{self, Tree};
use iced::advanced::{mouse, overlay, Clipboard, Shell, Widget};
use iced::widget::{container, rule};
use iced::{Element, Event, Length, Rectangle, Size, Vector};

/// 分栏比例的最小值。
pub const MIN_RATIO: f32 = 0.2;
/// 分栏比例的最大值。
pub const MAX_RATIO: f32 = 0.8;
/// 两侧区域的默认最小像素宽度。
pub const DEFAULT_MIN_PANE_PIXELS: f32 = 160.0;
/// 分隔条的布局宽度。
pub const DIVIDER_WIDTH: f32 = 6.0;
/// 分隔条额外的鼠标命中范围。
pub const DIVIDER_HIT_SLOP: f32 = 6.0;

/// 受控分栏状态。
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SplitPaneState {
    /// 左侧区域占总宽度的比例。
    pub ratio: f32,
}

impl SplitPaneState {
    /// 创建默认比例为 50% 的分栏状态。
    pub const fn new() -> Self {
        Self { ratio: 0.5 }
    }

    /// 返回钳制后的比例。
    pub fn clamped(self) -> Self {
        Self {
            ratio: clamp_ratio(self.ratio),
        }
    }
}

impl Default for SplitPaneState {
    fn default() -> Self {
        Self::new()
    }
}

/// 将比例限制在可用的最小和最大范围内。
pub fn clamp_ratio(ratio: f32) -> f32 {
    ratio.clamp(MIN_RATIO, MAX_RATIO)
}

/// 将比例转换为便于测试的千分比两侧宽度。
pub fn split_portions(ratio: f32) -> (u16, u16) {
    let left = (clamp_ratio(ratio) * 1000.0).round() as u16;
    (left, 1000 - left)
}

fn pane_widths(ratio: f32, total_width: f32) -> (f32, f32) {
    let content_width = (total_width - DIVIDER_WIDTH).max(0.0);
    if content_width <= 0.0 {
        return (0.0, 0.0);
    }

    let minimum = DEFAULT_MIN_PANE_PIXELS.min(content_width * 0.5);
    let left = (content_width * clamp_ratio(ratio)).clamp(minimum, content_width - minimum);
    (left, content_width - left)
}

fn ratio_from_cursor(
    cursor_x: f32,
    total_width: f32,
    divider_width: f32,
    grab_offset: f32,
    fallback_ratio: f32,
) -> f32 {
    let content_width = total_width - divider_width;
    if content_width <= 0.0 {
        return clamp_ratio(fallback_ratio);
    }

    let position = cursor_x - grab_offset;
    let min_width = DEFAULT_MIN_PANE_PIXELS.min(content_width * 0.5);
    let left = position.clamp(min_width, content_width - min_width);
    clamp_ratio(left / content_width)
}

fn divider_bounds(bounds: Rectangle, left_width: f32) -> Rectangle {
    Rectangle {
        x: bounds.x + left_width,
        y: bounds.y,
        width: DIVIDER_WIDTH,
        height: bounds.height,
    }
}

fn divider_hit(bounds: Rectangle, left_width: f32) -> Rectangle {
    let divider = divider_bounds(bounds, left_width);
    Rectangle {
        x: divider.x - DIVIDER_HIT_SLOP,
        width: divider.width + DIVIDER_HIT_SLOP * 2.0,
        ..divider
    }
}

struct SplitPaneWidget<'a, Message> {
    first: Element<'a, Message, HudTheme>,
    divider: Element<'a, Message, HudTheme>,
    second: Element<'a, Message, HudTheme>,
    ratio: f32,
    on_ratio_changed: Box<dyn Fn(f32) -> Message + 'a>,
}

struct SplitPaneInnerState {
    dragging: bool,
    grab_offset: f32,
    original_ratio: f32,
}

impl Default for SplitPaneInnerState {
    fn default() -> Self {
        Self {
            dragging: false,
            grab_offset: 0.0,
            original_ratio: 0.5,
        }
    }
}

impl<'a, Message> SplitPaneWidget<'a, Message> {
    fn children_elements(&self) -> [&Element<'a, Message, HudTheme>; 3] {
        [&self.first, &self.divider, &self.second]
    }
}

impl<Message> Widget<Message, HudTheme, iced::Renderer> for SplitPaneWidget<'_, Message>
where
    Message: Clone + 'static,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<SplitPaneInnerState>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(SplitPaneInnerState::default())
    }

    fn children(&self) -> Vec<Tree> {
        self.children_elements()
            .into_iter()
            .map(Tree::new)
            .collect()
    }

    fn diff(&self, tree: &mut Tree) {
        for (child, state) in self.children_elements().into_iter().zip(&mut tree.children) {
            child.as_widget().diff(state);
        }
    }

    fn size(&self) -> Size<Length> {
        Size::new(Length::Fill, Length::Fill)
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let bounds = limits.resolve(Length::Fill, Length::Fill, Size::ZERO);
        let (left_width, right_width) = pane_widths(self.ratio, bounds.width);
        let mut children = Vec::with_capacity(3);
        let mut x = 0.0;
        for (child, width) in [left_width, DIVIDER_WIDTH, right_width]
            .into_iter()
            .enumerate()
        {
            let limits = layout::Limits::new(
                Size::new(width, bounds.height),
                Size::new(width, bounds.height),
            );
            let mut node = match child {
                0 => self
                    .first
                    .as_widget_mut()
                    .layout(&mut tree.children[0], renderer, &limits),
                1 => self
                    .divider
                    .as_widget_mut()
                    .layout(&mut tree.children[1], renderer, &limits),
                _ => self
                    .second
                    .as_widget_mut()
                    .layout(&mut tree.children[2], renderer, &limits),
            };
            node.move_to_mut(iced::Point::new(x, 0.0));
            children.push(node);
            x += width;
        }

        layout::Node::with_children(bounds, children)
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn iced::advanced::widget::Operation,
    ) {
        operation.container(None, layout.bounds());
        operation.traverse(&mut |operation| {
            self.first.as_widget_mut().operate(
                &mut tree.children[0],
                layout.children().nth(0).unwrap(),
                renderer,
                operation,
            );
            self.divider.as_widget_mut().operate(
                &mut tree.children[1],
                layout.children().nth(1).unwrap(),
                renderer,
                operation,
            );
            self.second.as_widget_mut().operate(
                &mut tree.children[2],
                layout.children().nth(2).unwrap(),
                renderer,
                operation,
            );
        });
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &iced::Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_mut::<SplitPaneInnerState>();
        let bounds = layout.bounds();
        let (left_width, _) = pane_widths(self.ratio, bounds.width);
        let hit = divider_hit(bounds, left_width);

        if state.dragging {
            match event {
                Event::Mouse(mouse::Event::CursorMoved { position }) => {
                    let next = ratio_from_cursor(
                        position.x - bounds.x,
                        bounds.width,
                        DIVIDER_WIDTH,
                        state.grab_offset,
                        self.ratio,
                    );
                    if (next - self.ratio).abs() > f32::EPSILON {
                        shell.publish((self.on_ratio_changed)(next));
                    }
                    shell.capture_event();
                    return;
                }
                Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                    state.dragging = false;
                    shell.capture_event();
                    return;
                }
                Event::Keyboard(iced::keyboard::Event::KeyPressed { key, .. })
                    if *key == iced::keyboard::Key::Named(iced::keyboard::key::Named::Escape) =>
                {
                    state.dragging = false;
                    shell.publish((self.on_ratio_changed)(state.original_ratio));
                    shell.capture_event();
                    return;
                }
                Event::Window(iced::window::Event::Unfocused) => {
                    state.dragging = false;
                    shell.publish((self.on_ratio_changed)(state.original_ratio));
                    shell.capture_event();
                    return;
                }
                _ => {}
            }
        }

        if let Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) = event {
            if cursor.position_over(hit).is_some() {
                let divider_center = left_width + DIVIDER_WIDTH * 0.5;
                let position = cursor.position().unwrap().x - bounds.x;
                state.dragging = true;
                state.original_ratio = self.ratio;
                state.grab_offset = position - divider_center;
                shell.capture_event();
                return;
            }
        }

        if state.dragging {
            return;
        }

        self.first.as_widget_mut().update(
            &mut tree.children[0],
            event,
            layout.children().nth(0).unwrap(),
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );
        self.divider.as_widget_mut().update(
            &mut tree.children[1],
            event,
            layout.children().nth(1).unwrap(),
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );
        self.second.as_widget_mut().update(
            &mut tree.children[2],
            event,
            layout.children().nth(2).unwrap(),
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
        renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        let state = tree.state.downcast_ref::<SplitPaneInnerState>();
        let bounds = layout.bounds();
        let (left_width, _) = pane_widths(self.ratio, bounds.width);
        if state.dragging || cursor.is_over(divider_hit(bounds, left_width)) {
            return mouse::Interaction::ResizingHorizontally;
        }

        self.children_elements()
            .into_iter()
            .zip(&tree.children)
            .zip(layout.children())
            .map(|((child, child_state), child_layout)| {
                child.as_widget().mouse_interaction(
                    child_state,
                    child_layout,
                    cursor,
                    viewport,
                    renderer,
                )
            })
            .find(|interaction| *interaction != mouse::Interaction::None)
            .unwrap_or_default()
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut iced::Renderer,
        theme: &HudTheme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        self.first.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            layout.children().nth(0).unwrap(),
            cursor,
            viewport,
        );
        self.divider.as_widget().draw(
            &tree.children[1],
            renderer,
            theme,
            style,
            layout.children().nth(1).unwrap(),
            cursor,
            viewport,
        );
        self.second.as_widget().draw(
            &tree.children[2],
            renderer,
            theme,
            style,
            layout.children().nth(2).unwrap(),
            cursor,
            viewport,
        );
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &iced::Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, HudTheme, iced::Renderer>> {
        let mut children = Vec::new();
        let (first_tree, rest) = tree.children.split_at_mut(1);
        let (divider_tree, second_tree) = rest.split_at_mut(1);
        let mut layouts = layout.children();
        let first_layout = layouts.next().unwrap();
        let divider_layout = layouts.next().unwrap();
        let second_layout = layouts.next().unwrap();

        if let Some(overlay) = self.first.as_widget_mut().overlay(
            &mut first_tree[0],
            first_layout,
            renderer,
            viewport,
            translation,
        ) {
            children.push(overlay);
        }
        if let Some(overlay) = self.divider.as_widget_mut().overlay(
            &mut divider_tree[0],
            divider_layout,
            renderer,
            viewport,
            translation,
        ) {
            children.push(overlay);
        }
        if let Some(overlay) = self.second.as_widget_mut().overlay(
            &mut second_tree[0],
            second_layout,
            renderer,
            viewport,
            translation,
        ) {
            children.push(overlay);
        }
        (!children.is_empty()).then(|| overlay::Group::with_children(children).overlay())
    }
}

impl<'a, Message> From<SplitPaneWidget<'a, Message>> for Element<'a, Message, HudTheme>
where
    Message: Clone + 'a + 'static,
{
    fn from(widget: SplitPaneWidget<'a, Message>) -> Self {
        Element::new(widget)
    }
}

/// 渲染受控左右分栏，并支持拖拽分隔条调整比例。
pub fn split_pane<'a, Message: Clone + 'a + 'static>(
    state: &SplitPaneState,
    first: Element<'a, Message, HudTheme>,
    second: Element<'a, Message, HudTheme>,
    on_ratio_changed: impl Fn(f32) -> Message + 'a,
) -> Element<'a, Message, HudTheme> {
    SplitPaneWidget {
        first: container(first).into(),
        divider: rule::vertical(1).into(),
        second: container(second).into(),
        ratio: state.clamped().ratio,
        on_ratio_changed: Box::new(on_ratio_changed),
    }
    .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ratio_is_clamped_to_safe_bounds() {
        assert_eq!(clamp_ratio(-1.0), MIN_RATIO);
        assert_eq!(clamp_ratio(2.0), MAX_RATIO);
        assert_eq!(clamp_ratio(0.5), 0.5);
    }

    #[test]
    fn split_widths_preserve_total_portions() {
        assert_eq!(split_portions(0.25), (250, 750));
        assert_eq!(split_portions(0.5), (500, 500));
    }

    #[test]
    fn ratio_from_cursor_preserves_grab_offset() {
        assert!((ratio_from_cursor(501.0, 1000.0, 6.0, 4.0, 0.5) - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn ratio_from_cursor_respects_minimum_pixel_width() {
        assert!((ratio_from_cursor(100.0, 500.0, 6.0, 0.0, 0.5) - 0.32388663).abs() < f32::EPSILON);
    }

    #[test]
    fn ratio_from_cursor_degrades_safely_for_small_windows() {
        assert!(ratio_from_cursor(10.0, 4.0, 6.0, 0.0, 0.5).is_finite());
        assert_eq!(ratio_from_cursor(10.0, 4.0, 6.0, 0.0, 0.5), 0.5);
    }

    #[test]
    fn pane_widths_keep_divider_out_of_ratio() {
        let (left, right) = pane_widths(0.5, 1006.0);
        assert_eq!(left, 500.0);
        assert_eq!(right, 500.0);
    }
}
