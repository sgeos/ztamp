//! tanf-concat: CLI tool for concatenating multiple PDF files into one.
//!
//! Usage:
//!     tanf-concat --output <path> <input1.pdf> <input2.pdf> ...
//!
//! Merges the input PDFs in the order specified, producing a single output
//! PDF. Refuses to overwrite an existing output file.

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::process;

use lopdf::{Document, Object, ObjectId};

/// Parsed command line arguments.
struct Args {
    output_path: PathBuf,
    input_paths: Vec<PathBuf>,
}

fn parse_args() -> Args {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 4 {
        eprintln!("Usage: tanf-concat --output <path> <input1.pdf> <input2.pdf> ...");
        eprintln!("  Concatenates multiple PDF files into a single output PDF.");
        process::exit(1);
    }

    let mut output_path: Option<PathBuf> = None;
    let mut input_paths: Vec<PathBuf> = Vec::new();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--output" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("Error: --output requires a path argument.");
                    process::exit(1);
                }
                output_path = Some(PathBuf::from(&args[i]));
            }
            other => {
                input_paths.push(PathBuf::from(other));
            }
        }
        i += 1;
    }

    let output_path = output_path.unwrap_or_else(|| {
        eprintln!("Error: --output is required.");
        process::exit(1);
    });

    if input_paths.len() < 2 {
        eprintln!("Error: at least two input PDF files are required.");
        process::exit(1);
    }

    Args {
        output_path,
        input_paths,
    }
}

/// Merge multiple lopdf Documents into a single Document.
///
/// Follows the canonical lopdf merge pattern: renumber object IDs to avoid
/// collisions, collect pages and objects, then stitch together the Catalog
/// and Pages tree.
fn merge_documents(documents: Vec<Document>) -> Result<Document, Box<dyn std::error::Error>> {
    let mut max_id = 1u32;
    let mut all_pages: BTreeMap<ObjectId, Object> = BTreeMap::new();
    let mut all_objects: BTreeMap<ObjectId, Object> = BTreeMap::new();
    let mut merged = Document::with_version("1.5");

    for mut doc in documents {
        doc.renumber_objects_with(max_id);
        max_id = doc.max_id + 1;

        // Collect page objects.
        for (_page_num, object_id) in doc.get_pages() {
            let page_object = doc
                .get_object(object_id)
                .map_err(|e| format!("Failed to get page object {object_id:?}: {e}"))?
                .to_owned();
            all_pages.insert(object_id, page_object);
        }

        // Collect all objects.
        all_objects.extend(doc.objects);
    }

    // Separate Catalog and Pages objects from everything else.
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
            // Skip Page objects (handled via all_pages) and outline objects.
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

    // Build the unified Pages node with all page references.
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

    // Insert all page objects, updating their Parent reference.
    for (object_id, object) in &all_pages {
        if let Ok(dict) = object.as_dict() {
            let mut dict = dict.clone();
            dict.set("Parent", pages_id);
            merged.objects.insert(*object_id, Object::Dictionary(dict));
        }
    }

    // Build the Catalog pointing to the unified Pages node.
    if let Ok(dict) = catalog_obj.as_dict() {
        let mut dict = dict.clone();
        dict.set("Pages", pages_id);
        merged.objects.insert(catalog_id, Object::Dictionary(dict));
    }

    // Finalize the document.
    merged.trailer.set("Root", catalog_id);
    merged.max_id = merged.objects.len() as u32;
    merged.renumber_objects();
    merged.adjust_zero_pages();

    Ok(merged)
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

    // Validate all input files exist.
    for path in &args.input_paths {
        if !path.exists() {
            eprintln!("Error: input file not found: {}", path.display());
            process::exit(1);
        }
    }

    eprintln!(
        "Concatenating {} PDF files...",
        args.input_paths.len()
    );

    // Load all input documents.
    let documents: Vec<Document> = args
        .input_paths
        .iter()
        .map(|path| {
            eprintln!("  Loading: {}", path.display());
            Document::load(path).unwrap_or_else(|e| {
                eprintln!("Failed to load PDF {}: {e}", path.display());
                process::exit(1);
            })
        })
        .collect();

    // Merge.
    let mut merged = merge_documents(documents).unwrap_or_else(|e| {
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

    let size = std::fs::metadata(&args.output_path)
        .map(|m| m.len())
        .unwrap_or(0);
    eprintln!("Output written to: {}", args.output_path.display());
    eprintln!("PDF size: {} bytes", size);
}
