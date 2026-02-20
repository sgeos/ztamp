//! tanf-export: CLI tool for generating a complete TANF job search PDF packet.
//!
//! Produces a single PDF containing:
//! 1. Cover page with summary statistics
//! 2. TANF form pages (10 entries each) with template background
//! 3. Screenshot pages (1 per entry) with header info
//!
//! Usage:
//!     tanf-export --manifest <path> --offsets <path> --secrets <path>
//!                 --template <path> --output <path>

use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::{Path, PathBuf};
use std::process;

use lopdf::{Document, Object, ObjectId};
use serde::Deserialize;

use rztamp::offsets;
use rztamp::pdf::{self, Rotation, TextColor, TextField};

// -- Manifest types (from Elixir JSON) --

#[derive(Debug, Deserialize)]
struct Manifest {
    date_from: String,
    date_to: String,
    #[allow(dead_code)]
    total_hours: u32,
    total_time_display: String,
    total_entries: u32,
    recruiter_count: u32,
    entries: Vec<ManifestEntry>,
}

#[derive(Debug, Deserialize)]
struct ManifestEntry {
    date: String,
    employer_name_address: String,
    how_contact_made: String,
    telephone_fax: String,
    telephone_number: String,
    internet_confirmation: String,
    time_in: String,
    time_out: String,
    screenshot_path: String,
}

// -- Secrets types (reused from fill.rs) --

#[derive(Debug, Deserialize)]
struct Secrets {
    #[allow(dead_code)]
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
    #[allow(dead_code)]
    date_from: String,
    #[allow(dead_code)]
    date_to: String,
    hours: u32,
}

#[derive(Debug, Deserialize)]
struct SubmissionSecrets {
    deadline_time: String,
    deadline_date: String,
    location: String,
}

// -- CLI argument parsing --

struct Args {
    manifest_path: PathBuf,
    offsets_path: PathBuf,
    secrets_path: PathBuf,
    template_path: PathBuf,
    output_path: PathBuf,
}

fn parse_args() -> Args {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 11 {
        eprintln!(
            "Usage: tanf-export --manifest <path> --offsets <path> --secrets <path> \
             --template <path> --output <path>"
        );
        process::exit(1);
    }

    let mut manifest_path = None;
    let mut offsets_path = None;
    let mut secrets_path = None;
    let mut template_path = None;
    let mut output_path = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--manifest" => {
                i += 1;
                manifest_path = Some(PathBuf::from(&args[i]));
            }
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
            other => {
                eprintln!("Unknown argument: {other}");
                process::exit(1);
            }
        }
        i += 1;
    }

    let missing = [
        manifest_path.is_none().then_some("--manifest"),
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
        manifest_path: manifest_path.unwrap(),
        offsets_path: offsets_path.unwrap(),
        secrets_path: secrets_path.unwrap(),
        template_path: template_path.unwrap(),
        output_path: output_path.unwrap(),
    }
}

// -- Page generation --

/// Build cover page text fields.
fn build_cover_page(
    secrets: &Secrets,
    manifest: &Manifest,
) -> Vec<TextField> {
    let black = TextColor::BLACK;
    let mut fields = Vec::new();

    // Title
    fields.push(TextField {
        text: "TANF Individual Job Search Activity Record".to_string(),
        x_mm: 30.0,
        y_mm: 40.0,
        font_size: 16.0,
        color: black,
    });
    fields.push(TextField {
        text: "Proof of Job Search Packet".to_string(),
        x_mm: 30.0,
        y_mm: 50.0,
        font_size: 14.0,
        color: black,
    });

    // Participant info
    fields.push(TextField {
        text: format!("Name: {}", secrets.participant.case_name),
        x_mm: 30.0,
        y_mm: 75.0,
        font_size: 12.0,
        color: black,
    });
    fields.push(TextField {
        text: format!("UPI: {}", secrets.participant.upi_number),
        x_mm: 30.0,
        y_mm: 85.0,
        font_size: 12.0,
        color: black,
    });

    // Period
    fields.push(TextField {
        text: format!(
            "Period Covered: {} to {}",
            manifest.date_from, manifest.date_to
        ),
        x_mm: 30.0,
        y_mm: 100.0,
        font_size: 12.0,
        color: black,
    });

    // Statistics
    fields.push(TextField {
        text: format!("Total Applications: {}", manifest.total_entries),
        x_mm: 30.0,
        y_mm: 120.0,
        font_size: 12.0,
        color: black,
    });
    fields.push(TextField {
        text: format!("Applications via Recruiter: {}", manifest.recruiter_count),
        x_mm: 30.0,
        y_mm: 130.0,
        font_size: 12.0,
        color: black,
    });
    fields.push(TextField {
        text: format!("Total Time Spent: {}", manifest.total_time_display),
        x_mm: 30.0,
        y_mm: 140.0,
        font_size: 12.0,
        color: black,
    });

    // Submission info
    fields.push(TextField {
        text: format!(
            "Submission Deadline: {} on {}",
            secrets.submission.deadline_time, secrets.submission.deadline_date
        ),
        x_mm: 30.0,
        y_mm: 160.0,
        font_size: 11.0,
        color: black,
    });
    fields.push(TextField {
        text: format!("Submission Location: {}", secrets.submission.location),
        x_mm: 30.0,
        y_mm: 170.0,
        font_size: 11.0,
        color: black,
    });

    fields
}

/// Build header field values for a TANF form page.
fn build_form_header_values(
    secrets: &Secrets,
    manifest: &Manifest,
) -> HashMap<String, String> {
    let mut map = HashMap::new();

    // Participant fields.
    map.insert("case_name".to_string(), secrets.participant.case_name.clone());
    map.insert("upi_number".to_string(), secrets.participant.upi_number.clone());

    // Job search period from manifest (overrides secrets).
    map.insert("job_search_from".to_string(), manifest.date_from.clone());
    map.insert("job_search_to".to_string(), manifest.date_to.clone());
    map.insert("job_search_hours".to_string(), secrets.job_search.hours.to_string());

    // Submission fields from secrets.
    map.insert("submission_deadline_time".to_string(), secrets.submission.deadline_time.clone());
    map.insert("submission_deadline_date".to_string(), secrets.submission.deadline_date.clone());
    map.insert("submission_location".to_string(), secrets.submission.location.clone());

    map
}

/// Convert a ManifestEntry to a table row HashMap.
fn entry_to_table_row(entry: &ManifestEntry) -> HashMap<String, String> {
    let mut row = HashMap::new();
    row.insert("date".to_string(), entry.date.clone());
    row.insert("employer_name_address".to_string(), entry.employer_name_address.clone());
    row.insert("how_contact_made".to_string(), entry.how_contact_made.clone());
    row.insert("telephone_fax".to_string(), entry.telephone_fax.clone());
    if !entry.telephone_number.is_empty() {
        row.insert("telephone_number".to_string(), entry.telephone_number.clone());
    }
    if !entry.internet_confirmation.is_empty() {
        row.insert("internet_confirmation".to_string(), entry.internet_confirmation.clone());
    }
    row.insert("time_in".to_string(), entry.time_in.clone());
    row.insert("time_out".to_string(), entry.time_out.clone());
    row
}

/// Build screenshot page header text fields.
fn build_screenshot_header(
    secrets: &Secrets,
    entry: &ManifestEntry,
) -> Vec<TextField> {
    let black = TextColor::BLACK;
    let mut fields = Vec::new();

    fields.push(TextField {
        text: "TANF Job Search - Proof of Application".to_string(),
        x_mm: 15.0,
        y_mm: 12.0,
        font_size: 12.0,
        color: black,
    });
    fields.push(TextField {
        text: format!(
            "Name: {}    UPI: {}",
            secrets.participant.case_name, secrets.participant.upi_number
        ),
        x_mm: 15.0,
        y_mm: 20.0,
        font_size: 10.0,
        color: black,
    });
    fields.push(TextField {
        text: format!(
            "Date: {}    Employer: {}",
            entry.date, entry.employer_name_address
        ),
        x_mm: 15.0,
        y_mm: 27.0,
        font_size: 10.0,
        color: black,
    });
    fields.push(TextField {
        text: format!(
            "Contact: {}    Time: {} - {}",
            entry.how_contact_made, entry.time_in, entry.time_out
        ),
        x_mm: 15.0,
        y_mm: 34.0,
        font_size: 10.0,
        color: black,
    });

    fields
}

// -- PDF merge (adapted from concat.rs) --

/// Merge multiple lopdf Documents into a single Document.
fn merge_documents(documents: Vec<Document>) -> Result<Document, Box<dyn std::error::Error>> {
    let mut max_id = 1u32;
    let mut all_pages: BTreeMap<ObjectId, Object> = BTreeMap::new();
    let mut all_objects: BTreeMap<ObjectId, Object> = BTreeMap::new();
    let mut merged = Document::with_version("1.5");

    for mut doc in documents {
        doc.renumber_objects_with(max_id);
        max_id = doc.max_id + 1;

        for (_page_num, object_id) in doc.get_pages() {
            let page_object = doc
                .get_object(object_id)
                .map_err(|e| format!("Failed to get page object {object_id:?}: {e}"))?
                .to_owned();
            all_pages.insert(object_id, page_object);
        }

        all_objects.extend(doc.objects);
    }

    let mut catalog_object: Option<(ObjectId, Object)> = None;
    let mut pages_object: Option<(ObjectId, Object)> = None;

    for (object_id, object) in all_objects {
        match object.type_name().unwrap_or(b"") {
            b"Catalog" => {
                if catalog_object.is_none() {
                    catalog_object = Some((object_id, object));
                }
            }
            b"Pages" => {
                if let Ok(dict) = object.as_dict() {
                    let mut dict = dict.clone();
                    if let Some((_, ref prev)) = pages_object {
                        if let Ok(old) = prev.as_dict() {
                            dict.extend(old);
                        }
                    }
                    pages_object = Some((
                        pages_object.map_or(object_id, |(id, _)| id),
                        Object::Dictionary(dict),
                    ));
                }
            }
            b"Page" | b"Outlines" | b"Outline" => {}
            _ => {
                merged.objects.insert(object_id, object);
            }
        }
    }

    let (pages_id, pages_obj) = pages_object
        .ok_or("No Pages object found in input documents")?;
    let (catalog_id, catalog_obj) = catalog_object
        .ok_or("No Catalog object found in input documents")?;

    if let Ok(dict) = pages_obj.as_dict() {
        let mut dict = dict.clone();
        dict.set("Count", all_pages.len() as u32);
        dict.set(
            "Kids",
            all_pages
                .keys()
                .map(|id| Object::Reference(*id))
                .collect::<Vec<_>>(),
        );
        merged.objects.insert(pages_id, Object::Dictionary(dict));
    }

    for (object_id, object) in &all_pages {
        if let Ok(dict) = object.as_dict() {
            let mut dict = dict.clone();
            dict.set("Parent", pages_id);
            merged.objects.insert(*object_id, Object::Dictionary(dict));
        }
    }

    if let Ok(dict) = catalog_obj.as_dict() {
        let mut dict = dict.clone();
        dict.set("Pages", pages_id);
        merged.objects.insert(catalog_id, Object::Dictionary(dict));
    }

    merged.trailer.set("Root", catalog_id);
    merged.max_id = merged.objects.len() as u32;
    merged.renumber_objects();
    merged.adjust_zero_pages();

    Ok(merged)
}

// -- Main --

fn load_secrets(path: &Path) -> Result<Secrets, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let secrets: Secrets = toml::from_str(&content)?;
    Ok(secrets)
}

fn load_manifest(path: &Path) -> Result<Manifest, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let manifest: Manifest = serde_json::from_str(&content)?;
    Ok(manifest)
}

fn main() {
    let args = parse_args();

    // Validate output does not overwrite.
    if args.output_path.exists() {
        eprintln!(
            "Error: output file already exists: {}",
            args.output_path.display()
        );
        eprintln!("Refusing to overwrite. Choose a different output path.");
        process::exit(1);
    }

    // Load inputs.
    let form_offsets = offsets::load_offsets(&args.offsets_path)
        .unwrap_or_else(|e| {
            eprintln!("Failed to load offsets: {e}");
            process::exit(1);
        });

    let secrets = load_secrets(&args.secrets_path)
        .unwrap_or_else(|e| {
            eprintln!("Failed to load secrets: {e}");
            process::exit(1);
        });

    let manifest = load_manifest(&args.manifest_path)
        .unwrap_or_else(|e| {
            eprintln!("Failed to load manifest: {e}");
            process::exit(1);
        });

    let template_bytes = fs::read(&args.template_path)
        .unwrap_or_else(|e| {
            eprintln!("Failed to read template image: {e}");
            process::exit(1);
        });

    let page_w = form_offsets.meta.page_width_mm;
    let page_h = form_offsets.meta.page_height_mm;
    let template_dpi = form_offsets.meta.template_dpi as f32;

    let mut pdf_documents: Vec<Document> = Vec::new();

    // -- 1. Cover page --
    eprintln!("Generating cover page...");
    let cover_fields = build_cover_page(&secrets, &manifest);
    let cover_bytes = pdf::generate_text_page(page_w, page_h, &cover_fields)
        .unwrap_or_else(|e| {
            eprintln!("Failed to generate cover page: {e}");
            process::exit(1);
        });
    let cover_doc = Document::load_mem(&cover_bytes)
        .unwrap_or_else(|e| {
            eprintln!("Failed to parse cover page PDF: {e}");
            process::exit(1);
        });
    pdf_documents.push(cover_doc);

    // -- 2. TANF form pages (10 entries per page) --
    let header_values = build_form_header_values(&secrets, &manifest);
    let black = TextColor::BLACK;

    let chunks: Vec<&[ManifestEntry]> = manifest.entries.chunks(10).collect();
    for (page_idx, chunk) in chunks.iter().enumerate() {
        eprintln!(
            "Generating TANF form page {} ({} entries)...",
            page_idx + 1,
            chunk.len()
        );

        // Build header text fields (no table, no debug data).
        let (header_fields, circle_marks) = pdf::build_calibration_fields(
            &form_offsets,
            &header_values,
            black,
            black,
            black,
            false, // show_circle_labels
            false, // circle_all
            false, // fill_table
        );

        // Build table text fields from custom entry data.
        let table_rows: Vec<HashMap<String, String>> =
            chunk.iter().map(|e| entry_to_table_row(e)).collect();
        let table_fields = pdf::build_table_fields(
            &form_offsets,
            &table_rows,
            black,
            black,
        );

        // Combine header and table fields.
        let mut all_fields = header_fields;
        all_fields.extend(table_fields);

        // Generate the form page with template background.
        let form_bytes = pdf::generate_form_pdf(
            &template_bytes,
            template_dpi,
            page_w,
            page_h,
            &all_fields,
            &circle_marks,
            Rotation::Ccw90,
            None, // no grid
            None, // no watermark
        )
        .unwrap_or_else(|e| {
            eprintln!("Failed to generate form page {}: {e}", page_idx + 1);
            process::exit(1);
        });

        let form_doc = Document::load_mem(&form_bytes)
            .unwrap_or_else(|e| {
                eprintln!("Failed to parse form page PDF: {e}");
                process::exit(1);
            });
        pdf_documents.push(form_doc);
    }

    // -- 3. Screenshot pages (1 per entry) --
    for (entry_idx, entry) in manifest.entries.iter().enumerate() {
        if entry.screenshot_path.is_empty() {
            eprintln!(
                "Skipping screenshot page for entry {} (no screenshot).",
                entry_idx + 1
            );
            continue;
        }

        eprintln!(
            "Generating screenshot page {} of {}...",
            entry_idx + 1,
            manifest.entries.len()
        );

        let screenshot_bytes = fs::read(&entry.screenshot_path)
            .unwrap_or_else(|e| {
                eprintln!(
                    "Failed to read screenshot {}: {e}",
                    entry.screenshot_path
                );
                process::exit(1);
            });

        let header_fields = build_screenshot_header(&secrets, entry);

        let screenshot_page_bytes = pdf::generate_image_page(
            page_w,
            page_h,
            &screenshot_bytes,
            42.0,                // image area starts 42mm from top
            page_w - 30.0,       // 15mm margins each side
            page_h - 42.0 - 10.0, // remaining height minus bottom margin
            &header_fields,
        )
        .unwrap_or_else(|e| {
            eprintln!(
                "Failed to generate screenshot page {}: {e}",
                entry_idx + 1
            );
            process::exit(1);
        });

        let screenshot_doc = Document::load_mem(&screenshot_page_bytes)
            .unwrap_or_else(|e| {
                eprintln!("Failed to parse screenshot page PDF: {e}");
                process::exit(1);
            });
        pdf_documents.push(screenshot_doc);
    }

    // -- 4. Merge all pages --
    eprintln!(
        "Merging {} PDF documents...",
        pdf_documents.len()
    );

    let mut merged = merge_documents(pdf_documents)
        .unwrap_or_else(|e| {
            eprintln!("Failed to merge PDFs: {e}");
            process::exit(1);
        });

    // Save.
    merged.save(&args.output_path).unwrap_or_else(|e| {
        eprintln!(
            "Failed to write output to {}: {e}",
            args.output_path.display()
        );
        process::exit(1);
    });

    let size = fs::metadata(&args.output_path)
        .map(|m| m.len())
        .unwrap_or(0);
    eprintln!("Output written to: {}", args.output_path.display());
    eprintln!("PDF size: {} bytes ({} pages)", size, 1 + chunks.len() + manifest.entries.len());
}
