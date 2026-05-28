<?php

@mkdir(__DIR__ . '/outputs', 0777, true);

$path = __DIR__ . '/outputs/index.bin';

$index = new ImageHashIndex();

assert($index->add('img1', 'aaaaaaaaaaaaaaaa') === true);
assert($index->add('img2', 'aaaaaaaaaaaaaaab') === true);
assert($index->save($path) === true);
assert(file_exists($path));

$loaded = new ImageHashIndex();

assert($loaded->loadFromFile($path) === true);
assert($loaded->count() === 2);

$matches = $loaded->search('aaaaaaaaaaaaaaaa', 4, 10);

assert(count($matches) === 2);
assert($matches[0]['id'] === 'img1');
assert((int)$matches[0]['distance'] === 0);

echo "persistence: ok\n";