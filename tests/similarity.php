<?php

$dir = __DIR__ . '/fixtures/images';

$index = new ImageHashIndex();

assert($index->addImage('base', $dir . '/base.png') === true);
assert($index->addImage('similar1', $dir . '/similar1.png') === true);
assert($index->addImage('similar2', $dir . '/similar2.png') === true);
assert($index->addImage('different', $dir . '/different.png') === true);

assert($index->count() === 4);

$matches = $index->searchImage($dir . '/base.png', 64, 10);

assert(count($matches) >= 1);
assert($matches[0]['id'] === 'base');
assert((int)$matches[0]['distance'] === 0);

echo "similarity: ok\n";