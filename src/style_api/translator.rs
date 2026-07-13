//! # CYBERDECK: Style Translator
//!
//! Provides the canonical dictionary for shorthand-to-standard CSS conversion.
//! Uses `OnceLock` for efficient, thread-safe global access.
//!
//! ## Implementation Notes
//! - **Lookup Table**: Centralizes all property shorthands.
//! - **Safety**: Ensures no overhead on repeated lookups.

use std::collections::HashMap;
use std::sync::OnceLock;

pub fn get_shorthand_map() -> &'static std::collections::HashMap<&'static str, &'static str> {
    shorthand_map()
}

/// Thread-safe global dictionary for your custom shorthand mappings
fn shorthand_map() -> &'static HashMap<&'static str, &'static str> {
    static MAP: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();
    MAP.get_or_init(|| {
        let mut m = HashMap::new();

        // 1. Layout & Positioning
        m.insert("dsp", "display");
        m.insert("psn", "position");
        m.insert("flxdir", "flex-direction");
        m.insert("juscon", "justify-content");
        m.insert("alit", "align-items");
        m.insert("gtcol", "grid-template-columns");
        m.insert("g", "gap");
        m.insert("f", "float");
        m.insert("clm", "clear");
        m.insert("zin", "z-index");
        m.insert("flx", "flex"); // Added: Flex child shorthand
        m.insert("grow", "flex-grow"); // Added: Flex grow control
        m.insert("shrk", "flex-shrink"); // Added: Flex shrink control

        // 2. Box Model (Spacing & Sizing)
        m.insert("w", "width");
        m.insert("h", "height");
        m.insert("maxw", "max-width"); // Added: Sizing boundary
        m.insert("minw", "min-width"); // Added: Sizing boundary
        m.insert("maxh", "max-height"); // Added: Sizing boundary
        m.insert("minh", "min-height"); // Added: Sizing boundary
        m.insert("m", "margin");
        m.insert("p", "padding");
        m.insert("bor", "border");
        m.insert("bxsz", "box-sizing");
        m.insert("of", "overflow");

        // 3. Typography & Text Styling
        m.insert("clr", "color");
        m.insert("fnt", "font-family");
        m.insert("fs", "font-size");
        m.insert("fw", "font-weight");
        m.insert("tal", "text-align");
        m.insert("texdec", "text-decoration");
        m.insert("lineh", "line-height");
        m.insert("lspc", "letter-spacing");

        // 4. Backgrounds & Visuals
        m.insert("bgclr", "background-color");
        m.insert("bgim", "background-image");
        m.insert("bgsz", "background-size");
        m.insert("rad", "border-radius");
        m.insert("bxshad", "box-shadow");

        // 5. Interface & Interaction (New Additions)
        m.insert("trn", "transition"); // Added: UI Transitions
        m.insert("cur", "cursor"); // Added: Mouse Pointer control
        m.insert("usrsel", "user-select"); // Added: Text highlight locking
        m.insert("ani", "animation"); // Added: Custom animations

        m
    })
}

/// Takes a raw token (like "clr") and turns it into standard target property (like "color")
pub fn translate(raw_property: &str) -> String {
    shorthand_map()
        .get(raw_property)
        .map(|&standard| standard.to_string())
        .unwrap_or_else(|| raw_property.to_string())
}
