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
