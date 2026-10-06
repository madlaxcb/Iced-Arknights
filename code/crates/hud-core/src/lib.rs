//! 基础设施：形状几何、像素对齐与对 iced 的薄适配层。
//!
//! 依赖方向：`hud-tokens → hud-core → …`。对 `iced::advanced` 的直接使用
//! 必须收敛在本 crate（计划书 2.1）。

pub mod chamfered_box;
pub mod geometry;

pub use chamfered_box::{Appearance, ChamferedBox};
pub use geometry::{chamfer_clamp, chamfered_rect_points, snap_to_pixel};
