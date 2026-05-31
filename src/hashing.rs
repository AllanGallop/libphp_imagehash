use std::cmp::Ordering;
use image::imageops::FilterType;
use rustdct::DctPlanner;

pub fn parse_hash(hash: &str) -> Result<u64, std::num::ParseIntError> {
    let cleaned = hash
        .trim()
        .trim_start_matches("0x")
        .trim_start_matches("0X");

    u64::from_str_radix(cleaned, 16)
}

pub fn dhash_file(path: &str) -> Result<u64, image::ImageError> {
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

pub fn phash_file(path: &str) -> Result<u64, image::ImageError> {
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

pub fn hash_to_hex(hash: u64) -> String {
    format!("{:016x}", hash)
}

pub(crate) fn hamming_distance_hex(a: String, b: String) -> u32 {
    let Ok(a) = parse_hash(&a) else {
        return u32::MAX;
    };

    let Ok(b) = parse_hash(&b) else {
        return u32::MAX;
    };

    (a ^ b).count_ones()
}

pub(crate) fn image_dhash_hex(path: String) -> String {
    match dhash_file(&path) {
        Ok(hash) => hash_to_hex(hash),
        Err(_) => "".to_string(),
    }
}

pub(crate) fn image_phash_hex(path: String) -> String {
    match phash_file(&path) {
        Ok(hash) => hash_to_hex(hash),
        Err(_) => "".to_string(),
    }
}
