<?php

$count = (int)($argv[1] ?? 1000);
$dir = __DIR__ . '/fixtures/images';

@mkdir($dir, 0777, true);

for ($i = 0; $i < $count; $i++) {
    $img = imagecreatetruecolor(256, 256);

    mt_srand($i);

    for ($y = 0; $y < 256; $y++) {
        for ($x = 0; $x < 256; $x++) {
            $v = mt_rand(0, 255);
            $c = imagecolorallocate($img, $v, $v, $v);
            imagesetpixel($img, $x, $y, $c);
        }
    }

    imagepng($img, sprintf('%s/img_%05d.png', $dir, $i));
    imagedestroy($img);
}

echo "Generated {$count} images in {$dir}\n";