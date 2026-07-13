//! # CYBERDECK: Style Registry
//!
//! The central hub for all stylesheet processing.
//! Manages different style formats (CSS, SCSS, Custom DSL) via a pluggable loader system.
//!
//! ## Implementation Notes
//! - **Pluggable**: New loaders can be added by implementing the `StyleLoader` trait.
//! - **Documentation**: Includes automated `generate_docs` to keep `docs/STYLE_REFERENCE.md` in sync.

use crate::style_api;
use std::collections::HashMap;

pub trait StyleLoader {
    fn transpile(&self, input: &str) -> Result<String, String>;
}

// Concrete implementation for your Custom DSL
pub struct CyberStyleLoader;
impl StyleLoader for CyberStyleLoader {
    fn transpile(&self, input: &str) -> Result<String, String> {
        let elements = style_api::parse_dsl(input)?;
        // Logic to convert the AST elements into CSS string goes here
        Ok(format!(
            "/* Compiled CYBERDECK CSS */\n/* Count: {} blocks */",
            elements.len()
        ))
    }
}

pub struct StyleRegistry {
    loaders: HashMap<String, Box<dyn StyleLoader>>,
}

impl StyleRegistry {
    pub fn new() -> Self {
        let mut loaders: HashMap<String, Box<dyn StyleLoader>> = HashMap::new();
        loaders.insert("cyber".to_string(), Box::new(CyberStyleLoader));
        Self { loaders }
    }

    pub fn process_file(&self, ext: &str, content: &str) -> Result<String, String> {
        match self.loaders.get(ext) {
            Some(loader) => loader.transpile(content),
            None => Err(format!("No loader registered for extension: {}", ext)),
        }
    }
}

pub fn generate_docs() -> std::io::Result<()> {
    let map = crate::style_api::translator::get_shorthand_map();
    let mut content =
        String::from("# CYBERDECK Style Reference\n\n| Shorthand | CSS Property |\n|---|---|\n");

    let mut keys: Vec<_> = map.keys().collect();
    keys.sort();

    for key in keys {
        content.push_str(&format!("| {} | {} |\n", key, map.get(key).unwrap()));
    }

    std::fs::write("docs/STYLE_REFERENCE.md", content)?;
    println!("✅ Documentation generated at docs/STYLE_REFERENCE.md");
    Ok(())
}
