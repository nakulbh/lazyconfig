use gpui::*;

/// Color constants for the application theme (returned as functions since
/// gpui::rgb() is not const in v0.2.2)
pub struct Theme;

impl Theme {
    // Background colors
    pub fn bg_primary() -> Hsla {
        rgb(0x1e1e1e).into()
    }
    pub fn bg_secondary() -> Hsla {
        rgb(0x252526).into()
    }
    pub fn bg_tertiary() -> Hsla {
        rgb(0x2d2d30).into()
    }
    pub fn bg_elevated() -> Hsla {
        rgb(0x3c3c3c).into()
    }
    pub fn bg_hover() -> Hsla {
        rgb(0x2a2d2e).into()
    }
    pub fn bg_selected() -> Hsla {
        rgb(0x094771).into()
    }

    // Text colors
    pub fn text_primary() -> Hsla {
        rgb(0xcccccc).into()
    }
    pub fn text_secondary() -> Hsla {
        rgb(0x9cdcfe).into()
    }
    pub fn text_muted() -> Hsla {
        rgb(0x858585).into()
    }

    // Border colors
    pub fn border() -> Hsla {
        rgb(0x3e3e42).into()
    }
    pub fn border_active() -> Hsla {
        rgb(0x007acc).into()
    }

    // Platform badges
    pub fn claude_badge() -> Hsla {
        rgb(0xd97757).into()
    }
    pub fn opencode_badge() -> Hsla {
        rgb(0x58a6ff).into()
    }
    pub fn codex_badge() -> Hsla {
        rgb(0x10a37f).into()
    }
    pub fn copilot_badge() -> Hsla {
        rgb(0xa371f7).into()
    }

    // Status
    pub fn success() -> Hsla {
        rgb(0x4ec9b0).into()
    }
    pub fn error() -> Hsla {
        rgb(0xf48771).into()
    }
}

/// Spacing helpers
pub fn pad_small() -> Pixels {
    px(8.0)
}
pub fn pad_medium() -> Pixels {
    px(16.0)
}
pub fn pad_large() -> Pixels {
    px(24.0)
}

pub fn sidebar_width() -> Pixels {
    px(260.0)
}
