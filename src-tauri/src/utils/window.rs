use anyhow::Result;
use tauri_plugin_decorum::WebviewWindowExt;

pub fn set_titlebar_style(win: &tauri::WebviewWindow) -> Result<()> {
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    {
        win.create_overlay_titlebar()?;
    }

    #[cfg(target_os = "macos")]
    {
        win.set_traffic_lights_inset(12.0, 16.0)?;
    }

    Ok(())
}

pub fn set_background_effect(win: &tauri::WebviewWindow) -> Result<()> {
    // 非常好 helper function，不使用 macOS Private APIs
    win.make_transparent()?;
    #[cfg(target_os = "windows")]
    {
        window_vibrancy::apply_mica(win, None)?;
    }

    #[cfg(target_os = "macos")]
    {
        window_vibrancy::apply_vibrancy(
            win,
            window_vibrancy::NSVisualEffectMaterial::HeaderView,
            Some(window_vibrancy::NSVisualEffectState::Active),
            None,
        )?;
    }

    Ok(())
}
