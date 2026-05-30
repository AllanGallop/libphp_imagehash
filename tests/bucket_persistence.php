<?php

@mkdir(__DIR__ . '/outputs', 0777, true);

$path = __DIR__ . '/outputs/index.bin';

$index = new ImageHashIndex();

for ($i = 0; $i < 100000; $i++) {
    $hash = sprintf('%016x', random_int(0, PHP_INT_MAX));

    $index->add("id_$i", $hash);
}

$index->save($path);

$loaded = new ImageHashIndex();
$loaded->loadFromFile($path);

assert($loaded->count() === $index->count());

echo "bucket_persistence: ok\n";