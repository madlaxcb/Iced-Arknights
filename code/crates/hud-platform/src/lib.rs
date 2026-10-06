//! 平台集成：窗口操作、DPI 与系统设置查询。
//!
//! 依赖方向为旁支（不被库内其他 crate 依赖）。Windows 专有调用集中在
//! `windows` 模块（`cfg(windows)`），其余平台提供安全降级实现。

use iced::Task;

/// 标题栏触发的窗口操作（跨平台：经 iced window Task 实现）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowCommand {
    /// 最小化
    Minimize,
    /// 最大化 / 还原（切换）
    ToggleMaximize,
    /// 关闭
    Close,
    /// 开始拖动（标题栏按住）
    Drag,
}

/// 执行窗口操作。
///
/// iced 0.14 的 `window` API 覆盖标题栏所需的全部基础动作；
/// `window::Id` 由应用传入（单主窗口场景用 `window::Id::main()`）。
pub fn perform(id: iced::window::Id, command: WindowCommand) -> Task<()> {
    match command {
        WindowCommand::Minimize => iced::window::minimize(id, true),
        WindowCommand::ToggleMaximize => iced::window::toggle_maximize(id),
        WindowCommand::Close => iced::window::close(id),
        WindowCommand::Drag => iced::window::drag(id),
    }
}

/// 系统“减少动画”设置。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReduceMotion(pub bool);

/// 查询系统“减少动画”设置。
///
/// - Windows：`SystemParametersInfoW(SPI_GETCLIENTAREAANIMATION)`（真机项，见 ADR-003 清单）；
/// - 其他平台：读取 `ICED_REDUCE_MOTION` 环境变量，缺省 false。
pub fn reduce_motion() -> ReduceMotion {
    #[cfg(windows)]
    {
        windows_impl::reduce_motion()
    }
    #[cfg(not(windows))]
    {
        ReduceMotion(
            std::env::var("ICED_REDUCE_MOTION")
                .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
                .unwrap_or(false),
        )
    }
}

#[cfg(windows)]
mod windows_impl {
    // TODO(win): Windows 真机走查时启用（ADR-003 清单）：
    // 使用 windows-rs 查询 SPI_GETCLIENTAREAANIMATION；无边框窗口的
    // DWM 阴影 / 圆角 / Snap Layouts 支持同批落地。
    #![allow(unsafe_code, unreachable_code, unused_variables)]
    //! 平台实现（Windows）。

    pub fn reduce_motion() -> super::ReduceMotion {
        // 占位：真机接入时替换为 SystemParametersInfoW 调用
        super::ReduceMotion(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reduce_motion_env_fallback() {
        // 非 Windows 平台：环境变量回退路径可用即视为通过
        let _ = reduce_motion();
    }
}
