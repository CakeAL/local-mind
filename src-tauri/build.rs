fn main() {
    tauri_build::try_build(
        tauri_build::Attributes::new()
            .codegen(tauri_build::CodegenContext::new())
            .plugin(
                "assistant",
                tauri_build::InlinedPlugin::new().commands(&[
                    "new_assistant",
                    "get_all_assistant",
                    "get_assistant_config",
                    "update_assistant_config",
                    "delete_assistant",
                ]),
            ),
    )
    .expect("Failed to run tauri-build");
}
