use anyhow::Result;
use primitive_types::U256;
use std::fs::File;
use std::io::Write;

mod v1 {
    pub mod bls12_381;
    pub mod bluesky;
    pub mod goldilocks;
}

mod v2 {
    pub mod bls12_381;
    pub mod bluesky;
    pub mod goldilocks;
}

mod utils;

const NUM_ROUNDS: usize = 8 + 56;

fn write_constants_u64<const N: usize, const M: usize>(
    values: &Vec<Vec<u64>>,
    file_name: &str,
) -> Result<()> {
    assert_eq!(values.len(), N);
    let mut file = File::create(file_name)?;
    for i in 0..N {
        assert_eq!(values[i].len(), M);
        for j in 0..M {
            file.write(values[i][j].to_le_bytes().as_slice())?;
        }
    }
    Ok(())
}

fn write_constants_u256<const N: usize, const M: usize>(
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

fn write_constants_v1_bls12_381_t3() -> Result<()> {
    write_constants_u256::<NUM_ROUNDS, 3>(&*v1::bls12_381::RC3, "out/v1/bls12_381/arc_t3.bin")?;
    write_constants_u256::<3, 3>(&*v1::bls12_381::MDS3, "out/v1/bls12_381/mds_t3.bin")?;
    Ok(())
}

fn write_constants_v1_bls12_381_t4() -> Result<()> {
    write_constants_u256::<NUM_ROUNDS, 4>(&*v1::bls12_381::RC4, "out/v1/bls12_381/arc_t4.bin")?;
    write_constants_u256::<4, 4>(&*v1::bls12_381::MDS4, "out/v1/bls12_381/mds_t4.bin")?;
    Ok(())
}

fn write_constants_v1_bluesky_t3() -> Result<()> {
    write_constants_u256::<NUM_ROUNDS, 3>(&*v1::bluesky::RC3, "out/v1/bluesky/arc_t3.bin")?;
    write_constants_u256::<3, 3>(&*v1::bluesky::MDS3, "out/v1/bluesky/mds_t3.bin")?;
    Ok(())
}

fn write_constants_v1_bluesky_t4() -> Result<()> {
    write_constants_u256::<NUM_ROUNDS, 4>(&*v1::bluesky::RC4, "out/v1/bluesky/arc_t4.bin")?;
    write_constants_u256::<4, 4>(&*v1::bluesky::MDS4, "out/v1/bluesky/mds_t4.bin")?;
    Ok(())
}

fn write_constants_v1_goldilocks_t12() -> Result<()> {
    write_constants_u64::<30, 12>(&*v1::goldilocks::RC12, "out/v1/goldilocks/arc_t12.bin")?;
    write_constants_u64::<12, 12>(&*v1::goldilocks::MDS12, "out/v1/goldilocks/mds_t12.bin")?;
    Ok(())
}

fn write_constants_v1_goldilocks_t16() -> Result<()> {
    write_constants_u64::<30, 16>(&*v1::goldilocks::RC16, "out/v1/goldilocks/arc_t16.bin")?;
    write_constants_u64::<16, 16>(&*v1::goldilocks::MDS16, "out/v1/goldilocks/mds_t16.bin")?;
    Ok(())
}

fn write_constants_v2_bls12_381_t3() -> Result<()> {
    write_constants_u256::<NUM_ROUNDS, 3>(&*v2::bls12_381::RC3, "out/v2/bls12_381/arc_t3.bin")?;
    write_constants_u256::<3, 3>(&*v2::bls12_381::FL3, "out/v2/bls12_381/fl_t3.bin")?;
    write_constants_u256::<3, 3>(&*v2::bls12_381::PL3, "out/v2/bls12_381/pl_t3.bin")?;
    Ok(())
}

fn write_constants_v2_bls12_381_t4() -> Result<()> {
    write_constants_u256::<NUM_ROUNDS, 4>(&*v2::bls12_381::RC4, "out/v2/bls12_381/arc_t4.bin")?;
    write_constants_u256::<4, 4>(&*v2::bls12_381::FL4, "out/v2/bls12_381/fl_t4.bin")?;
    write_constants_u256::<4, 4>(&*v2::bls12_381::PL4, "out/v2/bls12_381/pl_t4.bin")?;
    Ok(())
}

fn write_constants_v2_bluesky_t3() -> Result<()> {
    write_constants_u256::<NUM_ROUNDS, 3>(&*v2::bluesky::RC3, "out/v2/bluesky/arc_t3.bin")?;
    write_constants_u256::<3, 3>(&*v2::bluesky::FL3, "out/v2/bluesky/fl_t3.bin")?;
    write_constants_u256::<3, 3>(&*v2::bluesky::PL3, "out/v2/bluesky/pl_t3.bin")?;
    Ok(())
}

fn write_constants_v2_bluesky_t4() -> Result<()> {
    write_constants_u256::<NUM_ROUNDS, 4>(&*v2::bluesky::RC4, "out/v2/bluesky/arc_t4.bin")?;
    write_constants_u256::<4, 4>(&*v2::bluesky::FL4, "out/v2/bluesky/fl_t4.bin")?;
    write_constants_u256::<4, 4>(&*v2::bluesky::PL4, "out/v2/bluesky/pl_t4.bin")?;
    Ok(())
}

fn main() -> Result<()> {
    write_constants_v1_bls12_381_t3()?;
    write_constants_v1_bls12_381_t4()?;
    write_constants_v1_bluesky_t3()?;
    write_constants_v1_bluesky_t4()?;
    write_constants_v1_goldilocks_t12()?;
    write_constants_v1_goldilocks_t16()?;
    write_constants_v2_bls12_381_t3()?;
    write_constants_v2_bls12_381_t4()?;
    write_constants_v2_bluesky_t3()?;
    write_constants_v2_bluesky_t4()?;
    Ok(())
}
