use crate::config::{FileConfigManager, ConfigManager, ExportOptions, ExportFormat, ImportOptions, ImportFormat};
use crate::utils::file_utils::{detect_file_format, validate_file_format, FileFormat};
use crate::utils::feedback::{
    ProgressIndicator, display_error_with_suggestions, display_success_with_next_steps,
    display_warning, display_operation_summary, display_file_operation_result,
    display_verbose_info, format_file_size
};
use std::error::Error;
use std::path::Path;
use std::time::Instant;

/// Handle the export command to export configurations to a file
pub fn handle_export_command(
    config_manager: &FileConfigManager,
    output: Option<String>,
    configs: Vec<String>,
    format: String,
    metadata: bool,
    pretty: bool,
    verbose: bool,
) -> Result<(), Box<dyn Error>> {
    let start_time = Instant::now();
    let mut progress = ProgressIndicator::new("🚀 Starting export operation");
    
    if verbose {
        progress.start();
    }
    
    // Determine output file path
    let output_path = match output {
        Some(path) => path,
        None => "envswitch_export.json".to_string(),
    };
    
    if verbose {
        display_verbose_info("Export configuration", &[
            ("Output file", &output_path),
            ("Format", &format),
            ("Include metadata", &metadata.to_string()),
            ("Pretty print", &pretty.to_string()),
        ]);
    }
    
    // Parse and validate format
    let export_format = match format.to_lowercase().as_str() {
        "json" => ExportFormat::Json,
        "env" => ExportFormat::Env,
        "yaml" => ExportFormat::Yaml,
        _ => {
            let error = format!("Unsupported format '{}'. Supported formats: json, env, yaml", format);
            if verbose {
                progress.finish_error(&error);
            }
            return Err(error.into());
        }
    };
    
    // Validate specific configurations if provided
    if !configs.is_empty() {
        let available_configs = config_manager.list_configs()?;
        let mut invalid_configs = Vec::new();
        
        for config in &configs {
            if !available_configs.contains(config) {
                invalid_configs.push(config.clone());
            }
        }
        
        if !invalid_configs.is_empty() {
            let error = format!(
                "Configuration(s) not found: {}\nAvailable configurations: {}",
                invalid_configs.join(", "),
                available_configs.join(", ")
            );
            if verbose {
                progress.finish_error(&error);
            }
            return Err(error.into());
        }
        
        if verbose {
            println!("📋 Exporting {} specific configurations: {}", configs.len(), configs.join(", "));
        }
    } else {
        let all_configs = config_manager.list_configs()?;
        if all_configs.is_empty() {
            if verbose {
                progress.finish_warning("No configurations found to export");
            }
            display_warning(
                "No configurations found to export",
                Some(&["Create configurations first with: envswitch set <name> -e KEY=value"])
            );
            return Ok(());
        }
        
        if verbose {
            display_verbose_info("Export scope", &[
                ("Total configurations", &all_configs.len().to_string()),
                ("Configuration names", &all_configs.join(", ")),
            ]);
        }
    }
    
    // Create export options
    let export_options = ExportOptions {
        format: export_format,
        include_metadata: metadata,
        pretty_print: pretty,
        configs: if configs.is_empty() { None } else { Some(configs.clone()) },
    };
    
    // Create output directory if it doesn't exist
    let output_path_obj = Path::new(&output_path);
    if let Some(parent_dir) = output_path_obj.parent() {
        if !parent_dir.exists() {
            if verbose {
                display_verbose_info("Creating directory", &[
                    ("Path", &parent_dir.display().to_string()),
                ]);
                progress.tick();
            }
            std::fs::create_dir_all(parent_dir).map_err(|e| {
                if verbose {
                    progress.finish_error("Failed to create directory");
                }
                e
            })?;
        }
    }
    
    // Perform the export
    if verbose {
        progress.tick();
    }
    
    config_manager.export_to_file_with_options(output_path_obj, &export_options).map_err(|e| {
        if verbose {
            progress.finish_error("Export failed");
        }
        e
    })?;
    
    // Get file size for reporting
    let file_size = std::fs::metadata(output_path_obj)?.len();
    let file_size_str = if file_size < 1024 {
        format!("{} bytes", file_size)
    } else if file_size < 1024 * 1024 {
        format!("{:.1} KB", file_size as f64 / 1024.0)
    } else {
        format!("{:.1} MB", file_size as f64 / (1024.0 * 1024.0))
    };
    
    // Count exported configurations and variables
    let store = config_manager.load_configs()?;
    let exported_configs = if !configs.is_empty() {
        configs.len()
    } else {
        store.configs.len()
    };
    
    let total_variables: usize = if !configs.is_empty() {
        configs.iter()
            .filter_map(|name| store.configs.get(name))
            .map(|config| config.variables.len())
            .sum()
    } else {
        store.configs.values()
            .map(|config| config.variables.len())
            .sum()
    };
    
    let duration = start_time.elapsed();
    
    if verbose {
        progress.finish_success("Export completed successfully");
    }
    
    // Display file operation result
    display_file_operation_result("Export", &output_path, Some(file_size), true);
    
    // Display operation summary
    display_operation_summary(
        "Export",
        exported_configs,
        0,
        0,
        duration,
        Some(&[
            &format!("Total variables: {}", total_variables),
            &format!("File size: {}", format_file_size(file_size)),
            &format!("Format: {}", format),
        ])
    );
    
    if !configs.is_empty() {
        display_verbose_info("Exported configurations", &[
            ("Names", &configs.join(", ")),
        ]);
    }
    
    if verbose {
        let mut details = vec![
            ("Format", format.as_str()),
        ];
        if metadata {
            details.push(("Metadata", "included (timestamps and descriptions)"));
        }
        if pretty {
            details.push(("Formatting", "pretty print enabled"));
        }
        display_verbose_info("Export options", &details);
    }
    
    // Show next steps
    display_success_with_next_steps(
        &format!("Exported {} configurations", exported_configs),
        &[
            &format!("envswitch import {}        # Import on another machine", output_path),
            &format!("cat {}                     # View exported content", output_path),
        ]
    );
    
    Ok(())
}

/// Handle the import command to import configurations from a file
pub fn handle_import_command(
    config_manager: &FileConfigManager,
    file: String,
    force: bool,
    merge: bool,
    dry_run: bool,
    skip_validation: bool,
    backup: bool,
    verbose: bool,
) -> Result<(), Box<dyn Error>> {
    let start_time = Instant::now();
    let mut progress = ProgressIndicator::new("📥 Starting import operation");
    
    if verbose {
        progress.start();
        display_verbose_info("Import configuration", &[
            ("Import file", &file),
            ("Force overwrite", &force.to_string()),
            ("Merge existing", &merge.to_string()),
            ("Dry run", &dry_run.to_string()),
            ("Skip validation", &skip_validation.to_string()),
            ("Create backup", &backup.to_string()),
        ]);
    }
    
    // Check if import file exists
    let import_path = Path::new(&file);
    if !import_path.exists() {
        let error = format!("Import file '{}' not found", file);
        if verbose {
            progress.finish_error(&error);
        }
        display_error_with_suggestions(
            &std::io::Error::new(std::io::ErrorKind::NotFound, error.clone()),
            verbose
        );
        return Err(error.into());
    }
    
    // Detect and validate format based on file extension and content
    if verbose {
        progress.tick();
    }
    
    let detected_format = detect_file_format(import_path).map_err(|e| {
        if verbose {
            progress.finish_error("Format detection failed");
        }
        e
    })?;
    
    if verbose {
        display_verbose_info("Format detection", &[
            ("Detected format", &format!("{:?}", detected_format)),
        ]);
    }
    
    // Validate the file format
    if verbose {
        progress.tick();
    }
    
    let validation_result = validate_file_format(import_path, &detected_format).map_err(|e| {
        if verbose {
            progress.finish_error("Format validation failed");
        }
        e
    })?;
    
    if !validation_result.is_valid {
        let mut error_msg = format!("Invalid {} file format:", format!("{:?}", detected_format).to_lowercase());
        for error in &validation_result.errors {
            error_msg.push_str(&format!("\n  • {}", error));
        }
        if verbose {
            progress.finish_error("File validation failed");
        }
        return Err(error_msg.into());
    }
    
    // Show warnings if any
    if !validation_result.warnings.is_empty() {
        display_warning(
            "Format validation warnings",
            Some(&validation_result.warnings.iter().map(|s| s.as_str()).collect::<Vec<_>>())
        );
    }
    
    // Create backup if requested
    if backup && config_manager.config_file_exists() {
        if verbose {
            progress.tick();
            display_verbose_info("Creating backup", &[
                ("Reason", "Backup requested before import"),
            ]);
        }
        let backup_path = config_manager.backup_config().map_err(|e| {
            if verbose {
                progress.finish_error("Backup creation failed");
            }
            e
        })?;
        display_success_with_next_steps(
            &format!("Backup created: {}", backup_path.display()),
            &[]
        );
    }
    
    // Create import options
    let import_options = crate::config::ImportOptions {
        format: match detected_format {
            FileFormat::Json => crate::config::ImportFormat::Json,
            FileFormat::Env => crate::config::ImportFormat::Env,
            FileFormat::Yaml => crate::config::ImportFormat::Yaml,
        },
        force_overwrite: force,
        merge_existing: merge,
        skip_validation,
        dry_run,
    };
    
    if verbose {
        display_verbose_info("Import options", &[
            ("Force overwrite", &force.to_string()),
            ("Merge existing", &merge.to_string()),
            ("Dry run", &dry_run.to_string()),
            ("Skip validation", &skip_validation.to_string()),
        ]);
        progress.tick();
    }
    
    // Perform the import
    let result = config_manager.import_from_file_with_options(import_path, &import_options).map_err(|e| {
        if verbose {
            progress.finish_error("Import operation failed");
        }
        display_error_with_suggestions(&e, verbose);
        e
    })?;
    
    if dry_run {
        println!("🔍 Dry run results:");
        if !result.imported.is_empty() {
            println!("✅ Would import {} configurations:", result.imported.len());
            for config in &result.imported {
                println!("   • {}", config);
            }
        }
        
        if !result.conflicts.is_empty() {
            println!("⚠️  {} conflicts found:", result.conflicts.len());
            for config in &result.conflicts {
                println!("   • {} (already exists)", config);
            }
            
            if !force && !merge {
                println!();
                println!("💡 Resolution options:");
                println!("   --force    Overwrite existing configurations");
                println!("   --merge    Merge with existing configurations");
            }
        }
        
        if !result.errors.is_empty() {
            println!("❌ {} errors found:", result.errors.len());
            for error in &result.errors {
                println!("   • {}", error);
            }
        }
        
        println!();
        println!("🚀 To perform the actual import, run the same command without --dry-run");
        return Ok(());
    }
    
    // Report results
    if !result.imported.is_empty() {
        println!("✅ Successfully imported {} configurations:", result.imported.len());
        for config in &result.imported {
            println!("   • {}", config);
        }
    }
    
    if !result.conflicts.is_empty() {
        if force {
            println!("🔄 Overwrote {} existing configurations:", result.conflicts.len());
            for config in &result.conflicts {
                println!("   • {}", config);
            }
        } else if merge {
            println!("🔗 Merged with {} existing configurations:", result.conflicts.len());
            for config in &result.conflicts {
                println!("   • {}", config);
            }
        } else {
            println!("⚠️  {} conflicts found (skipped):", result.conflicts.len());
            for config in &result.conflicts {
                println!("   • {} (already exists)", config);
            }
            println!();
            println!("💡 To resolve conflicts:");
            println!("   --force    Overwrite existing configurations");
            println!("   --merge    Merge with existing configurations");
        }
    }
    
    if !result.errors.is_empty() {
        println!("❌ {} errors occurred:", result.errors.len());
        for error in &result.errors {
            println!("   • {}", error);
        }
        return Err("Import completed with errors".into());
    }
    
    let total_imported = result.imported.len();
    if total_imported > 0 {
        println!();
        println!("📊 Import summary:");
        println!("   Total imported: {}", total_imported);
        
        if verbose {
            // Count total variables imported
            let store = config_manager.load_configs()?;
            let total_variables: usize = result.imported.iter()
                .filter_map(|name| store.configs.get(name))
                .map(|config| config.variables.len())
                .sum();
            println!("   Total variables: {}", total_variables);
        }
        
        println!();
        println!("🚀 Next steps:");
        println!("   envswitch list             # View all configurations");
        if let Some(first_config) = result.imported.first() {
            println!("   envswitch use {}           # Activate imported configuration", first_config);
        }
    } else {
        println!("📭 No configurations were imported");
    }
    
    Ok(())
}

// Format detection is now handled by utils::file_utils module