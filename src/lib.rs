use ext_php_rs::prelude::*;
use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};
use image::imageops::FilterType;

#[php_class]
#[derive(Serialize, Deserialize)]
pub struct ImageHashIndex {
    records: Vec<(String, u64)>,
    buckets: Vec<HashMap<u16, Vec<usize>>>,
}

#[php_impl]
impl ImageHashIndex {
    pub fn __construct() -> Self {
        Self {
            records: Vec::new(),
            buckets: vec![
                HashMap::new(),
                HashMap::new(),
                HashMap::new(),
                HashMap::new(),
            ],
        }
    }

    pub fn add(&mut self, id: String, hash: String) -> bool {
        match parse_hash(&hash) {
            Ok(value) => {
                let index = self.records.len();
                self.records.push((id, value));

                for part in 0..4 {
                    let key = bucket_key(value, part);

                    self.buckets[part]
                        .entry(key)
                        .or_insert_with(Vec::new)
                        .push(index);
                }

                true
            }
            Err(_) => false,
        }
    }

    pub fn add_image(&mut self, id: String, path: String) -> bool {
        let Ok(hash) = dhash_file(&path) else {
            return false;
        };

        let index = self.records.len();
        self.records.push((id, hash));

        for part in 0..4 {
            let key = bucket_key(hash, part);

            self.buckets[part]
                .entry(key)
                .or_insert_with(Vec::new)
                .push(index);
        }

        true
    }

    pub fn count(&self) -> usize {
        self.records.len()
    }

    pub fn search(&self, hash: String, max_distance: u32, limit: usize) -> Vec<Vec<(String, String)>> {
        let Ok(query) = parse_hash(&hash) else {
            return Vec::new();
        };

        let mut candidate_set: HashSet<usize> = HashSet::new();

        for part in 0..4 {
            let key = bucket_key(query, part);

            if let Some(indexes) = self.buckets[part].get(&key) {
                for idx in indexes {
                    candidate_set.insert(*idx);
                }
            }
        }

        let mut matches: Vec<(String, u32)> = candidate_set
            .into_iter()
            .filter_map(|idx| {
                let (id, candidate) = &self.records[idx];
                let distance = (query ^ *candidate).count_ones();

                if distance <= max_distance {
                    Some((id.clone(), distance))
                } else {
                    None
                }
            })
            .collect();

        matches.sort_by_key(|(_, distance)| *distance);
        matches.truncate(limit);

        matches
            .into_iter()
            .map(|(id, distance)| {
                vec![
                    ("id".to_string(), id),
                    ("distance".to_string(), distance.to_string()),
                ]
            })
            .collect()
    }

    pub fn search_image(&self, path: String, max_distance: u32, limit: usize) -> Vec<Vec<(String, String)>> {
        let Ok(hash) = dhash_file(&path) else {
            return Vec::new();
        };

        self.search(hash_to_hex(hash), max_distance, limit)
    }

    pub fn save(&self, path: String) -> bool {
        let Ok(bytes) = bincode::serialize(&self.records) else {
            return false;
        };

        std::fs::write(path, bytes).is_ok()
    }

    pub fn load_from_file(&mut self, path: String) -> bool {
        let Ok(bytes) = std::fs::read(path) else {
            return false;
        };

        let Ok(records) = bincode::deserialize::<Vec<(String, u64)>>(&bytes) else {
            return false;
        };

        self.records = records;
        self.rebuild_buckets();

        true
    }

    fn rebuild_buckets(&mut self) {
        self.buckets = vec![
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
        ];

        for (idx, (_id, hash)) in self.records.iter().enumerate() {
            for part in 0..4 {
                let key = bucket_key(*hash, part);

                self.buckets[part]
                    .entry(key)
                    .or_insert_with(Vec::new)
                    .push(idx);
            }
        }
    }
}

fn parse_hash(hash: &str) -> Result<u64, std::num::ParseIntError> {
    let cleaned = hash
        .trim()
        .trim_start_matches("0x")
        .trim_start_matches("0X");

    u64::from_str_radix(cleaned, 16)
}

fn bucket_key(hash: u64, part: usize) -> u16 {
    let shift = 48 - (part * 16);
    ((hash >> shift) & 0xffff) as u16
}

fn dhash_file(path: &str) -> Result<u64, image::ImageError> {
    let img = image::open(path)?;

    let gray = img
        .resize_exact(9, 8, FilterType::Triangle)
        .to_luma8();

    let mut hash: u64 = 0;

    for y in 0..8 {
        for x in 0..8 {
            let left = gray.get_pixel(x, y)[0];
            let right = gray.get_pixel(x + 1, y)[0];

            hash <<= 1;

            if left > right {
                hash |= 1;
            }
        }
    }

    Ok(hash)
}

fn hash_to_hex(hash: u64) -> String {
    format!("{:016x}", hash)
}

#[php_function]
pub fn hamming_distance(a: String, b: String) -> u32 {
    let Ok(a) = parse_hash(&a) else {
        return u32::MAX;
    };

    let Ok(b) = parse_hash(&b) else {
        return u32::MAX;
    };

    (a ^ b).count_ones()
}

#[php_function]
pub fn image_dhash(path: String) -> String {
    match dhash_file(&path) {
        Ok(hash) => hash_to_hex(hash),
        Err(_) => "".to_string(),
    }
}

#[php_function]
pub fn imagehash_version() -> &'static str {
    "php_imagehash 0.2.0"
}

#[php_module]
pub fn module(module: ModuleBuilder) -> ModuleBuilder {
    module
        .class::<ImageHashIndex>()
        .function(wrap_function!(hamming_distance))
        .function(wrap_function!(image_dhash))
        .function(wrap_function!(imagehash_version))
}