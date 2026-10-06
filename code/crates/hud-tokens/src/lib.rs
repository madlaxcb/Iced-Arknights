//! 设计 Token：HUD 风格组件库的唯一视觉常量来源。
//!
//! 组件内部只读 Token，禁止字面颜色与尺寸（计划书 1.3 / 6.2）。
//! 本 crate 为纯数据，不依赖 iced，可独立单测与序列化。

use serde::{Deserialize, Serialize};

/// 8bit RGB 颜色（const 可用，避免依赖 f32 转换）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Color {
    /// 红（0-255）
    pub r: u8,
    /// 绿（0-255）
    pub g: u8,
    /// 蓝（0-255）
    pub b: u8,
}

impl Color {
    /// 从 0xRRGGBB 创建（const）。
    pub const fn hex(rgb: u32) -> Self {
        Self {
            r: ((rgb >> 16) & 0xFF) as u8,
            g: ((rgb >> 8) & 0xFF) as u8,
            b: (rgb & 0xFF) as u8,
        }
    }

    /// 相对亮度（WCAG 2.x 口径）。
    pub fn relative_luminance(self) -> f64 {
        fn channel(c: f64) -> f64 {
            if c <= 0.03928 {
                c / 12.92
            } else {
                ((c + 0.055) / 1.055).powf(2.4)
            }
        }
        let r = channel(f64::from(self.r) / 255.0);
        let g = channel(f64::from(self.g) / 255.0);
        let b = channel(f64::from(self.b) / 255.0);
        0.2126 * r + 0.7152 * g + 0.0722 * b
    }

    /// 两色对比度（1.0-21.0，WCAG 2.x）。
    pub fn contrast_ratio(a: Color, b: Color) -> f64 {
        let (la, lb) = (a.relative_luminance(), b.relative_luminance());
        let (hi, lo) = if la >= lb { (la, lb) } else { (lb, la) };
        (hi + 0.05) / (lo + 0.05)
    }
}

/// 调色板：近黑底色分层 + 细线 + 文字 + 单一强调色与功能色。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Palette {
    /// 窗口底色（bg.0）
    pub bg0: Color,
    /// 面板底色（bg.1）
    pub bg1: Color,
    /// 悬浮 / 选中底色（bg.2）
    pub bg2: Color,
    /// 弱分隔线（line.dim）
    pub line_dim: Color,
    /// 常规边框（line.base）
    pub line_base: Color,
    /// 强调边框 / 悬停边框（line.strong）
    pub line_strong: Color,
    /// 主文字（text.primary）
    pub text_primary: Color,
    /// 次要文字（text.secondary）
    pub text_secondary: Color,
    /// 禁用文字（text.disabled）
    pub text_disabled: Color,
    /// 强调色（偏黄，克制使用，占屏 ≤5%~10%）
    pub accent: Color,
    /// 信息色（青）
    pub info: Color,
    /// 危险色（红）
    pub danger: Color,
    /// 成功色（绿）
    pub success: Color,
}

impl Default for Palette {
    fn default() -> Self {
        Self::dark()
    }
}

impl Palette {
    /// 暗色调色板（首版唯一内置主题，计划书 6.1）。
    pub const fn dark() -> Self {
        Self {
            bg0: Color::hex(0x0E_0F_11),
            bg1: Color::hex(0x16_18_1C),
            bg2: Color::hex(0x1F_22_28),
            line_dim: Color::hex(0x2C_30_37),
            line_base: Color::hex(0x3A_3F_47),
            line_strong: Color::hex(0x5A_60_6B),
            text_primary: Color::hex(0xE6_E8_EB),
            text_secondary: Color::hex(0x8A_90_99),
            text_disabled: Color::hex(0x55_5A_63),
            accent: Color::hex(0xFF_D4_00),
            info: Color::hex(0x3F_B6_FF),
            danger: Color::hex(0xFF_4D_4F),
            success: Color::hex(0x4A_DE_80),
        }
    }
}

/// 间距（4px 栅格）与切角/线宽档位。
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
pub struct Geometry {
    /// 间距档位（4px 基数）
    pub space: Spacing,
    /// 切角档位
    pub chamfer: Chamfer,
    /// 线宽档位
    pub line: LineWidth,
}

/// 间距档位（4px 基数：4 / 8 / 12 / 16 / 24 / 32）。
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Spacing {
    /// xs = 4
    pub xs: f32,
    /// s = 8
    pub s: f32,
    /// m = 12
    pub m: f32,
    /// l = 16
    pub l: f32,
    /// xl = 24
    pub xl: f32,
    /// xxl = 32
    pub xxl: f32,
}

impl Default for Spacing {
    fn default() -> Self {
        Self {
            xs: 4.0,
            s: 8.0,
            m: 12.0,
            l: 16.0,
            xl: 24.0,
            xxl: 32.0,
        }
    }
}

/// 切角档位（45° 斜切，无圆角）。
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Chamfer {
    /// s = 4
    pub s: f32,
    /// m = 8
    pub m: f32,
    /// l = 12
    pub l: f32,
}

impl Default for Chamfer {
    fn default() -> Self {
        Self {
            s: 4.0,
            m: 8.0,
            l: 12.0,
        }
    }
}

/// 线宽档位（1px 常规 / 2px 强调）。
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct LineWidth {
    /// 常规 = 1
    pub base: f32,
    /// 强调 = 2
    pub strong: f32,
}

impl Default for LineWidth {
    fn default() -> Self {
        Self {
            base: 1.0,
            strong: 2.0,
        }
    }
}

/// 字号阶梯（最小 12px，计划书附录 B）。
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Typography {
    /// 12 —— 辅助小字（下限）
    pub xxs: f32,
    /// 14 —— 正文
    pub xs: f32,
    /// 16 —— 强调正文
    pub s: f32,
    /// 20 —— 小标题
    pub m: f32,
    /// 28 —— 页面标题
    pub l: f32,
    /// 40 —— 展示数字
    pub xl: f32,
}

impl Default for Typography {
    fn default() -> Self {
        Self {
            xxs: 12.0,
            xs: 14.0,
            s: 16.0,
            m: 20.0,
            l: 28.0,
            xl: 40.0,
        }
    }
}

/// 动效时长（ms）与缓动。仅"单属性补间"，禁编排（计划书 6.1）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Motion {
    /// 120ms —— 交互反馈
    pub feedback_ms: u64,
    /// 200ms —— 内容切换
    pub transition_ms: u64,
    /// 280ms —— 浮层进出
    pub overlay_ms: u64,
}

impl Default for Motion {
    fn default() -> Self {
        Self {
            feedback_ms: 120,
            transition_ms: 200,
            overlay_ms: 280,
        }
    }
}

impl Motion {
    /// ease-out：仅支持缓出（短促、克制）。
    pub const EASING: &'static str = "ease-out";
}

/// 设计 Token 汇总（可由文件加载，serde 口径；缺省字段回退暗色默认）。
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
pub struct Tokens {
    /// 调色板
    #[serde(default)]
    pub palette: Palette,
    /// 几何
    #[serde(default)]
    pub geometry: Geometry,
    /// 排版
    #[serde(default)]
    pub typography: Typography,
    /// 动效
    #[serde(default)]
    pub motion: Motion,
}

impl Tokens {
    /// 暗色主题（首版内置）。
    pub fn dark() -> Self {
        Self {
            palette: Palette::dark(),
            geometry: Geometry::default(),
            typography: Typography::default(),
            motion: Motion::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dark_palette_is_serde_roundtrip() {
        let tokens = Tokens::dark();
        let s = toml::to_string(&tokens).expect("serialize");
        let back: Tokens = toml::from_str(&s).expect("deserialize");
        assert_eq!(tokens, back);
    }

    #[test]
    fn accent_on_bg0_meets_aa_for_normal_text() {
        let p = Palette::dark();
        let ratio = Color::contrast_ratio(p.accent, p.bg0);
        assert!(ratio >= 4.5, "accent/bg0 对比度 {ratio:.2} < 4.5");
    }

    #[test]
    fn text_primary_on_panel_meets_aa() {
        let p = Palette::dark();
        let ratio = Color::contrast_ratio(p.text_primary, p.bg1);
        assert!(ratio >= 4.5, "text_primary/bg1 对比度 {ratio:.2} < 4.5");
    }

    #[test]
    fn text_secondary_used_as_large_text_only_passes_3_to_1() {
        let p = Palette::dark();
        let ratio = Color::contrast_ratio(p.text_secondary, p.bg0);
        assert!(ratio >= 3.0, "text_secondary/bg0 对比度 {ratio:.2} < 3.0");
    }

    #[test]
    fn text_disabled_never_used_for_body_copy_is_documented_by_low_ratio() {
        let p = Palette::dark();
        let ratio = Color::contrast_ratio(p.text_disabled, p.bg1);
        assert!(ratio < 4.5, "禁用色应明显弱于正文（实测 {ratio:.2}）");
    }

    #[test]
    fn geometry_scales_are_on_the_4px_grid() {
        let g = Geometry::default();
        for v in [
            g.space.xs,
            g.space.s,
            g.space.m,
            g.space.l,
            g.space.xl,
            g.space.xxl,
        ] {
            assert_eq!(v % 4.0, 0.0, "{v} 不在 4px 栅格上");
        }
        for v in [g.chamfer.s, g.chamfer.m, g.chamfer.l] {
            assert_eq!(v % 4.0, 0.0, "切角 {v} 不在档位上");
        }
    }
}
