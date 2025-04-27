use std::{fs, path::PathBuf};

use ollama_rust_api::model::embedding::EmbedRequestParameters;
use tauri::Manager;

use crate::{
    dbaccess::{
        embed::{insert_embedding, new_embedding_table},
        knowledge_base::{
            insert_knowledge_base, select_all_knowledge_base, update_knowledge_base_files,
        },
    },
    models::{
        app_state::AppState,
        knowledge_base::{self, KnowledgeBaseInfo},
    },
    utils::path,
};

#[tauri::command]
pub async fn get_all_knowledge_base(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<KnowledgeBaseInfo>, String> {
    let db = state.db.read().await;
    select_all_knowledge_base(&db)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn new_knowledge_base(
    state: tauri::State<'_, AppState>,
    name: String,
    model: String,
) -> Result<KnowledgeBaseInfo, String> {
    let db = state.db.read().await;
    // 这里如果名字重复会报错返回阻止继续新建
    let new_knowledge_base = insert_knowledge_base(&db, &name, &model)
        .await
        .map_err(|e| e.to_string())?;
    // 再在向量数据库新建
    let embedding_db = state.get_embedding_db().await?;
    new_embedding_table(&embedding_db, &name)
        .await
        .map_err(|e| e.to_string())?;
    let new_knowledge_base_info: KnowledgeBaseInfo = new_knowledge_base.into();
    Ok(new_knowledge_base_info)
}

/// 应该传入拷贝后的文件，返回提取出来的文字（按照1024分割（默认））
fn parse_file(file_path: &PathBuf) -> Result<Vec<String>, String> {
    let extension = file_path.extension();
    let full_text = match extension {
        Some(ext) if ext == "pdf" => {
            let bytes = std::fs::read(file_path).unwrap();
            let output = pdf_extract::extract_text_from_mem(&bytes)
                .map_err(|e| e.to_string())?
                .replace("\n", " ")
                .replace("\r", " ")
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ");
            output
        }
        _ => String::new(),
    };
    let chunk_size = 1024;
    let chunks: Vec<String> = full_text
        .chars()
        .collect::<Vec<_>>()
        .chunks(chunk_size)
        .map(|chunk| chunk.iter().collect())
        .collect();

    Ok(chunks)
}

/// 传过来的 file 应该都是有效的文件
#[tauri::command]
pub async fn add_file_to_path(
    app: tauri::AppHandle,
    knowledge_base: knowledge_base::Model,
    file_paths: Vec<String>,
) -> Result<Vec<String>, String> {
    let state = app.state::<AppState>();
    let db = state.db.read().await;
    let embedding_db = state.get_embedding_db().await?;
    let ollama = state.ollama.read().await;
    let app_data_path = path::get_data_dir(&app).map_err(|e| e.to_string())?;
    let mut file_paths_succeed = vec![];
    let mut embed_file_failed = vec![];
    // 生成嵌入向量并返回添加到向量数据库中
    for file_path in file_paths {
        // 复制文件到 knowledge_base/{knowledge_base.name} 文件夹下
        let file_path = PathBuf::from(file_path);
        let file_name = file_path.file_name().unwrap().to_str().unwrap();
        let copied_file_path = app_data_path.join(format!(
            "knowledge_base/{}/{}",
            &knowledge_base.name, file_name
        ));
        let _ = fs::copy(&file_path, &copied_file_path);
        // 解析该文件
        let chunks = match parse_file(&copied_file_path) {
            Ok(v) => v,
            Err(e) => {
                embed_file_failed.push(format!("{} embed failed: {}", &file_path.display(), e));
                continue;
            }
        };
        // 生成嵌入
        let response = match ollama
            .embedding(&EmbedRequestParameters {
                model: knowledge_base.model.clone(),
                input: chunks.clone(),
            })
            .await
        {
            Ok(v) => v,
            Err(e) => {
                embed_file_failed.push(format!("{} embed failed: {}", &file_path.display(), e));
                continue;
            }
        };
        // 插入生成的嵌入到向量数据库
        let copied_file_path = copied_file_path.to_str().unwrap_or_default();
        for (embedding, content) in response.embeddings.iter().zip(chunks) {
            match insert_embedding(
                &embedding_db,
                &knowledge_base.name,
                &content,
                copied_file_path,
                embedding,
            )
            .await
            {
                Ok(_) => {
                    file_paths_succeed.push(copied_file_path.to_owned());
                }
                Err(e) => {
                    embed_file_failed.push(format!("{} embed failed: {}", &file_path.display(), e));
                }
            }
        }
    }
    // 向知识库更新新增的文件
    let _new_knowledge_base =
        update_knowledge_base_files(&db, &knowledge_base.name, file_paths_succeed)
            .await
            .map_err(|e| e.to_string())?;
    Ok(embed_file_failed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_parse_file() {
        let file_path = PathBuf::from(
            "/Users/cakeal/Downloads/Funding-Guidelines-for-Teaching-Assistantships.pdf",
        );
        let result = parse_file(&file_path).unwrap();
        dbg!(result);
    }
}
