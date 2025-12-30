    // Initialize beads if requested
    if let Some(vars) = vars_file {
        if let Ok(vars_map) = serde_yaml::from_str::<std::collections::HashMap<String, serde_yaml::Value>>(
            &std::fs::read_to_string(&std::fs::canonicalize(vars)?)?
        ) {
            if vars_map.get("initialize_beads").and_then(|v| v.as_bool()).unwrap_or(false) {
                initialize_beads(out)?;
            }
        }
    }