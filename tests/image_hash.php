<?php

$base = __DIR__ . '/fixtures/images/base.png';
$similar1 = __DIR__ . '/fixtures/images/similar1.png';

assert(file_exists($base));
assert(file_exists($similar1));

$hashBase = image_dhash($base);
$hashSimilar = image_dhash($similar1);

assert(strlen($hashBase) === 16);
assert(strlen($hashSimilar) === 16);

$distance = hamming_distance($hashBase, $hashSimilar);

echo "base hash: {$hashBase}\n";
echo "similar1 hash: {$hashSimilar}\n";
echo "distance: {$distance}\n";

assert($distance >= 0);
assert($distance <= 64);

echo "image_hash: ok\n";