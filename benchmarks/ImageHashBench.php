<?php

use Jenssegers\ImageHash\ImageHash;
use Jenssegers\ImageHash\Implementations\DifferenceHash;
use Jenssegers\ImageHash\Implementations\PerceptualHash;

class ImageHashBench
{
    private array $images;
    private ImageHash $jensDhash;
    private ImageHash $jensPhash;

    public function __construct()
    {
        $this->images = glob(__DIR__ . '/fixtures/images/*.{png,jpg,jpeg}', GLOB_BRACE);

        $this->jensDhash = new ImageHash(new DifferenceHash());
        $this->jensPhash = new ImageHash(new PerceptualHash());
    }

    /**
     * @Revs(3)
     * @Iterations(3)
     */
    public function benchRustDhash(): void
    {
        foreach ($this->images as $image) {
            image_dhash($image);
        }
    }

    /**
     * @Revs(3)
     * @Iterations(3)
     */
    public function benchJenssegersDhash(): void
    {
        foreach ($this->images as $image) {
            (string) $this->jensDhash->hash($image);
        }
    }

    /**
     * @Revs(3)
     * @Iterations(3)
     */
    public function benchRustPhash(): void
    {
        foreach ($this->images as $image) {
            image_phash($image);
        }
    }

    /**
     * @Revs(3)
     * @Iterations(3)
     */
    public function benchJenssegersPhash(): void
    {
        foreach ($this->images as $image) {
            (string) $this->jensPhash->hash($image);
        }
    }
}