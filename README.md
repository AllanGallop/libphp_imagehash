![CI](https://github.com/AllanGallop/libphp_imagehash/actions/workflows/ci.yml/badge.svg)
![Release](https://github.com/AllanGallop/libphp_imagehash/actions/workflows/release.yml/badge.svg)
![License](https://img.shields.io/github/license/AllanGallop/libphp_imagehash)
![Latest Release](https://img.shields.io/github/v/release/AllanGallop/libphp_imagehash)

# php_imagehash

`php_imagehash` is a PHP extension written in Rust that provides perceptual image hashing and fast near-duplicate search using a `dHash` implementation.

## Features

- Compute a 64-bit image hash from a file with `image_dhash()`
- Measure similarity with `hamming_distance()`
- Store and search image hashes in an in-memory index with `ImageHashIndex`
- Save/load index data from disk
- Search by hash string or image file path

## API

### Functions

- `imagehash_version(): string`
  - Returns the compiled extension version string.
- `image_dhash(string $path): string`
  - Computes and returns the `dHash` of an image file as a 16-character hexadecimal string.
  - Returns an empty string on failure.
- `hamming_distance(string $a, string $b): int`
  - Computes the Hamming distance between two hex hashes.
  - Returns `4294967295` (`PHP_INT_MAX` equivalent for an invalid parse) if either hash is invalid.

### Class: `ImageHashIndex`

Methods:

- `__construct()`
  - Creates a new empty index.
- `add(string $id, string $hash): bool`
  - Adds a hash string to the index with a custom identifier.
  - Returns `true` on success.
- `add_image(string $id, string $path): bool`
  - Computes the image hash for the given file and adds it to the index.
- `count(): int`
  - Returns the number of records stored in the index.
- `search(string $hash, int $max_distance, int $limit): array`
  - Searches for stored hashes within `max_distance` of the given hash.
  - Returns up to `limit` matches.
- `search_image(string $path, int $max_distance, int $limit): array`
  - Computes the hash for the provided image and searches against the index.
- `save(string $path): bool`
  - Serializes index data to a file.
- `loadFromFile(string $path): bool`
  - Loads serialized index data from a file.

## Quick start

If you already have a prebuilt release library, you can load it directly in PHP without building the extension yourself.

1. Copy or install the release library `libphp_imagehash.so` to a location accessible by PHP.
2. Add the library to `php.ini`:

```ini
extension=/path/to/libphp_imagehash.so
```

3. Restart PHP or your web server if needed.

## Build and install

If you want to build from source, use the release library from a Linux/macOS build:

```bash
cargo build --release
```

Then load the shared library from the built release:

```ini
extension=/path/to/php_imagehash/target/release/libphp_imagehash.so
```

## Docker usage

This repository includes a `Dockerfile` that installs PHP and Rust, so you can build and test the extension in a container.

1. Build the image:

```bash
docker build -t php-imagehash .
```

2. Run a container from the image and mount your project if desired.

## Example usage

```php
<?php

echo imagehash_version() . PHP_EOL;

// compute an image hash
$hash = image_dhash('/app/data/images/base.png');
echo "base hash: $hash\n";

// compare two images
$distance = hamming_distance(
    image_dhash('/app/data/images/base.png'),
    image_dhash('/app/data/images/similar1.png')
);
echo "distance: $distance\n";

// build an index
$index = new ImageHashIndex();
$index->add('base', $hash);
$index->add_image('similar1', '/app/data/images/similar1.png');
$index->add_image('different', '/app/data/images/different.png');

// search by hash
$results = $index->search($hash, 4, 10);
print_r($results);

// save and load index
$index->save('/app/data/index.bin');
$loaded = new ImageHashIndex();
$loaded->loadFromFile('/app/data/index.bin');
```

## Notes

- `image_dhash()` currently uses a 9x8 resized grayscale image and computes a 64-bit difference hash.
- The extension stores hashes internally as 64-bit values, and searches are performed by bucketed candidate selection.
- Use `max_distance` to control how close matches must be.

## Repository structure

- `src/lib.rs` - Rust source for the PHP extension
- `Cargo.toml` - Rust crate metadata and dependencies
- `test.php` - example PHP script showing the module API
- `Dockerfile` - container environment for building/testing
- `data/images/` - sample images used by the example script
