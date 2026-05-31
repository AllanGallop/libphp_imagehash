<?php

$index = new ImageHashIndex();

$stats = $index->stats();

assert($stats['records'] === '0');
assert($stats['bucket_maps'] === '4');
assert($stats['used_buckets'] === '0');
assert($stats['bucket_entries'] === '0');

$index->add('img1', 'aaaaaaaaaaaaaaaa');
$index->add('img2', 'aaaaaaaaaaaaaaab');

$stats = $index->stats();

assert($stats['records'] === '2');
assert($stats['bucket_maps'] === '4');

// Each hash is inserted into 4 bucket maps.
assert($stats['bucket_entries'] === '8');

assert((int) $stats['used_buckets'] >= 4);
assert((float) $stats['avg_bucket_size'] >= 1.0);
assert((int) $stats['max_bucket_size'] >= 1);

print_r($stats);

echo "stats: ok\n";