use dotext::*;
use std::{io::Read, path::PathBuf};

/// 应该传入拷贝后的文件，返回提取出来的文字（按照1024分割（默认））
pub fn parse_file(file_path: &PathBuf) -> Result<Vec<String>, String> {
    let extension = file_path.extension();
    let full_text = match extension {
        Some(ext) if ext == "pdf" => parse_pdf(file_path)?,
        Some(ext) if ext == "docx" => parse_docx(file_path)?,
        Some(ext) if ext == "pptx" => parse_pptx(file_path)?,
        Some(ext) if ext == "xlsx" => parse_xlsx(file_path)?,
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
fn parse_pdf(file_path: &PathBuf) -> Result<String, String> {
    let bytes = std::fs::read(file_path).unwrap();
    let output = pdf_extract::extract_text_from_mem(&bytes)
        .map_err(|e| e.to_string())?
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    Ok(output)
}

fn parse_docx(file_path: &PathBuf) -> Result<String, String> {
    let mut file = Docx::open(file_path).map_err(|e| e.to_string())?;
    let mut isi = String::new();
    let _ = file.read_to_string(&mut isi);
    let output = isi.split_whitespace().collect::<Vec<_>>().join(" ");
    Ok(output)
}

fn parse_pptx(file_path: &PathBuf) -> Result<String, String> {
    let mut file = Pptx::open(file_path).map_err(|e| e.to_string())?;
    let mut isi = String::new();
    let _ = file.read_to_string(&mut isi);
    let output = isi.split_whitespace().collect::<Vec<_>>().join(" ");
    Ok(output)
}

fn parse_xlsx(file_path: &PathBuf) -> Result<String, String> {
    let mut file = Xlsx::open(file_path).map_err(|e| e.to_string())?;
    let mut isi = String::new();
    let _ = file.read_to_string(&mut isi);
    let output = isi.split_whitespace().collect::<Vec<_>>().join("");
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_pdf() {
        let file_path = PathBuf::from("/Users/cakeal/Downloads/北科大学生手册.pdf");
        let result = parse_pdf(&file_path);
        dbg!(result.unwrap());
    }

    #[test]
    fn test_parse_docx() {
        let file_path =
            PathBuf::from("/Users/cakeal/Downloads/软件工程计算机+Java图书管理系统开发与设计.docx");
        let result = parse_docx(&file_path);
        dbg!(result.unwrap());
    }

    #[test]
    fn test_parse_pptx() {
        let file_path = PathBuf::from("/Users/cakeal/Downloads/吉林医药学院.pptx");
        let result = parse_pptx(&file_path);
        dbg!(result.unwrap());
    }

    #[test]
    fn test_parse_xlsx() {
        let file_path = PathBuf::from("/Users/cakeal/Downloads/24年上半年课表.xlsx");
        let result = parse_xlsx(&file_path);
        dbg!(result.unwrap());
    }
}
