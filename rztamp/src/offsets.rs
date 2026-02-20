//! Form field offset types for TANF form layout.
//!
//! These types deserialize from `assets/form/form_offsets.toml`.
//! Coordinates are in millimeters from the top-left corner of the page.

use serde::Deserialize;
use std::collections::HashMap;

/// Top-level form offsets configuration.
#[derive(Debug, Deserialize)]
pub struct FormOffsets {
    pub meta: Meta,
    pub fields: HashMap<String, FieldOffset>,
    pub table: TableConfig,
}

/// Page and template metadata.
#[derive(Debug, Deserialize)]
pub struct Meta {
    pub page_width_mm: f32,
    pub page_height_mm: f32,
    pub template_image: String,
    pub template_width_px: u32,
    pub template_height_px: u32,
    pub template_dpi: u32,
}

/// Text alignment within a field's declared width.
#[derive(Debug, Clone, Copy, Default, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Alignment {
    #[default]
    Left,
    Center,
    Right,
}

/// A single field's position on the form.
#[derive(Debug, Deserialize)]
pub struct FieldOffset {
    pub x: f32,
    pub y: f32,
    #[serde(default)]
    pub width: Option<f32>,
    #[serde(default = "default_font_size")]
    pub font_size: f32,
    #[serde(default)]
    pub alignment: Alignment,
}

/// Job search table configuration.
#[derive(Debug, Deserialize)]
pub struct TableConfig {
    pub first_row_y: f32,
    pub row_height: f32,
    pub row_count: u32,
    pub columns: HashMap<String, ColumnOffset>,
}

/// A table column's position.
#[derive(Debug, Deserialize)]
pub struct ColumnOffset {
    pub x: f32,
    #[serde(default)]
    pub width: Option<f32>,
    #[serde(default = "default_font_size")]
    pub font_size: f32,
    #[serde(default)]
    pub alignment: Alignment,
}

fn default_font_size() -> f32 {
    10.0
}

/// Load form offsets from a TOML file.
///
/// # Errors
///
/// Returns an error if the file cannot be read or parsed.
pub fn load_offsets(path: &std::path::Path) -> Result<FormOffsets, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;
    let offsets: FormOffsets = toml::from_str(&content)?;
    Ok(offsets)
}
