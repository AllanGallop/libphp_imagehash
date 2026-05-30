from pathlib import Path
from time import perf_counter
from PIL import Image
import imagehash

images = list(Path("benchmarks/fixtures/images").glob("*.png"))

if not images:
    raise RuntimeError("No benchmark images found")

def bench(name, fn):
    start = perf_counter()

    for image_path in images:
        with Image.open(image_path) as img:
            fn(img)

    elapsed = perf_counter() - start
    print(f"{name}: {elapsed:.3f}s for {len(images)} images ({len(images) / elapsed:.1f} hashes/sec)")

bench("Python ImageHash dHash", lambda img: imagehash.dhash(img))
bench("Python ImageHash pHash", lambda img: imagehash.phash(img))