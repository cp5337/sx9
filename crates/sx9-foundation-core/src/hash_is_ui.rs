//! Hash-IS-UI System - Direct Hash to UI Mapping
//!
//! GROUND TRUTH: Direct hash-to-UI mapping with no translation layers
//! LUT systems for colors, symbols, animations based on hash positions

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Hash-IS-UI System implementation
#[derive(Debug, Clone)]
pub struct HashIsUISystem {
    pub color_lut: HashMap<String, String>, // 3 bits -> 8 color schemes
    pub symbol_lut: HashMap<String, String>, // 4 bits -> 16 symbol sets
    pub animation_lut: HashMap<String, String>, // 2 bits -> 4 animation types
    pub initialized: bool,
}

/// Visual properties extracted from SCH positions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualProperties {
    pub primary_color: String,
    pub secondary_color: String,
    pub symbol_set: String,
    pub icon_style: String,
}

/// Animation properties extracted from CUID positions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationProperties {
    pub animation_type: String,
    pub duration: u32,
    pub easing: String,
    pub loop_behavior: String,
}

impl HashIsUISystem {
    #[must_use]
    pub fn new() -> Self {
        Self {
            color_lut: HashMap::new(),
            symbol_lut: HashMap::new(),
            animation_lut: HashMap::new(),
            initialized: false,
        }
    }

    /// Initialize LUT systems
    pub async fn initialize_lut_systems(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.initialize_color_lut();
        self.initialize_symbol_lut();
        self.initialize_animation_lut();

        self.initialized = true;
        println!("🎨 Hash-IS-UI LUT systems initialized");
        Ok(())
    }

    /// Initialize color lookup table
    fn initialize_color_lut(&mut self) {
        let colors = vec![
            ("000", "#FF0000"),
            ("001", "#00FF00"),
            ("010", "#0000FF"),
            ("011", "#FFFF00"),
            ("100", "#FF00FF"),
            ("101", "#00FFFF"),
            ("110", "#FFA500"),
            ("111", "#800080"),
        ];

        for (key, color) in colors {
            self.color_lut.insert(key.to_string(), color.to_string());
        }
    }

    /// Initialize symbol lookup table
    fn initialize_symbol_lut(&mut self) {
        let symbols = vec![
            ("0000", "●"),
            ("0001", "■"),
            ("0010", "▲"),
            ("0011", "♦"),
            ("0100", "★"),
            ("0101", "◆"),
            ("0110", "▼"),
            ("0111", "◀"),
            ("1000", "▶"),
            ("1001", "⬟"),
            ("1010", "⬢"),
            ("1011", "⬡"),
            ("1100", "⭘"),
            ("1101", "⭗"),
            ("1110", "⭖"),
            ("1111", "⭕"),
        ];

        for (key, symbol) in symbols {
            self.symbol_lut.insert(key.to_string(), symbol.to_string());
        }
    }

    /// Initialize animation lookup table
    fn initialize_animation_lut(&mut self) {
        let animations = vec![
            ("00", "fade"),
            ("01", "slide"),
            ("10", "rotate"),
            ("11", "pulse"),
        ];

        for (key, animation) in animations {
            self.animation_lut
                .insert(key.to_string(), animation.to_string());
        }
    }

    /// Extract visual properties from SCH (positions 1-16)
    #[must_use]
    pub fn extract_visual_properties(&self, sch: &str) -> VisualProperties {
        if sch.len() < 15 {
            return VisualProperties::default();
        }

        VisualProperties {
            primary_color: self.hash_to_color(&sch[0..3]),
            secondary_color: self.hash_to_color(&sch[4..7]),
            symbol_set: self.hash_to_symbol(&sch[8..12]),
            icon_style: self.extract_icon_style(&sch[12..15]),
        }
    }

    /// Extract animation properties from CUID (positions 17-32)
    #[must_use]
    pub fn extract_animation_properties(&self, cuid: &str) -> AnimationProperties {
        if cuid.len() < 11 {
            return AnimationProperties::default();
        }

        AnimationProperties {
            animation_type: self.hash_to_animation(&cuid[0..2]),
            duration: self.extract_duration(&cuid[3..5]),
            easing: self.extract_easing(&cuid[6..8]),
            loop_behavior: self.extract_looping(&cuid[9..11]),
        }
    }

    /// Hash to color mapping
    fn hash_to_color(&self, hash_segment: &str) -> String {
        let binary = self.hash_to_binary(hash_segment, 3);
        self.color_lut
            .get(&binary)
            .unwrap_or(&"#FFFFFF".to_string())
            .clone()
    }

    /// Hash to symbol mapping
    fn hash_to_symbol(&self, hash_segment: &str) -> String {
        let binary = self.hash_to_binary(hash_segment, 4);
        self.symbol_lut
            .get(&binary)
            .unwrap_or(&"●".to_string())
            .clone()
    }

    /// Hash to animation mapping
    fn hash_to_animation(&self, hash_segment: &str) -> String {
        let binary = self.hash_to_binary(hash_segment, 2);
        self.animation_lut
            .get(&binary)
            .unwrap_or(&"fade".to_string())
            .clone()
    }

    /// Convert hash segment to binary representation
    fn hash_to_binary(&self, hash_segment: &str, bits: usize) -> String {
        let mut result = String::new();
        for (i, c) in hash_segment.chars().enumerate() {
            if i >= bits {
                break;
            }
            let bit = match c.to_ascii_lowercase() {
                '0'..='9' => (c as u8 - b'0') % 2,
                'a'..='z' => (c as u8 - b'a') % 2,
                _ => 0,
            };
            result.push_str(&bit.to_string());
        }
        while result.len() < bits {
            result.push('0');
        }
        result
    }

    /// Extract icon style from hash
    fn extract_icon_style(&self, hash_segment: &str) -> String {
        let styles = ["outline", "filled", "gradient", "textured"];
        let index = hash_segment.chars().next().unwrap_or('0') as usize % styles.len();
        styles[index].to_string()
    }

    /// Extract duration from hash (in milliseconds)
    fn extract_duration(&self, hash_segment: &str) -> u32 {
        let base_duration = 500;
        let multiplier = hash_segment.chars().next().unwrap_or('0') as u32 % 8 + 1;
        base_duration * multiplier
    }

    /// Extract easing function from hash
    fn extract_easing(&self, hash_segment: &str) -> String {
        let easings = ["linear", "ease-in", "ease-out", "ease-in-out"];
        let index = hash_segment.chars().next().unwrap_or('0') as usize % easings.len();
        easings[index].to_string()
    }

    /// Extract loop behavior from hash
    fn extract_looping(&self, hash_segment: &str) -> String {
        let behaviors = ["none", "infinite", "once", "bounce"];
        let index = hash_segment.chars().next().unwrap_or('0') as usize % behaviors.len();
        behaviors[index].to_string()
    }
}

impl Default for HashIsUISystem {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for VisualProperties {
    fn default() -> Self {
        Self {
            primary_color: "#FFFFFF".to_string(),
            secondary_color: "#000000".to_string(),
            symbol_set: "●".to_string(),
            icon_style: "outline".to_string(),
        }
    }
}

impl Default for AnimationProperties {
    fn default() -> Self {
        Self {
            animation_type: "fade".to_string(),
            duration: 500,
            easing: "linear".to_string(),
            loop_behavior: "none".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_lut_initialization() {
        let mut system = HashIsUISystem::new();
        system.initialize_lut_systems().await.unwrap();

        assert!(system.initialized);
        assert!(!system.color_lut.is_empty());
        assert!(!system.symbol_lut.is_empty());
        assert!(!system.animation_lut.is_empty());
    }

    #[test]
    fn test_visual_properties_extraction() {
        let mut system = HashIsUISystem::new();
        system
            .color_lut
            .insert("000".to_string(), "#FF0000".to_string());
        system
            .symbol_lut
            .insert("0000".to_string(), "●".to_string());

        let sch = "000111000011122";
        let props = system.extract_visual_properties(sch);

        assert_eq!(props.primary_color, "#FF0000");
        assert_eq!(props.symbol_set, "●");
    }
}
