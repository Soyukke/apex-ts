use anyhow::{Context, Result};
use regex::Regex;
use std::fs;
use tracing::{debug, warn};

#[derive(Debug, Clone)]
pub struct ApexClass {
    pub name: String,
    pub fields: Vec<ApexField>,
    pub methods: Vec<ApexMethod>,
}

#[derive(Debug, Clone)]
pub struct ApexField {
    pub name: String,
    pub field_type: String,
    pub is_optional: bool,
}

#[derive(Debug, Clone)]
pub struct ApexMethod {
    pub name: String,
    pub return_type: String,
    pub parameters: Vec<ApexParameter>,
    pub is_static: bool,
}

#[derive(Debug, Clone)]
pub struct ApexParameter {
    pub name: String,
    pub param_type: String,
}

pub struct ApexParser {
    class_regex: Regex,
    field_with_line_regex: Regex,
    method_regex: Regex,
    annotation_regex: Regex,
    aura_enabled_regex: Regex,
}

impl ApexParser {
    pub fn new() -> Result<Self> {
        Ok(Self {
            // /** ... @tsexport ... */ 複数行ドキュメントコメント内のどこかに@tsexportがあればOK
            annotation_regex: Regex::new(r"/\*\*[\s\S]*?@tsexport[\s\S]*?\*/")?,
            // @AuraEnabled または @AuraEnabled(パラメータ) を検出
            aura_enabled_regex: Regex::new(r"@AuraEnabled(?:\([^)]*\))?")?,
            // public class ClassName の形式を検出
            class_regex: Regex::new(r"(?m)^\s*public\s+class\s+(\w+)")?,
            // フィールド定義を検出（複数行アノテーションとワンライン形式の両方に対応）
            // キャプチャ: 1=直前の行のアノテーション, 2=同じ行のアノテーション(optional), 3=型, 4=フィールド名
            field_with_line_regex: Regex::new(
                r"(?m)((?:^\s*(?:/\*\*[\s\S]*?\*/|@\w+(?:\([^)]*\))?)\s*\n)*)\s*(@\w+(?:\([^)]*\))?\s+)?public\s+(\w+(?:<[\w\s,]+>)?)\s+(\w+)\s*;"
            )?,
            // メソッド定義を検出（複数行アノテーションとワンライン形式の両方に対応）
            method_regex: Regex::new(
                r"(?m)((?:^\s*(?:/\*\*[\s\S]*?\*/|@\w+(?:\([^)]*\))?)\s*\n)*)\s*(@\w+(?:\([^)]*\))?\s+)?public\s+(static\s+)?(\w+(?:<[\w\s,]+>)?)\s+(\w+)\s*\(([^)]*)\)"
            )?,
        })
    }

    pub fn parse_file(&self, content: &str) -> Result<Option<ApexClass>> {
        // @tsexport アノテーションがあるかチェック
        if !self.annotation_regex.is_match(content) {
            return Ok(None);
        }

        // クラス名を取得
        let class_name = self
            .class_regex
            .captures(content)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str().to_string())
            .context("Failed to find class name")?;

        debug!("Parsing class: {}", class_name);

        // フィールドを解析
        let mut fields = Vec::new();
        for cap in self.field_with_line_regex.captures_iter(content) {
            let prev_line_annotations = cap.get(1).unwrap().as_str();
            let same_line_annotation = cap.get(2).map(|m| m.as_str()).unwrap_or("");
            let field_type = cap.get(3).unwrap().as_str().to_string();
            let field_name = cap.get(4).unwrap().as_str().to_string();
            
            // 直前の行または同じ行に @AuraEnabled があるかチェック
            let has_aura_enabled = self.aura_enabled_regex.is_match(prev_line_annotations) 
                || self.aura_enabled_regex.is_match(same_line_annotation);
            
            if has_aura_enabled {
                debug!("  Field: {} ({})", field_name, field_type);
                fields.push(ApexField {
                    name: field_name,
                    field_type,
                    is_optional: false,
                });
            } else {
                warn!(
                    "  Skipping field '{}' in class '{}' (missing @AuraEnabled)",
                    field_name, class_name
                );
            }
        }

        // メソッドを解析
        let mut methods = Vec::new();
        for cap in self.method_regex.captures_iter(content) {
            let prev_line_annotations = cap.get(1).unwrap().as_str();
            let same_line_annotation = cap.get(2).map(|m| m.as_str()).unwrap_or("");
            let is_static = cap.get(3).is_some();
            let return_type = cap.get(4).unwrap().as_str().to_string();
            let method_name = cap.get(5).unwrap().as_str().to_string();
            let params_str = cap.get(6).unwrap().as_str();

            // 直前の行または同じ行に @AuraEnabled があるかチェック
            let has_aura_enabled = self.aura_enabled_regex.is_match(prev_line_annotations) 
                || self.aura_enabled_regex.is_match(same_line_annotation);

            if has_aura_enabled {
                let parameters = self.parse_parameters(params_str);
                debug!(
                    "  Method: {} ({}) -> {}",
                    method_name,
                    params_str,
                    return_type
                );
                
                methods.push(ApexMethod {
                    name: method_name,
                    return_type,
                    parameters,
                    is_static,
                });
            } else {
                warn!(
                    "  Skipping method '{}' in class '{}' (missing @AuraEnabled)",
                    method_name, class_name
                );
            }
        }

        Ok(Some(ApexClass {
            name: class_name,
            fields,
            methods,
        }))
    }

    fn parse_parameters(&self, params_str: &str) -> Vec<ApexParameter> {
        if params_str.trim().is_empty() {
            return Vec::new();
        }

        params_str
            .split(',')
            .filter_map(|param| {
                let parts: Vec<&str> = param.trim().split_whitespace().collect();
                if parts.len() >= 2 {
                    Some(ApexParameter {
                        param_type: parts[0].to_string(),
                        name: parts[1].to_string(),
                    })
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn parse_files(&self, paths: &[String]) -> Result<Vec<ApexClass>> {
        let mut classes = Vec::new();

        for path in paths {
            let content = fs::read_to_string(path)
                .with_context(|| format!("Failed to read file: {}", path))?;

            if let Some(class) = self.parse_file(&content)? {
                classes.push(class);
            }
        }

        Ok(classes)
    }
}

impl Default for ApexParser {
    fn default() -> Self {
        Self::new().unwrap()
    }
}
