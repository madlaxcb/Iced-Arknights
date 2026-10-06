//! 测试辅助：样式快照与几何断言（headless，无渲染器依赖）。
//!
//! 渲染层质量以“快照 + 走查清单”为主（计划书 7.2）；本 crate 先提供
//! 样式函数全状态枚举断言工具。

use hud_theme::HudTheme;

/// 对给定 widget 的所有状态执行断言，确保样式函数全状态可用（不 panic 且返回合法 Style）。
#[macro_export]
macro_rules! assert_all_statuses {
    ($style_fn:path, $theme:expr, $statuses:expr) => {
        for status in $statuses {
            let _ = $style_fn($theme, status);
        }
    };
}

/// 返回内置暗色主题（测试基线）。
pub fn test_theme() -> HudTheme {
    HudTheme::dark()
}
