<?php

$index = new ImageHashIndex();

$index->add('exact', 'aaaaaaaaaaaaaaaa');
$index->add('near', 'aaaaaaaaaaaaaaab');
$index->add('far', 'ffffffffffffffff');

$nearest = $index->nearest('aaaaaaaaaaaaaaaa');

assert(count($nearest) === 1);
assert($nearest[0]['id'] === 'exact');
assert((int) $nearest[0]['distance'] === 0);

$nearest = $index->nearest('aaaaaaaaaaaaaaac');

assert(count($nearest) === 1);
assert(in_array($nearest[0]['id'], ['exact', 'near'], true));

echo "nearest hex: ok\n";

$dir = __DIR__ . '/fixtures/images';

$index = new ImageHashIndex();

assert($index->addImage('base', $dir . '/base.png') === true);
assert($index->addImage('similar1', $dir . '/similar1.png') === true);

$nearest = $index->nearestImage($dir . '/base.png');

assert(count($nearest) === 1);
assert($nearest[0]['id'] === 'base');

echo "nearest image: ok\n";