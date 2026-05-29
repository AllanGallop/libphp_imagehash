<?php

INI_SET('memory_limit', '3G');

class LargeCandidateBench
{
    private ImageHashIndex $index;
    private string $query = 'aaaaaaaaaaaaaaaa';

    public function __construct()
    {
        $this->index = new ImageHashIndex();

        for ($i = 0; $i < 5000000; $i++) {
            $suffix = str_pad(dechex($i), 12, '0', STR_PAD_LEFT);
            $this->index->add('img_' . $i, 'aaaa' . $suffix);
        }
    }

    /**
     * @Revs(3)
     * @Iterations(5)
     */
    public function benchLargeCandidateSearchLimit20(): void
    {
        $this->index->search($this->query, 64, 20);
    }

    /**
     * @Revs(3)
     * @Iterations(5)
     */
    public function benchLargeCandidateSearchLimit1000(): void
    {
        $this->index->search($this->query, 64, 1000);
    }

    /**
     * @Revs(3)
     * @Iterations(5)
     */
    public function benchLargeCandidateSearchNoLimitEffect(): void
    {
        $this->index->search($this->query, 64, 5000000);
    }
}