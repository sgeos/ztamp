//! PDF generation for TANF form population.
//!
//! Creates new PDF documents with the cleaned form template as a background
//! layer and field data overlaid as native PDF text.

use printpdf::*;

use crate::offsets::FormOffsets;

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
/// Coordinates in `TextField` and `CircleMark` are in millimeters from the
/// top-left corner of the page. This function converts them to PDF coordinates
/// (origin at bottom-left).
///
/// When `rotation` is not `Normal`, text is rendered at the specified angle
/// to match form content that is rotated within the template image. Circle
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

    // Overlay text fields.
    let text_angle = rotation.text_angle_degrees();
    for field in text_fields {
        let pdf_x = mm_to_pt(field.x_mm);
        let pdf_y = mm_to_pt(page_height_mm - field.y_mm);

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
        let cx = mm_to_pt(circle.center_x_mm);
        let cy = mm_to_pt(page_height_mm - circle.center_y_mm);
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

    let page = PdfPage::new(Mm(page_width_mm), Mm(page_height_mm), ops);
    let pdf_bytes = doc.with_pages(vec![page]).save(&PdfSaveOptions::default(), &mut warnings);
    Ok(pdf_bytes)
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
pub fn build_calibration_fields(
    offsets: &FormOffsets,
    values: &std::collections::HashMap<String, String>,
    header_color: TextColor,
    row_color_a: TextColor,
    row_color_b: TextColor,
) -> (Vec<TextField>, Vec<CircleMark>) {
    let mut text_fields = Vec::new();
    let mut circle_marks = Vec::new();

    // Collect and sort field names for deterministic output.
    let mut field_names: Vec<&String> = offsets.fields.keys().collect();
    field_names.sort();

    // Place all named fields.
    for name in field_names {
        let offset = &offsets.fields[name];
        let text = values.get(name.as_str())
            .cloned()
            .unwrap_or_else(|| name.to_uppercase());

        if is_circle_option(name) {
            // Circle options: place text and draw circle around it.
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

        text_fields.push(TextField {
            text,
            x_mm: offset.x,
            y_mm: offset.y,
            font_size: offset.font_size,
            color: header_color,
        });
    }

    // Place job search table rows.
    // Collect and sort column names for deterministic output.
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
            text_fields.push(TextField {
                text,
                x_mm: col.x,
                y_mm: y,
                font_size: col.font_size,
                color,
            });
        }
    }

    (text_fields, circle_marks)
}

/// Check if a field name is a circle-one option.
fn is_circle_option(name: &str) -> bool {
    name.starts_with("employed_pay_frequency_")
        || name.starts_with("employed_insurance_")
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
        "date" => format!("01/{:02}/26", row),
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
