//! PDF generation for TANF form population.
//!
//! Creates new PDF documents with the cleaned form template as a background
//! layer and field data overlaid as native PDF text.

use printpdf::*;

use crate::offsets::{Alignment, FormOffsets};

/// RGB color for text rendering.
#[derive(Debug, Clone, Copy)]
pub struct TextColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl TextColor {
    pub const RED: Self = Self { r: 1.0, g: 0.0, b: 0.0 };
    pub const BLUE: Self = Self { r: 0.0, g: 0.0, b: 1.0 };
    pub const MAGENTA: Self = Self { r: 1.0, g: 0.0, b: 1.0 };
    pub const BLACK: Self = Self { r: 0.0, g: 0.0, b: 0.0 };

    fn to_pdf_color(self) -> Color {
        Color::Rgb(Rgb::new(self.r, self.g, self.b, None))
    }
}

/// A text item to place on the form at a specific position.
#[derive(Debug)]
pub struct TextField {
    pub text: String,
    pub x_mm: f32,
    pub y_mm: f32,
    pub font_size: f32,
    pub color: TextColor,
}

/// Form content rotation relative to the PDF page.
///
/// Describes how the form content is oriented in the template image.
/// The tool rotates text and adjusts circle radii to match the form's
/// orientation.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum Rotation {
    /// Form content is rightside-up (no rotation needed).
    #[default]
    Normal,
    /// Form content is rotated 90 degrees counter-clockwise.
    Ccw90,
    /// Form content is rotated 90 degrees clockwise.
    Cw90,
    /// Form content is rotated 180 degrees (upside-down).
    UpsideDown,
}

impl Rotation {
    /// Text rotation angle in degrees for `TextMatrix::TranslateRotate`.
    fn text_angle_degrees(self) -> f32 {
        match self {
            Rotation::Normal => 0.0,
            Rotation::Ccw90 => 90.0,
            Rotation::Cw90 => 270.0,
            Rotation::UpsideDown => 180.0,
        }
    }

    /// Return the form's natural dimensions given the PDF page dimensions.
    ///
    /// For a rightside-up or upside-down form, the form has the same
    /// dimensions as the page. For 90-degree rotations, the form is
    /// landscape (rotated to fit in portrait), so form width equals
    /// page height and form height equals page width.
    pub fn form_dimensions(self, page_w: f32, page_h: f32) -> (f32, f32) {
        match self {
            Rotation::Normal | Rotation::UpsideDown => (page_w, page_h),
            Rotation::Ccw90 | Rotation::Cw90 => (page_h, page_w),
        }
    }

    /// Transform form-space coordinates to page-space coordinates.
    ///
    /// Form-space coordinates (fx, fy) are in millimeters from the top-left
    /// of the rightside-up form. Page-space coordinates (px, py) are in
    /// millimeters from the top-left of the physical PDF page.
    ///
    /// For 90-degree rotations, the form is landscape (form_w = page_h,
    /// form_h = page_w). The transform maps the form's coordinate range
    /// onto the page without non-uniform scaling, producing square grid
    /// cells.
    fn transform_position(self, fx: f32, fy: f32, page_w: f32, page_h: f32) -> (f32, f32) {
        let (form_w, form_h) = self.form_dimensions(page_w, page_h);
        match self {
            Rotation::Normal => (fx, fy),
            Rotation::Ccw90 => (
                fy * page_w / form_h,
                page_h - fx * page_h / form_w,
            ),
            Rotation::Cw90 => (
                page_w - fy * page_w / form_h,
                fx * page_h / form_w,
            ),
            Rotation::UpsideDown => (
                page_w - fx,
                page_h - fy,
            ),
        }
    }
}

/// Configuration for a calibration grid overlay.
///
/// Grid lines are drawn in form-space coordinates and transformed to
/// page-space using the active rotation. Labels show form-space values
/// so they correspond directly to `form_offsets.toml` entries.
#[derive(Debug, Clone)]
pub struct GridConfig {
    /// Grid line interval in millimeters (form-space).
    pub interval_mm: f32,
    /// Grid line and label color.
    pub color: TextColor,
    /// Font size for grid labels in points.
    pub label_font_size: f32,
}

/// Configuration for a diagonal watermark overlay.
///
/// The watermark is rendered on top of all other content as large
/// diagonal text across the page.
#[derive(Debug, Clone)]
pub struct WatermarkConfig {
    /// Watermark text (e.g., "TEST SAMPLE").
    pub text: String,
    /// Text color.
    pub color: TextColor,
    /// Font size in points.
    pub font_size: f32,
}

/// A circle to draw on the form (for "circle one" options).
#[derive(Debug)]
pub struct CircleMark {
    pub center_x_mm: f32,
    pub center_y_mm: f32,
    pub radius_x_mm: f32,
    pub radius_y_mm: f32,
    pub color: TextColor,
}

/// Generate a filled PDF form.
///
/// Creates a new PDF with the template image as background and the provided
/// text fields and circle marks overlaid.
///
/// Coordinates in `TextField` and `CircleMark` are in form space: millimeters
/// from the top-left corner of the rightside-up form. When `rotation` is not
/// `Normal`, this function transforms form-space coordinates to page-space
/// coordinates using a rotation matrix, then converts to PDF coordinates
/// (origin at bottom-left).
///
/// For rotated modes, text is rendered at the appropriate angle and circle
/// radii are swapped for 90-degree rotations so the ellipse aligns with
/// the text direction.
///
/// # Arguments
///
/// * `template_image_bytes` - Raw bytes of the template image (TIFF or PNG).
/// * `template_dpi` - DPI of the template image.
/// * `page_width_mm` - Page width in millimeters.
/// * `page_height_mm` - Page height in millimeters.
/// * `text_fields` - Text items to overlay on the form.
/// * `circle_marks` - Circles to draw on the form.
/// * `rotation` - How the form content is rotated in the template image.
/// * `grid` - Optional grid overlay for calibration.
/// * `watermark` - Optional diagonal watermark text on top of all content.
///
/// # Errors
///
/// Returns an error if image decoding or PDF generation fails.
pub fn generate_form_pdf(
    template_image_bytes: &[u8],
    template_dpi: f32,
    page_width_mm: f32,
    page_height_mm: f32,
    text_fields: &[TextField],
    circle_marks: &[CircleMark],
    rotation: Rotation,
    grid: Option<&GridConfig>,
    watermark: Option<&WatermarkConfig>,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut doc = PdfDocument::new("TANF Job Search Form");
    let mut warnings: Vec<PdfWarnMsg> = Vec::new();

    // Decode template image.
    let raw_image = RawImage::decode_from_bytes(template_image_bytes, &mut warnings)
        .map_err(|e| format!("Failed to decode template image: {e}"))?;
    let image_id = doc.add_image(&raw_image);

    // Build page operations.
    let mut ops: Vec<Op> = Vec::new();

    // Place template image as background.
    ops.push(Op::UseXobject {
        id: image_id.clone(),
        transform: XObjectTransform {
            translate_x: Some(Pt(0.0)),
            translate_y: Some(Pt(0.0)),
            dpi: Some(template_dpi),
            ..Default::default()
        },
    });

    // Use builtin Helvetica font.
    let font_handle = PdfFontHandle::Builtin(BuiltinFont::Helvetica);

    // Draw grid overlay (above background, below text).
    if let Some(grid_cfg) = grid {
        ops.extend(build_grid_ops(
            grid_cfg,
            page_width_mm,
            page_height_mm,
            rotation,
            &font_handle,
        ));
    }

    // Overlay text fields.
    let text_angle = rotation.text_angle_degrees();
    for field in text_fields {
        // Transform form-space coordinates to page-space, then to PDF points.
        let (page_x_mm, page_y_mm) = rotation.transform_position(
            field.x_mm, field.y_mm, page_width_mm, page_height_mm,
        );
        let pdf_x = mm_to_pt(page_x_mm);
        let pdf_y = mm_to_pt(page_height_mm - page_y_mm);

        ops.push(Op::StartTextSection);
        ops.push(Op::SetFillColor { col: field.color.to_pdf_color() });
        ops.push(Op::SetFont { font: font_handle.clone(), size: Pt(field.font_size) });
        if rotation == Rotation::Normal {
            ops.push(Op::SetTextCursor {
                pos: Point {
                    x: Pt(pdf_x),
                    y: Pt(pdf_y),
                },
            });
        } else {
            ops.push(Op::SetTextMatrix {
                matrix: TextMatrix::TranslateRotate(
                    Pt(pdf_x),
                    Pt(pdf_y),
                    text_angle,
                ),
            });
        }
        ops.push(Op::ShowText {
            items: vec![TextItem::Text(field.text.clone())],
        });
        ops.push(Op::EndTextSection);
    }

    // Draw circle marks.
    // Swap ellipse radii for 90-degree rotations so the major axis
    // aligns with the rotated text direction.
    for circle in circle_marks {
        // Transform form-space center to page-space, then to PDF points.
        let (page_cx_mm, page_cy_mm) = rotation.transform_position(
            circle.center_x_mm, circle.center_y_mm, page_width_mm, page_height_mm,
        );
        let cx = mm_to_pt(page_cx_mm);
        let cy = mm_to_pt(page_height_mm - page_cy_mm);
        let (rx_mm, ry_mm) = match rotation {
            Rotation::Ccw90 | Rotation::Cw90 => (circle.radius_y_mm, circle.radius_x_mm),
            Rotation::Normal | Rotation::UpsideDown => {
                (circle.radius_x_mm, circle.radius_y_mm)
            }
        };
        let rx = mm_to_pt(rx_mm);
        let ry = mm_to_pt(ry_mm);

        // Approximate an ellipse with 4 bezier curves.
        let kappa: f32 = 0.5522848;
        let ox = rx * kappa;
        let oy = ry * kappa;

        let line = Line {
            points: vec![
                LinePoint { p: Point { x: Pt(cx + rx), y: Pt(cy) }, bezier: false },
                LinePoint { p: Point { x: Pt(cx + rx), y: Pt(cy + oy) }, bezier: true },
                LinePoint { p: Point { x: Pt(cx + ox), y: Pt(cy + ry) }, bezier: true },
                LinePoint { p: Point { x: Pt(cx), y: Pt(cy + ry) }, bezier: false },
                LinePoint { p: Point { x: Pt(cx - ox), y: Pt(cy + ry) }, bezier: true },
                LinePoint { p: Point { x: Pt(cx - rx), y: Pt(cy + oy) }, bezier: true },
                LinePoint { p: Point { x: Pt(cx - rx), y: Pt(cy) }, bezier: false },
                LinePoint { p: Point { x: Pt(cx - rx), y: Pt(cy - oy) }, bezier: true },
                LinePoint { p: Point { x: Pt(cx - ox), y: Pt(cy - ry) }, bezier: true },
                LinePoint { p: Point { x: Pt(cx), y: Pt(cy - ry) }, bezier: false },
                LinePoint { p: Point { x: Pt(cx + ox), y: Pt(cy - ry) }, bezier: true },
                LinePoint { p: Point { x: Pt(cx + rx), y: Pt(cy - oy) }, bezier: true },
                LinePoint { p: Point { x: Pt(cx + rx), y: Pt(cy) }, bezier: false },
            ],
            is_closed: true,
        };

        ops.push(Op::SetOutlineColor { col: circle.color.to_pdf_color() });
        ops.push(Op::SetOutlineThickness { pt: Pt(1.0) });
        ops.push(Op::DrawLine { line });
    }

    // Draw watermark on top of all other content.
    if let Some(wm) = watermark {
        let center_x = mm_to_pt(page_width_mm / 2.0);
        let center_y = mm_to_pt(page_height_mm / 2.0);
        // Estimate text width to offset the starting position so
        // the text is roughly centered on the page.
        let text_width_pt = wm.text.len() as f32 * wm.font_size * 0.5;
        let angle: f32 = 45.0;
        let angle_rad = angle.to_radians();
        let start_x = center_x - (text_width_pt / 2.0) * angle_rad.cos();
        let start_y = center_y - (text_width_pt / 2.0) * angle_rad.sin();

        ops.push(Op::StartTextSection);
        ops.push(Op::SetFillColor { col: wm.color.to_pdf_color() });
        ops.push(Op::SetFont { font: font_handle.clone(), size: Pt(wm.font_size) });
        ops.push(Op::SetTextMatrix {
            matrix: TextMatrix::TranslateRotate(
                Pt(start_x),
                Pt(start_y),
                angle,
            ),
        });
        ops.push(Op::ShowText {
            items: vec![TextItem::Text(wm.text.clone())],
        });
        ops.push(Op::EndTextSection);
    }

    let page = PdfPage::new(Mm(page_width_mm), Mm(page_height_mm), ops);
    let pdf_bytes = doc.with_pages(vec![page]).save(&PdfSaveOptions::default(), &mut warnings);
    Ok(pdf_bytes)
}

/// Build PDF operations for a form-space calibration grid.
///
/// Draws grid lines at regular intervals in form-space coordinates,
/// transformed to page-space using the active rotation. Each line is
/// labeled with its form-space coordinate value so the human can read
/// positions directly for `form_offsets.toml` editing.
///
/// Form-X lines (constant X, varying Y) label with "x{value}".
/// Form-Y lines (constant Y, varying X) label with "y{value}".
fn build_grid_ops(
    config: &GridConfig,
    page_w: f32,
    page_h: f32,
    rotation: Rotation,
    font: &PdfFontHandle,
) -> Vec<Op> {
    let mut ops = Vec::new();
    let color = config.color.to_pdf_color();
    let interval = config.interval_mm;

    // Form dimensions differ from page dimensions for 90-degree rotations.
    let (form_w, form_h) = rotation.form_dimensions(page_w, page_h);

    // Form-X grid lines: vertical lines in form-space (fx = constant).
    // For each fx value, draw a line from (fx, 0) to (fx, form_h) in form-space.
    let mut fx = 0.0_f32;
    while fx <= form_w {
        let (sx, sy) = rotation.transform_position(fx, 0.0, page_w, page_h);
        let (ex, ey) = rotation.transform_position(fx, form_h, page_w, page_h);

        let pdf_sx = mm_to_pt(sx);
        let pdf_sy = mm_to_pt(page_h - sy);
        let pdf_ex = mm_to_pt(ex);
        let pdf_ey = mm_to_pt(page_h - ey);

        ops.push(Op::SetOutlineColor { col: color.clone() });
        ops.push(Op::SetOutlineThickness { pt: Pt(0.3) });
        ops.push(Op::DrawLine {
            line: Line {
                points: vec![
                    LinePoint { p: Point { x: Pt(pdf_sx), y: Pt(pdf_sy) }, bezier: false },
                    LinePoint { p: Point { x: Pt(pdf_ex), y: Pt(pdf_ey) }, bezier: false },
                ],
                is_closed: false,
            },
        });

        // Label at the start of the line.
        let label = format!("x{}", fx as u32);
        ops.push(Op::StartTextSection);
        ops.push(Op::SetFillColor { col: color.clone() });
        ops.push(Op::SetFont { font: font.clone(), size: Pt(config.label_font_size) });
        ops.push(Op::SetTextCursor {
            pos: Point { x: Pt(pdf_sx + 1.0), y: Pt(pdf_sy - 1.0) },
        });
        ops.push(Op::ShowText { items: vec![TextItem::Text(label)] });
        ops.push(Op::EndTextSection);

        fx += interval;
    }

    // Form-Y grid lines: horizontal lines in form-space (fy = constant).
    // For each fy value, draw a line from (0, fy) to (form_w, fy) in form-space.
    let mut fy = 0.0_f32;
    while fy <= form_h {
        let (sx, sy) = rotation.transform_position(0.0, fy, page_w, page_h);
        let (ex, ey) = rotation.transform_position(form_w, fy, page_w, page_h);

        let pdf_sx = mm_to_pt(sx);
        let pdf_sy = mm_to_pt(page_h - sy);
        let pdf_ex = mm_to_pt(ex);
        let pdf_ey = mm_to_pt(page_h - ey);

        ops.push(Op::SetOutlineColor { col: color.clone() });
        ops.push(Op::SetOutlineThickness { pt: Pt(0.3) });
        ops.push(Op::DrawLine {
            line: Line {
                points: vec![
                    LinePoint { p: Point { x: Pt(pdf_sx), y: Pt(pdf_sy) }, bezier: false },
                    LinePoint { p: Point { x: Pt(pdf_ex), y: Pt(pdf_ey) }, bezier: false },
                ],
                is_closed: false,
            },
        });

        // Label at the start of the line.
        let label = format!("y{}", fy as u32);
        ops.push(Op::StartTextSection);
        ops.push(Op::SetFillColor { col: color.clone() });
        ops.push(Op::SetFont { font: font.clone(), size: Pt(config.label_font_size) });
        ops.push(Op::SetTextCursor {
            pos: Point { x: Pt(pdf_sx + 1.0), y: Pt(pdf_sy - 1.0) },
        });
        ops.push(Op::ShowText { items: vec![TextItem::Text(label)] });
        ops.push(Op::EndTextSection);

        fy += interval;
    }

    ops
}

/// Convert millimeters to PDF points.
fn mm_to_pt(mm: f32) -> f32 {
    mm * 72.0 / 25.4
}

/// Build text fields and circle marks from form offsets and field values
/// for a calibration PDF with colored text.
///
/// Non-job-search fields use `header_color`. Job search table rows alternate
/// between `row_color_a` and `row_color_b`. All circle-one options are circled.
///
/// When `show_circle_labels` is false, text labels for circle-one options
/// are omitted. The ellipses are still drawn.
///
/// When `circle_all` is true, all circle-one options get ellipses. When
/// false, only the default selected option in each series is circled
/// (pay frequency: Weekly, insurance: None).
///
/// When `fill_table` is true, job search table rows are populated with
/// hardcoded sample data. When false, the table is left empty.
///
/// Fields with no matching value in the `values` map are skipped entirely.
pub fn build_calibration_fields(
    offsets: &FormOffsets,
    values: &std::collections::HashMap<String, String>,
    header_color: TextColor,
    row_color_a: TextColor,
    row_color_b: TextColor,
    show_circle_labels: bool,
    circle_all: bool,
    fill_table: bool,
) -> (Vec<TextField>, Vec<CircleMark>) {
    let mut text_fields = Vec::new();
    let mut circle_marks = Vec::new();

    // Collect and sort field names for deterministic output.
    let mut field_names: Vec<&String> = offsets.fields.keys().collect();
    field_names.sort();

    // Place named fields that have values. Fields with no matching
    // value in the map are skipped (not rendered).
    for name in field_names {
        let offset = &offsets.fields[name];
        let text = match values.get(name.as_str()) {
            Some(v) => v.clone(),
            None => continue,
        };

        if is_circle_option(name) {
            let draw_ellipse = circle_all || is_default_selected(name);

            if draw_ellipse {
                // Draw ellipse around the option text.
                let rx = estimate_text_width(&text, offset.font_size) / 2.0 + 1.5;
                let ry = offset.font_size * 0.0353 * 10.0 / 2.0 + 0.5;
                circle_marks.push(CircleMark {
                    center_x_mm: offset.x + rx - 1.5,
                    center_y_mm: offset.y,
                    radius_x_mm: rx,
                    radius_y_mm: ry,
                    color: header_color,
                });
            }

            // Skip label text when labels are disabled.
            if !show_circle_labels {
                continue;
            }
        }

        // Align text within the field width.
        let x_mm = align_in_width(offset.x, offset.width, &text, offset.font_size, offset.alignment);

        text_fields.push(TextField {
            text,
            x_mm,
            y_mm: offset.y,
            font_size: offset.font_size,
            color: header_color,
        });
    }

    // Place job search table rows (hardcoded sample data, debug only).
    if fill_table {
        let mut col_names: Vec<&String> = offsets.table.columns.keys().collect();
        col_names.sort();

        for row in 0..offsets.table.row_count {
            let y = offsets.table.first_row_y + (row as f32) * offsets.table.row_height;
            let color = if row % 2 == 0 { row_color_a } else { row_color_b };

            for col_name in &col_names {
                if col_name.as_str() == "office_use_only" {
                    continue;
                }

                let col = &offsets.table.columns[col_name.as_str()];
                let text = sample_table_text(col_name, row + 1);
                let x_mm = align_in_width(col.x, col.width, &text, col.font_size, col.alignment);
                text_fields.push(TextField {
                    text,
                    x_mm,
                    y_mm: y,
                    font_size: col.font_size,
                    color,
                });
            }
        }
    }

    (text_fields, circle_marks)
}

/// Check if a field name is a circle-one option.
fn is_circle_option(name: &str) -> bool {
    name.starts_with("employed_pay_frequency_")
        || name.starts_with("employed_insurance_")
}

/// Check if a circle-one option is the default selected in its series.
///
/// Used when `circle_all` is false to circle only one option per series.
fn is_default_selected(name: &str) -> bool {
    name == "employed_pay_frequency_weekly"
        || name == "employed_insurance_none"
}

/// Compute an aligned x position within a field width.
///
/// For `Center`, offsets x by half the remaining space. For `Right`,
/// offsets x by the full remaining space. For `Left`, returns x
/// unchanged. Falls back to left-aligned when the text is wider than
/// the field or when no width is specified.
fn align_in_width(
    x: f32,
    width: Option<f32>,
    text: &str,
    font_size: f32,
    alignment: Alignment,
) -> f32 {
    if let Some(w) = width {
        let text_w = estimate_text_width(text, font_size);
        if text_w < w {
            return match alignment {
                Alignment::Left => x,
                Alignment::Center => x + (w - text_w) / 2.0,
                Alignment::Right => x + w - text_w,
            };
        }
    }
    x
}

/// Estimate text width in mm given text and font size in points.
/// Rough approximation for Helvetica (average character width ~0.5em).
fn estimate_text_width(text: &str, font_size: f32) -> f32 {
    let avg_char_width_pt = font_size * 0.5;
    let width_pt = text.len() as f32 * avg_char_width_pt;
    width_pt * 25.4 / 72.0
}

/// Generate sample text for a table column and row number.
fn sample_table_text(column: &str, row: u32) -> String {
    match column {
        "date" => format!("01/{:02}/2026", row),
        "employer_name_address" => format!("Company {row}, 123 Main St"),
        "how_contact_made" => "Online".to_string(),
        "telephone_fax" => "T".to_string(),
        "telephone_number" => format!("702-555-{row:04}"),
        "internet_confirmation" => "Yes".to_string(),
        "time_in" => "9:00".to_string(),
        "time_out" => "9:30".to_string(),
        _ => format!("R{row}"),
    }
}
