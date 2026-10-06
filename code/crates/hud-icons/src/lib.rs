//! 内嵌 SVG 图标与装饰素材（include_bytes!，按 feature 裁剪）。
//!
//! 图标统一 24×24 栅格、纯路径、无滤镜（计划书 3.4）。素材在 M5 阶段补充，
//! 当前先建立 feature 与命名约定。

/// 图标资源句柄（iced svg Handle）。
pub enum Icon {
    /// 关闭（×）
    Close,
    /// 最小化
    Minimize,
    /// 最大化 / 还原
    Maximize,
}

impl Icon {
    /// 图标 SVG 字节（当前为占位路径，M5 替换为正式素材）。
    pub fn bytes(self) -> &'static [u8] {
        // 占位：1×1 透明 SVG；正式图标随后续里程碑以 include_bytes! 嵌入
        const PLACEHOLDER: &[u8] =
            br#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24"/>"#;
        match self {
            Icon::Close | Icon::Minimize | Icon::Maximize => PLACEHOLDER,
        }
    }
}
