//! tanf-fill: CLI tool for generating filled TANF job search PDF forms.
//!
//! Reads form field offsets and secret values, then generates a PDF with
//! the form template as background and field data overlaid as colored text.
//!
//! Usage:
//!     tanf-fill --offsets <path> --secrets <path> --template <path> --output <path>
//!              [--rotation <mode>] [--grid <interval_mm>] [--grid-color <color>]
//!
//! Rotation modes: rightside-up (default), counter-clockwise, clockwise, upside-down
//! Grid colors: green (default), gray, red, blue, black, magenta, cyan
//!
//! For calibration mode, uses colored text:
//! - Red for non-job-search fields
//! - Alternating blue and magenta for job search table rows
//! - Circles drawn around all circle-one options

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;

use serde::Deserialize;

use rztamp::offsets;
use rztamp::pdf::{self, GridConfig, Rotation, TextColor};

/// Secrets file structure.
#[derive(Debug, Deserialize)]
struct Secrets {
    form: FormSecrets,
    participant: ParticipantSecrets,
    job_search: JobSearchSecrets,
    submission: SubmissionSecrets,
}

#[derive(Debug, Deserialize)]
struct FormSecrets {
    #[allow(dead_code)]
    base_form_path: String,
}

#[derive(Debug, Deserialize)]
struct ParticipantSecrets {
    case_name: String,
    upi_number: String,
}

#[derive(Debug, Deserialize)]
struct JobSearchSecrets {
    date_from: String,
    date_to: String,
    hours: u32,
}

#[derive(Debug, Deserialize)]
struct SubmissionSecrets {
    deadline_time: String,
    deadline_date: String,
    location: String,
}

/// Parsed command line arguments.
struct Args {
    offsets_path: PathBuf,
    secrets_path: PathBuf,
    template_path: PathBuf,
    output_path: PathBuf,
    rotation: Rotation,
    grid_interval: Option<f32>,
    grid_color: TextColor,
}

fn parse_args() -> Args {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 9 {
        eprintln!(
            "Usage: tanf-fill --offsets <path> --secrets <path> \
             --template <path> --output <path> [--rotation <mode>] \
             [--grid <interval_mm>] [--grid-color <color>]"
        );
        eprintln!("  Rotation modes: rightside-up (default), counter-clockwise, clockwise, upside-down");
        eprintln!("  Grid colors: green (default), gray, red, blue, black, magenta, cyan");
        process::exit(1);
    }

    let mut offsets_path = None;
    let mut secrets_path = None;
    let mut template_path = None;
    let mut output_path = None;
    let mut rotation = Rotation::Normal;
    let mut grid_interval: Option<f32> = None;
    let mut grid_color = TextColor { r: 0.0, g: 0.6, b: 0.0 }; // default green

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--offsets" => {
                i += 1;
                offsets_path = Some(PathBuf::from(&args[i]));
            }
            "--secrets" => {
                i += 1;
                secrets_path = Some(PathBuf::from(&args[i]));
            }
            "--template" => {
                i += 1;
                template_path = Some(PathBuf::from(&args[i]));
            }
            "--output" => {
                i += 1;
                output_path = Some(PathBuf::from(&args[i]));
            }
            "--rotation" => {
                i += 1;
                rotation = match args[i].as_str() {
                    "rightside-up" => Rotation::Normal,
                    "counter-clockwise" => Rotation::Ccw90,
                    "clockwise" => Rotation::Cw90,
                    "upside-down" => Rotation::UpsideDown,
                    other => {
                        eprintln!("Unknown rotation mode: {other}");
                        eprintln!("Valid modes: rightside-up, counter-clockwise, clockwise, upside-down");
                        process::exit(1);
                    }
                };
            }
            "--grid" => {
                i += 1;
                grid_interval = Some(args[i].parse::<f32>().unwrap_or_else(|_| {
                    eprintln!("Invalid grid interval: {}", args[i]);
                    process::exit(1);
                }));
            }
            "--grid-color" => {
                i += 1;
                grid_color = parse_color(&args[i]).unwrap_or_else(|| {
                    eprintln!("Unknown grid color: {}", args[i]);
                    eprintln!("Valid colors: green, gray, red, blue, black, magenta, cyan");
                    process::exit(1);
                });
            }
            other => {
                eprintln!("Unknown argument: {other}");
                process::exit(1);
            }
        }
        i += 1;
    }

    let missing = [
        offsets_path.is_none().then_some("--offsets"),
        secrets_path.is_none().then_some("--secrets"),
        template_path.is_none().then_some("--template"),
        output_path.is_none().then_some("--output"),
    ];

    let missing: Vec<&str> = missing.iter().filter_map(|m| *m).collect();
    if !missing.is_empty() {
        eprintln!("Missing required arguments: {}", missing.join(", "));
        process::exit(1);
    }

    Args {
        offsets_path: offsets_path.unwrap(),
        secrets_path: secrets_path.unwrap(),
        template_path: template_path.unwrap(),
        output_path: output_path.unwrap(),
        rotation,
        grid_interval,
        grid_color,
    }
}

/// Parse a named color string to a TextColor.
fn parse_color(name: &str) -> Option<TextColor> {
    match name {
        "green" => Some(TextColor { r: 0.0, g: 0.6, b: 0.0 }),
        "gray" | "grey" => Some(TextColor { r: 0.5, g: 0.5, b: 0.5 }),
        "red" => Some(TextColor::RED),
        "blue" => Some(TextColor::BLUE),
        "black" => Some(TextColor::BLACK),
        "magenta" => Some(TextColor::MAGENTA),
        "cyan" => Some(TextColor { r: 0.0, g: 0.7, b: 0.7 }),
        _ => None,
    }
}

fn main() {
    let args = parse_args();

    // Validate output does not overwrite existing file.
    if args.output_path.exists() {
        eprintln!(
            "Error: output file already exists: {}",
            args.output_path.display()
        );
        eprintln!("Refusing to overwrite. Choose a different output path.");
        process::exit(1);
    }

    // Load form offsets.
    let form_offsets = offsets::load_offsets(&args.offsets_path)
        .unwrap_or_else(|e| {
            eprintln!("Failed to load offsets from {}: {e}", args.offsets_path.display());
            process::exit(1);
        });

    // Load secrets.
    let secrets = load_secrets(&args.secrets_path)
        .unwrap_or_else(|e| {
            eprintln!("Failed to load secrets from {}: {e}", args.secrets_path.display());
            process::exit(1);
        });

    // Build field value map from secrets.
    let values = build_value_map(&secrets);

    // Load template image.
    let template_bytes = fs::read(&args.template_path)
        .unwrap_or_else(|e| {
            eprintln!("Failed to read template image {}: {e}", args.template_path.display());
            process::exit(1);
        });

    // Build calibration fields with colored text.
    let (text_fields, circle_marks) = pdf::build_calibration_fields(
        &form_offsets,
        &values,
        TextColor::RED,
        TextColor::BLUE,
        TextColor::MAGENTA,
    );

    let rotation_label = match args.rotation {
        Rotation::Normal => "rightside-up",
        Rotation::Ccw90 => "counter-clockwise (90 degrees)",
        Rotation::Cw90 => "clockwise (90 degrees)",
        Rotation::UpsideDown => "upside-down (180 degrees)",
    };

    // Build optional grid config.
    let grid_config = args.grid_interval.map(|interval| {
        eprintln!("Grid enabled: {}mm interval", interval);
        GridConfig {
            interval_mm: interval,
            color: args.grid_color,
            label_font_size: 5.0,
        }
    });

    eprintln!(
        "Generating PDF with {} text fields, {} circle marks, rotation: {}...",
        text_fields.len(), circle_marks.len(), rotation_label
    );

    // Generate PDF.
    let pdf_bytes = pdf::generate_form_pdf(
        &template_bytes,
        form_offsets.meta.template_dpi as f32,
        form_offsets.meta.page_width_mm,
        form_offsets.meta.page_height_mm,
        &text_fields,
        &circle_marks,
        args.rotation,
        grid_config.as_ref(),
    ).unwrap_or_else(|e| {
        eprintln!("Failed to generate PDF: {e}");
        process::exit(1);
    });

    // Write output.
    fs::write(&args.output_path, &pdf_bytes)
        .unwrap_or_else(|e| {
            eprintln!("Failed to write output to {}: {e}", args.output_path.display());
            process::exit(1);
        });

    eprintln!("Output written to: {}", args.output_path.display());
    eprintln!("PDF size: {} bytes", pdf_bytes.len());
}

fn load_secrets(path: &Path) -> Result<Secrets, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let secrets: Secrets = toml::from_str(&content)?;
    Ok(secrets)
}

/// Map secrets to field names matching form_offsets.toml field keys.
fn build_value_map(secrets: &Secrets) -> HashMap<String, String> {
    let mut map = HashMap::new();

    // Participant fields.
    map.insert("case_name".to_string(), secrets.participant.case_name.clone());
    map.insert("upi_number".to_string(), secrets.participant.upi_number.clone());

    // Job search fields.
    map.insert("job_search_from".to_string(), secrets.job_search.date_from.clone());
    map.insert("job_search_to".to_string(), secrets.job_search.date_to.clone());
    map.insert("job_search_hours".to_string(), secrets.job_search.hours.to_string());

    // Submission fields.
    map.insert("submission_deadline_time".to_string(), secrets.submission.deadline_time.clone());
    map.insert("submission_deadline_date".to_string(), secrets.submission.deadline_date.clone());
    map.insert("submission_location".to_string(), secrets.submission.location.clone());

    // Signature placeholders.
    map.insert("participant_signature_top".to_string(), "SIGNATURE".to_string());
    map.insert("participant_signature_top_date".to_string(), "DATE".to_string());
    map.insert("participant_signature_bottom".to_string(), "SIGNATURE".to_string());
    map.insert("participant_signature_bottom_date".to_string(), "DATE".to_string());

    // "Should you become employed" placeholders.
    map.insert("employed_name_address_line1".to_string(), "Acme Corp".to_string());
    map.insert("employed_name_address_line2".to_string(), "123 Employment Ave".to_string());
    map.insert("employed_name_address_line3".to_string(), "Las Vegas, NV 89101".to_string());
    map.insert("employed_telephone".to_string(), "702-555-0100".to_string());
    map.insert("employed_start_date".to_string(), "04/15/26".to_string());
    map.insert("employed_hours_per_week".to_string(), "40".to_string());
    map.insert("employed_hourly_rate".to_string(), "$15.00".to_string());

    // Pay frequency options (all shown for calibration).
    map.insert("employed_pay_frequency_weekly".to_string(), "Weekly".to_string());
    map.insert("employed_pay_frequency_biweekly".to_string(), "Bi-weekly".to_string());
    map.insert("employed_pay_frequency_semimonthly".to_string(), "Semi-monthly".to_string());
    map.insert("employed_pay_frequency_monthly".to_string(), "Monthly".to_string());

    // Date of first check.
    map.insert("employed_first_check_month".to_string(), "05".to_string());
    map.insert("employed_first_check_day".to_string(), "01".to_string());
    map.insert("employed_first_check_year".to_string(), "26".to_string());

    map.insert("employed_tips".to_string(), "No".to_string());
    map.insert("employed_job_title".to_string(), "Analyst".to_string());

    // Insurance options (all shown for calibration).
    map.insert("employed_insurance_none".to_string(), "None".to_string());
    map.insert("employed_insurance_employer_paid".to_string(), "Employer".to_string());
    map.insert("employed_insurance_employee_paid".to_string(), "Employee".to_string());
    map.insert("employed_insurance_both_paid".to_string(), "Both".to_string());

    map
}
