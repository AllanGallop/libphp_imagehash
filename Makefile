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

release: clean stubs build test

clean:
	cargo clean