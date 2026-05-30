<?php

/**
 * Return extension version.
 */
function imagehash_version(): string {}

/**
 * Calculate Hamming distance between two 64-bit hex hashes.
 */
function hamming_distance(string $a, string $b): int {}

/**
 * Generate a 64-bit dHash for an image file.
 */
function image_dhash(string $path): string {}

class ImageHashIndex
{
    public function __construct() {}

    public function add(string $id, string $hash): bool {}

    public function addImage(string $id, string $path): bool {}

    public function count(): int {}

    /**
     * @return array<int, array{id: string, distance: string}>
     */
    public function search(string $hash, int $maxDistance, int $limit): array {}

    /**
     * @return array<int, array{id: string, distance: string}>
     */
    public function searchImage(string $path, int $maxDistance, int $limit): array {}

    public function save(string $path): bool {}

    public function loadFromFile(string $path): bool {}
}