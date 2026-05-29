EXT=target/release/libphp_imagehash.so
PHP=php -d zend.assertions=1 -d assert.exception=1 -d extension=$(EXT)

build:
	cargo build --release

stubs:
	cargo php stubs --stdout > php_imagehash.stub.php

test-basic: build
	$(PHP) tests/basic.php

test-persistence: build
	$(PHP) tests/persistence.php

test-image: build
	$(PHP) tests/image_hash.php

test-similarity: build
	$(PHP) tests/similarity.php

test: test-basic test-persistence test-image test-similarity

bench-generate:
	php benchmarks/generate_images.php 500

bench-index: build
	vendor/bin/phpbench run benchmarks/ImageHashIndexBench.php --report=default

bench: build
	vendor/bin/phpbench run benchmarks --report=default

python-venv:
	python3 -m venv .venv
	. .venv/bin/activate && pip install ImageHash pillow scipy numpy

bench-python:
	. .venv/bin/activate && python benchmarks/python_imagehash_bench.py

bench-full: bench-generate bench

release: clean stubs build test

clean:
	cargo clean