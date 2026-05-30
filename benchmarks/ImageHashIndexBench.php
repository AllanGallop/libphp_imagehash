<?php

class ImageHashIndexBench
{
    private array $hashes;

    public function __construct()
    {
        $images = glob(__DIR__ . '/fixtures/images/*.{png,jpg,jpeg}', GLOB_BRACE);

        if (!$images) {
            throw new RuntimeException(
                'No benchmark images found. Run: php benchmarks/generate_images.php 500'
            );
        }

        $this->hashes = [];

        foreach ($images as $i => $image) {
            $this->hashes[] = [
                'id' => 'img_' . $i,
                'hash' => image_dhash($image),
            ];
        }
    }

    /**
     * @Revs(3)
     * @Iterations(3)
     */
    public function benchAddHashes(): void
    {
        $index = new ImageHashIndex();

        foreach ($this->hashes as $row) {
            $index->add($row['id'], $row['hash']);
        }
    }

    /**
     * @Revs(3)
     * @Iterations(3)
     */
    public function benchSearchHashes(): void
    {
        $index = new ImageHashIndex();

        foreach ($this->hashes as $row) {
            $index->add($row['id'], $row['hash']);
        }

        foreach ($this->hashes as $row) {
            $index->search($row['hash'], 8, 20);
        }
    }

    /**
     * @Revs(3)
     * @Iterations(3)
     */
    public function benchAddAndSearchHashes(): void
    {
        $index = new ImageHashIndex();

        foreach ($this->hashes as $row) {
            $index->add($row['id'], $row['hash']);
            $index->search($row['hash'], 8, 20);
        }
    }

    /**
     * @Revs(3)
     * @Iterations(3)
     */
    public function benchSaveLoadIndex(): void
    {
        $path = __DIR__ . '/../tests/outputs/bench-index.bin';

        @mkdir(dirname($path), 0777, true);

        $index = new ImageHashIndex();

        foreach ($this->hashes as $row) {
            $index->add($row['id'], $row['hash']);
        }

        $index->save($path);

        $loaded = new ImageHashIndex();
        $loaded->loadFromFile($path);
    }
}