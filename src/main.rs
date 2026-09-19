use anyhow::Result;
use primitive_types::U256;
use std::fs::File;
use std::io::Write;

mod v1 {
    pub mod bls12_381;
    pub mod bluesky;
    pub mod goldilocks;
    pub mod koalabear;
    pub mod schraderbrau;
}

mod v2 {
    pub mod bls12_381;
    pub mod bluesky;
    pub mod goldilocks;
    pub mod schraderbrau;
}

mod utils;

fn write_constants_u32<const N: usize, const M: usize>(
    values: &Vec<Vec<u32>>,
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
    write_constants_u256::<64, 3>(&*v1::bls12_381::RC3, "out/v1/bls12_381/arc_t3.bin")?;
    write_constants_u256::<3, 3>(&*v1::bls12_381::MDS3, "out/v1/bls12_381/mds_t3.bin")?;
    Ok(())
}

fn write_constants_v1_bls12_381_t4() -> Result<()> {
    write_constants_u256::<64, 4>(&*v1::bls12_381::RC4, "out/v1/bls12_381/arc_t4.bin")?;
    write_constants_u256::<4, 4>(&*v1::bls12_381::MDS4, "out/v1/bls12_381/mds_t4.bin")?;
    Ok(())
}

fn write_constants_v1_bluesky_t3() -> Result<()> {
    write_constants_u256::<64, 3>(&*v1::bluesky::RC3, "out/v1/bluesky/arc_t3.bin")?;
    write_constants_u256::<3, 3>(&*v1::bluesky::MDS3, "out/v1/bluesky/mds_t3.bin")?;
    Ok(())
}

fn write_constants_v1_bluesky_t4() -> Result<()> {
    write_constants_u256::<64, 4>(&*v1::bluesky::RC4, "out/v1/bluesky/arc_t4.bin")?;
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

fn write_constants_v1_koalabear_t24() -> Result<()> {
    write_constants_u32::<31, 24>(
        &v1::koalabear::RC24
            .chunks(24)
            .map(|chunk| chunk.iter().copied().collect())
            .collect(),
        "out/v1/koalabear/arc_t24.bin",
    )?;
    write_constants_u32::<24, 24>(&*v1::koalabear::MDS24, "out/v1/koalabear/mds_t24.bin")?;
    Ok(())
}

fn write_constants_v1_koalabear_t32() -> Result<()> {
    write_constants_u32::<39, 32>(
        &v1::koalabear::RC32
            .chunks(32)
            .map(|chunk| chunk.iter().copied().collect())
            .collect(),
        "out/v1/koalabear/arc_t32.bin",
    )?;
    write_constants_u32::<32, 32>(&*v1::koalabear::MDS32, "out/v1/koalabear/mds_t32.bin")?;
    Ok(())
}

fn write_constants_v1_schraderbrau_t3() -> Result<()> {
    write_constants_u256::<91, 3>(&*v1::schraderbrau::RC3, "out/v1/schraderbrau/arc_t3.bin")?;
    write_constants_u256::<3, 3>(&*v1::schraderbrau::MDS3, "out/v1/schraderbrau/mds_t3.bin")?;
    Ok(())
}

fn write_constants_v1_schraderbrau_t4() -> Result<()> {
    write_constants_u256::<92, 4>(&*v1::schraderbrau::RC4, "out/v1/schraderbrau/arc_t4.bin")?;
    write_constants_u256::<4, 4>(&*v1::schraderbrau::MDS4, "out/v1/schraderbrau/mds_t4.bin")?;
    Ok(())
}

fn write_constants_v2_bls12_381_t3() -> Result<()> {
    write_constants_u256::<64, 3>(&*v2::bls12_381::RC3, "out/v2/bls12_381/arc_t3.bin")?;
    write_constants_u256::<3, 3>(&*v2::bls12_381::FL3, "out/v2/bls12_381/fl_t3.bin")?;
    write_constants_u256::<3, 3>(&*v2::bls12_381::PL3, "out/v2/bls12_381/pl_t3.bin")?;
    Ok(())
}

fn write_constants_v2_bls12_381_t4() -> Result<()> {
    write_constants_u256::<64, 4>(&*v2::bls12_381::RC4, "out/v2/bls12_381/arc_t4.bin")?;
    write_constants_u256::<4, 4>(&*v2::bls12_381::FL4, "out/v2/bls12_381/fl_t4.bin")?;
    write_constants_u256::<4, 4>(&*v2::bls12_381::PL4, "out/v2/bls12_381/pl_t4.bin")?;
    Ok(())
}

fn write_constants_v2_bluesky_t3() -> Result<()> {
    write_constants_u256::<64, 3>(&*v2::bluesky::RC3, "out/v2/bluesky/arc_t3.bin")?;
    write_constants_u256::<3, 3>(&*v2::bluesky::FL3, "out/v2/bluesky/fl_t3.bin")?;
    write_constants_u256::<3, 3>(&*v2::bluesky::PL3, "out/v2/bluesky/pl_t3.bin")?;
    Ok(())
}

fn write_constants_v2_bluesky_t4() -> Result<()> {
    write_constants_u256::<64, 4>(&*v2::bluesky::RC4, "out/v2/bluesky/arc_t4.bin")?;
    write_constants_u256::<4, 4>(&*v2::bluesky::FL4, "out/v2/bluesky/fl_t4.bin")?;
    write_constants_u256::<4, 4>(&*v2::bluesky::PL4, "out/v2/bluesky/pl_t4.bin")?;
    Ok(())
}

fn write_constants_v2_goldilocks_t12() -> Result<()> {
    write_constants_u64::<30, 12>(&*v2::goldilocks::RC12, "out/v2/goldilocks/arc_t12.bin")?;
    write_constants_u64::<12, 12>(&*v2::goldilocks::FL12, "out/v2/goldilocks/fl_t12.bin")?;
    write_constants_u64::<12, 12>(&*v2::goldilocks::PL12, "out/v2/goldilocks/pl_t12.bin")?;
    Ok(())
}

fn write_constants_v2_goldilocks_t16() -> Result<()> {
    write_constants_u64::<30, 16>(&*v2::goldilocks::RC16, "out/v2/goldilocks/arc_t16.bin")?;
    write_constants_u64::<16, 16>(&*v2::goldilocks::FL16, "out/v2/goldilocks/fl_t16.bin")?;
    write_constants_u64::<16, 16>(&*v2::goldilocks::PL16, "out/v2/goldilocks/pl_t16.bin")?;
    Ok(())
}

fn write_constants_v2_schraderbrau_t3() -> Result<()> {
    write_constants_u256::<91, 3>(&*v2::schraderbrau::RC3, "out/v2/schraderbrau/arc_t3.bin")?;
    write_constants_u256::<3, 3>(&*v2::schraderbrau::FL3, "out/v2/schraderbrau/fl_t3.bin")?;
    write_constants_u256::<3, 3>(&*v2::schraderbrau::PL3, "out/v2/schraderbrau/pl_t3.bin")?;
    Ok(())
}

fn write_constants_v2_schraderbrau_t4() -> Result<()> {
    write_constants_u256::<92, 4>(&*v2::schraderbrau::RC4, "out/v2/schraderbrau/arc_t4.bin")?;
    write_constants_u256::<4, 4>(&*v2::schraderbrau::FL4, "out/v2/schraderbrau/fl_t4.bin")?;
    write_constants_u256::<4, 4>(&*v2::schraderbrau::PL4, "out/v2/schraderbrau/pl_t4.bin")?;
    Ok(())
}

fn main() -> Result<()> {
    write_constants_v1_bls12_381_t3()?;
    write_constants_v1_bls12_381_t4()?;
    write_constants_v1_bluesky_t3()?;
    write_constants_v1_bluesky_t4()?;
    write_constants_v1_goldilocks_t12()?;
    write_constants_v1_goldilocks_t16()?;
    write_constants_v1_koalabear_t24()?;
    write_constants_v1_koalabear_t32()?;
    write_constants_v1_schraderbrau_t3()?;
    write_constants_v1_schraderbrau_t4()?;
    write_constants_v2_bls12_381_t3()?;
    write_constants_v2_bls12_381_t4()?;
    write_constants_v2_bluesky_t3()?;
    write_constants_v2_bluesky_t4()?;
    write_constants_v2_goldilocks_t12()?;
    write_constants_v2_goldilocks_t16()?;
    write_constants_v2_schraderbrau_t3()?;
    write_constants_v2_schraderbrau_t4()?;
    Ok(())
}
