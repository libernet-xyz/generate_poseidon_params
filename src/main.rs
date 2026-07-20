use anyhow::Result;
use primitive_types::U256;
use std::fs::File;
use std::io::Write;

mod v2 {
    pub mod bls12_381;
    pub mod bluesky;
}

mod utils;

const NUM_ROUNDS: usize = 8 + 56;

fn write_constants<const N: usize, const M: usize>(
    values: &Vec<Vec<U256>>,
    file_name: &str,
) -> Result<()> {
    assert_eq!(values.len(), N);
    let mut file = File::create(file_name)?;
    for i in 0..N {
        assert_eq!(values[i].len(), M);
        for j in 0..M {
            file.write(values[i][j].to_little_endian().as_slice())?;
        }
    }
    Ok(())
}

fn write_constants_v2_t3() -> Result<()> {
    write_constants::<NUM_ROUNDS, 3>(&*v2::bls12_381::RC3, "out/v2/arc_t3.bin")?;
    write_constants::<3, 3>(&*v2::bls12_381::FL3, "out/v2/fl_t3.bin")?;
    write_constants::<3, 3>(&*v2::bls12_381::PL3, "out/v2/pl_t3.bin")?;
    Ok(())
}

fn write_constants_v2_t4() -> Result<()> {
    write_constants::<NUM_ROUNDS, 4>(&*v2::bls12_381::RC4, "out/v2/arc_t4.bin")?;
    write_constants::<4, 4>(&*v2::bls12_381::FL4, "out/v2/fl_t4.bin")?;
    write_constants::<4, 4>(&*v2::bls12_381::PL4, "out/v2/pl_t4.bin")?;
    Ok(())
}

fn main() -> Result<()> {
    write_constants_v2_t3()?;
    write_constants_v2_t4()?;
    Ok(())
}
