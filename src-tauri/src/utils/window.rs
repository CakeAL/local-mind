use anyhow::Result;
use tauri::{utils::config::WindowConfig, WebviewWindow, WebviewWindowBuilder};
use tauri_plugin_decorum::WebviewWindowExt;

pub fn build_main_window(app: &tauri::App) -> Result<WebviewWindow> {
    let win_builder = WebviewWindowBuilder::from_config(
        app,
        &WindowConfig {
            hidden_title: true,
            label: "main".into(),
            width: 1080.0,
            height: 670.0,
            min_height: Some(600.0),
            min_width: Some(800.0),
            ..Default::default()
        },
    )?;
    #[cfg(target_os = "windows")]
    let win_builder = win_builder.transparent(true);
    #[cfg(target_os = "macos")]
    let win_builder = win_builder.title_bar_style(TitleBarStyle::Overlay);
    Ok(win_builder.build()?)
}

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
    #[cfg(target_os = "windows")]
    {
        window_vibrancy::apply_mica(win, None)?;
    }

    #[cfg(target_os = "macos")]
    {
        // 非常好 helper function，不使用 macOS Private APIs
        win.make_transparent()?;
        window_vibrancy::apply_vibrancy(
            win,
            window_vibrancy::NSVisualEffectMaterial::HeaderView,
            Some(window_vibrancy::NSVisualEffectState::Active),
            None,
        )?;
    }

    Ok(())
}
