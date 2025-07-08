use csv::Writer;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;

fn main() -> anyhow::Result<()> {
    let mut rng = ChaCha20Rng::from_os_rng();
    let mut wtr = Writer::from_path("output.csv")?;
    wtr.write_record(&["index", "number"])?;

    for i in 0..3_000_000u32 {
        let number: u32 = rng.random();
        wtr.write_record(&[i.to_string(), number.to_string()])?;
    }
    wtr.flush()?;
    Ok(())
}
