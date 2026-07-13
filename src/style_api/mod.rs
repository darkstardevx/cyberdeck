//! # CYBERDECK: Style API
//!
//! The DSL transpiler and CSS engine.
//! Translates high-density shorthand (e.g., "clr", "m") into
//! standard web-compliant CSS.
//!
//! ## Implementation Notes
//! - **Parser**: Uses `pest` for recursive descent parsing.
//! - **Translator**: Thread-safe global dictionary for shorthand expansion.

use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;

mod translator;

#[derive(Parser)]
#[grammar = "styles.pest"] // Paths are relative to project root
struct StyleParser;

#[derive(Debug, Clone, PartialEq)]
pub struct StyleDeclaration {
    pub property: String,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StyleRuleBlock {
    pub selector: String,
    pub declarations: Vec<StyleDeclaration>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StyleDeclaration {
    pub property: String,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StyleRuleBlock {
    pub selector: String,
    pub declarations: Vec<StyleDeclaration>,
}

// A higher-level container that holds either a root style or a responsive media style
#[derive(Debug, Clone, PartialEq)]
pub enum StylesheetElement {
    Standard(StyleRuleBlock),
    Media {
        query: String,
        rules: Vec<StyleRuleBlock>,
    },
}

pub fn print_cheat_sheet() {
    println!("=== 📖 CYBERDECK STYLE CHEAT SHEET ===");
    println!("Shorthand | Standard CSS");
    println!("----------|-------------");
    // Dynamically iterate over your translator map here
    for (shorthand, standard) in shorthand_map() {
        println!("{: <9} | {}", shorthand, standard);
    }
}
pub fn print_help() {
    let map = translator::get_shorthand_map();
    println!("\n=== 📖 CYBERDECK STYLE ENGINE: CHEAT SHEET ===");
    println!("{:<15} | {:<20}", "SHORTHAND", "STANDARD CSS");
    println!("{:-<15}-|-{:-<20}", "", "");

    let mut keys: Vec<_> = map.keys().collect();
    keys.sort();

    for key in keys {
        println!("{:<15} | {:<20}", key, map.get(key).unwrap());
    }
    println!("==============================================\n");
}

/// The main API entrypoint. Pass it raw text, get a clean Vector of parsed style blocks.
pub fn parse_dsl(input: &str) -> Result<Vec<StylesheetElement>, String> {
    let mut elements = Vec::new();

    let parsed_file = StyleParser::parse(Rule::stylesheet, input)
        .map_err(|e| format!("DSL Syntax Error:\n{}", e))?;

    if let Some(stylesheet_pair) = parsed_file.into_iter().next() {
        for command in stylesheet_pair.into_inner() {
            if command.as_rule() == Rule::command {
                let inner_block = command.into_inner().next().unwrap();

                match inner_block.as_rule() {
                    Rule::style_block => {
                        let rule = parse_single_style_block(inner_block);
                        elements.push(StylesheetElement::Standard(rule));
                    }
                    Rule::media_block => {
                        let mut inner_pairs = inner_block.into_inner();
                        let query = inner_pairs.next().unwrap().as_str().trim().to_string();
                        let mut rules = Vec::new();

                        // Gather all style blocks inside the media block
                        for style_pair in inner_pairs {
                            if style_pair.as_rule() == Rule::style_block {
                                rules.push(parse_single_style_block(style_pair));
                            }
                        }

                        elements.push(StylesheetElement::Media { query, rules });
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    Ok(elements)
}

// Helper to parse individual style blocks cleanly
fn parse_single_style_block(pair: pest::iterators::Pair<Rule>) -> StyleRuleBlock {
    let mut inner_tokens = pair.into_inner();
    let selector = inner_tokens.next().unwrap().as_str().trim().to_string();
    let mut declarations = Vec::new();

    for decl_pair in inner_tokens {
        let mut tokens = decl_pair.into_inner();
        let raw_prop = tokens.next().unwrap().as_str().trim();
        let raw_val = tokens.next().unwrap().as_str().trim();

        declarations.push(StyleDeclaration {
            property: crate::style_api::translator::translate(raw_prop),
            value: raw_val.to_string(),
        });
    }

    StyleRuleBlock {
        selector,
        declarations,
    }
}
