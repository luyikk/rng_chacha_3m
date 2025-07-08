
```rust
/// This example demonstrates how to use the ChaCha20 random number generator
/// to generate random numbers and shuffle a vector.
/// It uses the `rand` and `rand_chacha` crates.
/// # Dependencies
/// ```toml
/// [dependencies]
/// rand = "0.9"
/// rand_chacha = "0.9"
/// # Example Code

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;

fn main(){
/// This example demonstrates how to use the ChaCha20 random number generator
let mut rng = ChaCha20Rng::from_os_rng();
/// Generate a random number using the ChaCha20 random number generator
let number: u32 = rng.random();

/// This example demonstrates how to use the ChaCha20 random number generator
let number: u32 = rng.random_range(0..100);

/// This example demonstrates how to shuffle a vector using the ChaCha20 random number generator
let mut data = vec! [1, 2, 3, 4, 5];
data.shuffle( & mut rand);

}
```