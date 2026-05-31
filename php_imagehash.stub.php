<?php

// Stubs for php_imagehash

namespace {
    class ImageHashIndex {
        public function __construct() {}

        /**
         * @param string $id
         * @param string $hash
         * @return bool
         */
        public function add(string $id, string $hash): bool {}

        /**
         * @param string $id
         * @param string $path
         * @return bool
         */
        public function addImage(string $id, string $path): bool {}

        /**
         * @return void
         */
        public function clear(): void {}

        /**
         * @return int
         */
        public function count(): int {}

        /**
         * @param string $path
         * @return bool
         */
        public function loadFromFile(string $path): bool {}

        /**
         * @param string $hash
         * @return array
         */
        public function nearest(string $hash): array {}

        /**
         * @param string $path
         * @return array
         */
        public function nearestImage(string $path): array {}

        /**
         * @return void
         */
        public function rebuildBuckets(): void {}

        /**
         * @param string $path
         * @return bool
         */
        public function save(string $path): bool {}

        /**
         * @param string $hash
         * @param int $max_distance
         * @param int $limit
         * @return array
         */
        public function search(string $hash, int $max_distance, int $limit): array {}

        /**
         * @param string $path
         * @param int $max_distance
         * @param int $limit
         * @return array
         */
        public function searchImage(string $path, int $max_distance, int $limit): array {}

        /**
         * @return array
         */
        public function stats(): array {}
    }

    /**
     * @param string $a
     * @param string $b
     * @return int
     */
    function hamming_distance(string $a, string $b): int {}

    /**
     * @param string $path
     * @return string
     */
    function image_dhash(string $path): string {}

    /**
     * @param string $path
     * @return string
     */
    function image_phash(string $path): string {}

    /**
     * @return string
     */
    function imagehash_version(): string {}
}
