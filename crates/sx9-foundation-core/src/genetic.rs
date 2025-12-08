use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use blake2::{Blake2b512, Digest};
use murmurhash3::murmurhash3_x86_32;

const TEELINE_MAP: &[(&str, &str)] = &[
    ("and", "&"), ("the", "þ"), ("that", "ð"), ("with", "w/"), ("for", "4"),
    ("have", "h^"), ("this", "ðs"), ("will", "wl"), ("your", "yr"), ("they", "ðy"),
];

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()_+-=";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentMapping {
    pub crate_name: String,
    pub component_id: String,
    pub mapping_type: String,
    pub priority: String,
    pub page_id: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrivariateMurmur {
    pub primary: String,
    pub secondary: String,
    pub composite: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RDFTriple {
    pub subject: String,
    pub predicate: String,
    pub object: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct USIM {
    pub genetic_hashes: HashMap<String, String>,
    pub trivariate_sets: HashMap<String, TrivariateMurmur>,
    pub rdf_triples: Vec<RDFTriple>,
    pub lisp_operators: Vec<String>,
    pub compressed_meta: HashMap<String, String>,
}

pub struct GeneticMetaProgramming {
    mappings: Vec<ComponentMapping>,
}

impl GeneticMetaProgramming {
    pub fn new(mappings: Vec<ComponentMapping>) -> Self {
        Self { mappings }
    }

    pub fn blake_qa_hash(&self, mapping: &ComponentMapping, qa_iteration: u32) -> String {
        let input = format!("{}:{}:qa{}", mapping.crate_name, 
                           mapping.component_id, qa_iteration);
        let mut hasher = Blake2b512::new();
        hasher.update(input.as_bytes());
        hex::encode(&hasher.finalize()[..16])
    }

    pub fn trivariate_murmur_hash(&self, var1: &str, var2: &str, var3: &str) 
        -> TrivariateMurmur {
        let seed1 = murmurhash3_x86_32(var1.as_bytes(), 42);
        let primary = self.generate_charset_hash(seed1, 16);
        
        let seed2 = murmurhash3_x86_32(var2.as_bytes(), 84);
        let secondary = self.generate_charset_hash(seed2, 16);
        
        let composite_input = format!("{}:{}:{}", var1, var2, var3);
        let seed3 = murmurhash3_x86_32(composite_input.as_bytes(), 126);
        let composite = self.generate_charset_hash(seed3, 16);

        TrivariateMurmur { primary, secondary, composite }
    }

    fn generate_charset_hash(&self, seed: u32, length: usize) -> String {
        let mut result = String::with_capacity(length);
        let mut current_seed = seed;
        
        for _ in 0..length {
            let index = (current_seed as usize) % CHARSET.len();
            result.push(CHARSET[index] as char);
            current_seed = current_seed.wrapping_mul(1103515245).wrapping_add(12345);
        }
        result
    }

    pub fn teeline_compress(&self, text: &str) -> String {
        let mut compressed = text.to_lowercase();
        for (word, shorthand) in TEELINE_MAP {
            compressed = compressed.replace(word, shorthand);
        }
        compressed
    }

    pub fn generate_usim(&self) -> USIM {
        let mut usim = USIM {
            genetic_hashes: HashMap::new(),
            trivariate_sets: HashMap::new(),
            rdf_triples: Vec::new(),
            lisp_operators: Vec::new(),
            compressed_meta: HashMap::new(),
        };

        for (index, mapping) in self.mappings.iter().enumerate() {
            let mapping_id = format!("m{}", index);
            
            let genetic_hash = self.blake_qa_hash(mapping, 1);
            usim.genetic_hashes.insert(mapping_id.clone(), genetic_hash);
            
            let trivariate = self.trivariate_murmur_hash(
                &mapping.crate_name,
                &mapping.component_id,
                &mapping.mapping_type,
            );
            usim.trivariate_sets.insert(mapping_id.clone(), trivariate);
            
            usim.rdf_triples.push(RDFTriple {
                subject: mapping.crate_name.clone(),
                predicate: "maps_to".to_string(),
                object: mapping.component_id.clone(),
            });
            
            usim.lisp_operators.push(format!(
                "(map '{}' '{}' :type '{}')",
                mapping.crate_name, mapping.component_id, mapping.mapping_type
            ));
            
            let description = mapping.description.as_deref()
                .unwrap_or("component maps crate functionality");
            let compressed = self.teeline_compress(description);
            usim.compressed_meta.insert(mapping_id, compressed);
        }

        usim
    }

    pub fn generate_noun_verb_comments(&self, total_loc: usize) -> Vec<String> {
        let target_lines = (total_loc as f32 * 0.3) as usize;
        let mut comments = Vec::with_capacity(target_lines);
        
        let patterns = [
            "// component-maps-crate-system",
            "// hash-generates-fingerprint-signature", 
            "// system-processes-mapping-data",
            "// trivariate-creates-murmur-hash",
            "// usim-contains-rdf-triples",
            "// lisp-operators-define-mappings",
            "// teeline-compression-reduces-metadata",
            "// genetic-evolution-improves-hashes",
        ];
        
        for i in 0..target_lines {
            let pattern_index = i % patterns.len();
            comments.push(patterns[pattern_index].to_string());
        }
        
        comments
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_genetic_meta_programming() {
        let mappings = vec![ComponentMapping {
            crate_name: "ctas7-hashing-engine".to_string(),
            component_id: "UniversalHashEngine".to_string(),
            mapping_type: "Direct".to_string(),
            priority: "P0".to_string(),
            page_id: "hash".to_string(),
            description: Some("Hash engine system".to_string()),
        }];
        
        let meta = GeneticMetaProgramming::new(mappings);
        let usim = meta.generate_usim();
        
        assert!(!usim.genetic_hashes.is_empty());
        assert_eq!(usim.genetic_hashes.get("m0").unwrap().len(), 32);
        
        let trivariate = usim.trivariate_sets.get("m0").unwrap();
        assert_eq!(trivariate.primary.len(), 16);
        assert_eq!(trivariate.secondary.len(), 16);
        assert_eq!(trivariate.composite.len(), 16);
    }
}