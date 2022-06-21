use plonky2::iop::witness::{PartialWitness, Witness};
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::plonk::circuit_data::CircuitConfig;
use plonky2::plonk::config::{GenericConfig, PoseidonGoldilocksConfig};
use sha2::{Sha512, Digest};
use plonky2_sha512::circuit::{array_to_bits, make_circuits};
use anyhow::Result;

pub fn prove_sha512(msg: &[u8]) -> Result<()> {
    let mut hasher = Sha512::new();
    hasher.update(msg);
    let hash = hasher.finalize();
    println!("Hash: {:#04X}", hash);

    let msg_bits = array_to_bits(msg);
    let hash_bits = array_to_bits(&hash.to_vec());

    //println!("{:?}", msg_bits);
    //println!("{:?}", hash_bits);

    let len = msg.len() * 8;
    const D: usize = 2;
    type C = PoseidonGoldilocksConfig;
    type F = <C as GenericConfig<D>>::F;
    let mut builder = CircuitBuilder::<F, D>::new(CircuitConfig::wide_ecc_config());
    let targets = make_circuits(&mut builder, len as u128);
    let mut pw = PartialWitness::new();

    for i in 0..len {
        pw.set_bool_target(targets.message[i], msg_bits[i]);
    }

    for i in 0..512 {
        if hash_bits[i] {
            builder.assert_one(targets.digest[i].target);
        } else {
            builder.assert_zero(targets.digest[i].target);
        }
    }

    let data = builder.build::<C>();
    let proof = data.prove(pw).unwrap();

    data.verify(proof)
}

fn main() -> Result<()> {
    let mut msg = vec![0; 128 as usize];
    for i in 0..127 {
        msg[i] = i as u8;
    }
    prove_sha512(&msg)
}
