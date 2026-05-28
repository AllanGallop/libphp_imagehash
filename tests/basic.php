<?php

echo imagehash_version() . PHP_EOL;

assert(imagehash_version() !== '');

assert(hamming_distance(
    'aaaaaaaaaaaaaaaa',
    'aaaaaaaaaaaaaaaa'
) === 0);

assert(hamming_distance(
    'aaaaaaaaaaaaaaaa',
    'aaaaaaaaaaaaaaab'
) === 1);

$index = new ImageHashIndex();

assert($index->count() === 0);
assert($index->add('img1', 'aaaaaaaaaaaaaaaa') === true);
assert($index->add('img2', 'aaaaaaaaaaaaaaab') === true);
assert($index->count() === 2);

$matches = $index->search('aaaaaaaaaaaaaaaa', 4, 10);

assert(count($matches) === 2);
assert($matches[0]['id'] === 'img1');
assert((int)$matches[0]['distance'] === 0);

echo "basic: ok\n";