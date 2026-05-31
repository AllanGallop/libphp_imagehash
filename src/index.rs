use ext_php_rs::prelude::*;
use crate::hashing::{dhash_file, hash_to_hex, parse_hash};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

const INDEX_VERSION: u32 = 1;
const HASH_TYPE: &str = "dhash64";
const MAX_SAFE_BUCKET_DISTANCE: u32 = 64;

#[php_class]
#[derive(Serialize, Deserialize)]
pub struct ImageHashIndex {
    records: Vec<(String, u64)>,
    buckets: Vec<HashMap<u16, Vec<usize>>>,
}

#[derive(Serialize, Deserialize)]
struct SavedIndex {
    version: u32,
    hash_type: String,
    records: Vec<(String, u64)>,
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

    pub fn search_image(&self, path: String, max_distance: u32, limit: usize) -> Vec<Vec<(String, String)>> {
        let Ok(hash) = dhash_file(&path) else {
            return Vec::new();
        };

        self.search(hash_to_hex(hash), max_distance, limit)
    }

    pub fn nearest(&self, hash: String) -> Vec<Vec<(String, String)>> {
        self.search(hash, MAX_SAFE_BUCKET_DISTANCE, 1)
    }

    pub fn nearest_image(&self, path: String) -> Vec<Vec<(String, String)>> {
        self.search_image(path, MAX_SAFE_BUCKET_DISTANCE, 1)
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

    pub fn clear(&mut self) {
        self.records.clear();
        self.buckets = vec![
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
        ];
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

        pub fn count(&self) -> usize {
        self.records.len()
    }

    pub fn stats(&self) -> Vec<(String, String)> {
        let bucket_count: usize = self.buckets.iter().map(|b| b.len()).sum();

        let mut total_bucket_entries = 0usize;
        let mut max_bucket_size = 0usize;

        for bucket_map in &self.buckets {
            for indexes in bucket_map.values() {
                total_bucket_entries += indexes.len();
                max_bucket_size = max_bucket_size.max(indexes.len());
            }
        }

        let avg_bucket_size = if bucket_count == 0 {
            0.0
        } else {
            total_bucket_entries as f64 / bucket_count as f64
        };

        vec![
            ("records".to_string(), self.records.len().to_string()),
            ("bucket_maps".to_string(), self.buckets.len().to_string()),
            ("used_buckets".to_string(), bucket_count.to_string()),
            ("bucket_entries".to_string(), total_bucket_entries.to_string()),
            ("avg_bucket_size".to_string(), format!("{:.2}", avg_bucket_size)),
            ("max_bucket_size".to_string(), max_bucket_size.to_string()),
        ]
    }
}

fn bucket_key(hash: u64, part: usize) -> u16 {
    let shift = 48 - (part * 16);
    ((hash >> shift) & 0xffff) as u16
}
