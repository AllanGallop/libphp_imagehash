use ext_php_rs::prelude::*;
use std::collections::{HashMap, HashSet};
use std::cmp::Ordering;
use serde::{Deserialize, Serialize};
use image::imageops::FilterType;
use rustdct::DctPlanner;

const INDEX_VERSION: u32 = 1;
const HASH_TYPE: &str = "dhash64";
const MAX_SAFE_BUCKET_DISTANCE: u32 = 64;

#[derive(Serialize, Deserialize)]
struct SavedIndex {
    version: u32,
    hash_type: String,
    records: Vec<(String, u64)>,
}

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

    #[php(name = "addImage")]
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

        if max_distance > MAX_SAFE_BUCKET_DISTANCE {
            return Vec::new();
        }

        let mut candidate_set: HashSet<usize> = HashSet::new();

        for part in 0..4 {
            let key = bucket_key(query, part);

            if let Some(indexes) = self.buckets[part].get(&key) {
                for idx in indexes {
                    candidate_set.insert(*idx);
                }
            }
        }

        let mut matches: Vec<(usize, u32)> = candidate_set
            .into_iter()
            .filter_map(|idx| {
                let (_id, candidate) = &self.records[idx];
                let distance = (query ^ *candidate).count_ones();

                if distance <= max_distance {
                    Some((idx, distance))
                } else {
                    None
                }
            })
            .collect();

        if matches.len() > limit {
            matches.select_nth_unstable_by_key(
                limit,
                |(_, distance)| *distance,
            );

            matches.truncate(limit);
        }

        matches.sort_by_key(|(_, distance)| *distance);

        matches
            .into_iter()
            .map(|(idx, distance)| {
                let id = &self.records[idx].0;

                vec![
                    ("id".to_string(), id.clone()),
                    ("distance".to_string(), distance.to_string()),
                ]
            })
            .collect()
    }

    #[php(name = "searchImage")]
    pub fn search_image(&self, path: String, max_distance: u32, limit: usize) -> Vec<Vec<(String, String)>> {
        let Ok(hash) = dhash_file(&path) else {
            return Vec::new();
        };

        self.search(hash_to_hex(hash), max_distance, limit)
    }

    pub fn save(&self, path: String) -> bool {
        let saved = SavedIndex {
            version: INDEX_VERSION,
            hash_type: HASH_TYPE.to_string(),
            records: self.records.clone(),
        };

        let Ok(bytes) = bincode::serialize(&saved) else {
            return false;
        };

        let tmp_path = format!("{}.tmp", path);

        if std::fs::write(&tmp_path, bytes).is_err() {
            return false;
        }

        std::fs::rename(tmp_path, path).is_ok()
    }

    #[php(name = "loadFromFile")]
    pub fn load_from_file(&mut self, path: String) -> bool {
        let Ok(bytes) = std::fs::read(path) else {
            return false;
        };

        let Ok(saved) = bincode::deserialize::<SavedIndex>(&bytes) else {
            return false;
        };

        if saved.version != INDEX_VERSION {
            return false;
        }

        if saved.hash_type != HASH_TYPE {
            return false;
        }

        self.records = saved.records;
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

fn phash_file(path: &str) -> Result<u64, image::ImageError> {
    let img = image::open(path)?;

    let gray = img
        .resize_exact(32, 32, FilterType::Triangle)
        .to_luma8();

    let mut matrix = vec![vec![0f32; 32]; 32];

    for y in 0..32 {
        for x in 0..32 {
            matrix[y][x] = gray.get_pixel(x as u32, y as u32)[0] as f32;
        }
    }

    let mut planner = DctPlanner::new();
    let dct = planner.plan_dct2(32);

    for row in matrix.iter_mut() {
        dct.process_dct2(row);
    }

    for x in 0..32 {
        let mut col: Vec<f32> = (0..32).map(|y| matrix[y][x]).collect();
        dct.process_dct2(&mut col);

        for y in 0..32 {
            matrix[y][x] = col[y];
        }
    }

    let mut values = Vec::with_capacity(64);

    for y in 0..8 {
        for x in 0..8 {
            if x == 0 && y == 0 {
                continue;
            }

            values.push(matrix[y][x]);
        }
    }

    let mut median_values = values.clone();
    let mid = median_values.len() / 2;

    let (_, median, _) = median_values.select_nth_unstable_by(mid, |a, b| {
        a.partial_cmp(b).unwrap_or(Ordering::Equal)
    });

    let median = *median;

    let mut hash: u64 = 0;

    for value in values {
        hash <<= 1;

        if value > median {
            hash |= 1;
        }
    }

    Ok(hash)
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
pub fn image_phash(path: String) -> String {
    match phash_file(&path) {
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
        .function(wrap_function!(image_phash))
        .function(wrap_function!(imagehash_version))
}