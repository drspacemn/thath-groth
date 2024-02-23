use itertools::Itertools;
use lambdaworks_groth16::{common::*, QuadraticArithmeticProgram as QAP};

pub mod print_utils;

//TODO: How is num_of_public_inputs used?
//TODO: step thru with vitalik example paper & doc & better outputs
//TODO: Use a library to generate the QAP from program?
//TODO: generate a fake proof and verify it using toxic waste

fn vitalik_qap() -> QAP {
    // https://vitalik.ca/general/2016/12/10/qap.html
    // x^3 + x + 5
    let num_of_public_inputs = 1;
    let [l, r, o] = [
        [
            ["0", "0", "0", "5"],
            ["1", "0", "1", "0"],
            ["0", "0", "0", "0"],
            ["0", "1", "0", "0"],
            ["0", "0", "1", "0"],
            ["0", "0", "0", "1"],
        ],
        [
            ["0", "0", "1", "1"],
            ["1", "1", "0", "0"],
            ["0", "0", "0", "0"],
            ["0", "0", "0", "0"],
            ["0", "0", "0", "0"],
            ["0", "0", "0", "0"],
        ],
        [
            ["0", "0", "0", "0"],
            ["0", "0", "0", "0"],
            ["0", "0", "0", "1"],
            ["1", "0", "0", "0"],
            ["0", "1", "0", "0"],
            ["0", "0", "1", "0"],
        ],
    ]
    .map(|matrix| matrix.map(|row| row.map(FrElement::from_hex_unchecked).to_vec()));
    QAP::from_variable_matrices(num_of_public_inputs, &l, &r, &o)
}

fn another_qap() -> QAP {
    // x^2 + 2*y + 3
    //
    // Flatten :
    // sym_1 = x*x
    // sym_2 = 2*y
    // sym_3 = sym_1 + sym_2
    // ~out = sym_3 + 3
    //
    // Variable mapping:       ~one,   x, ~out, sym_1,   y, sym_2, sym_3
    // Example solution vec : [   1,   2,   13,    4,   3,     6,    10]    
    //
    // Gates :
    // 1. sym_1 = x*x
    // [ 0, 1, 0, 0, 0, 0, 0]
    // [ 0, 1, 0, 0, 0, 0, 0]
    // [ 0, 0, 0, 1, 0, 0, 0]
    //
    // 2. sym_2 = 2*y
    // [ 2, 0, 0, 0, 0, 0, 0]
    // [ 0, 0, 0, 0, 1, 0, 0]
    // [ 0, 0, 0, 0, 0, 1, 0]
    //
    // 3. sym_3 = sym_1 + sym_2
    // [ 0, 0, 0, 1, 0, 1, 0]
    // [ 1, 0, 0, 0, 0, 0, 0]
    // [ 0, 0, 0, 0, 0, 0, 1]
    //
    // 4. ~out = sym_3 + 3
    // [ 3, 0, 0, 0, 0, 0, 1]
    // [ 1, 0, 0, 0, 0, 0, 0]
    // [ 0, 0, 1, 0, 0, 0, 0]
    //
    // Gives R1CS :
    // A:
    // [ 0, 1, 0, 0, 0, 0, 0]
    // [ 2, 0, 0, 0, 0, 0, 0]
    // [ 0, 0, 0, 1, 0, 1, 0]
    // [ 3, 0, 0, 0, 0, 0, 1]
    //
    // B:
    // [ 0, 1, 0, 0, 0, 0, 0]
    // [ 0, 0, 0, 0, 1, 0, 0]
    // [ 1, 0, 0, 0, 0, 0, 0]
    // [ 1, 0, 0, 0, 0, 0, 0]
    //
    // C:
    // [ 0, 0, 0, 1, 0, 0, 0]
    // [ 0, 0, 0, 0, 0, 1, 0]
    // [ 0, 0, 0, 0, 0, 0, 1]
    // [ 0, 0, 1, 0, 0, 0, 0]

    let num_of_public_inputs = 2;
    let [l, r, o] = [
        [
            ["0", "2", "0", "3"],
            ["1", "0", "0", "0"],
            ["0", "0", "0", "0"],
            ["0", "0", "1", "0"],
            ["0", "0", "0", "0"],
            ["0", "0", "1", "0"],
            ["0", "0", "0", "1"],
        ],
        [
            ["0", "0", "1", "1"],
            ["1", "0", "0", "0"],
            ["0", "0", "0", "0"],
            ["0", "0", "0", "0"],
            ["0", "1", "0", "0"],
            ["0", "0", "0", "0"],
            ["0", "0", "0", "0"],
        ],
        [
            ["0", "0", "0", "0"],
            ["0", "0", "0", "0"],
            ["0", "0", "0", "1"],
            ["1", "0", "0", "0"],
            ["0", "0", "0", "0"],
            ["0", "1", "0", "0"],
            ["0", "0", "1", "0"],

        ],
    ]
    .map(|matrix| matrix.map(|row| row.map(FrElement::from_hex_unchecked).to_vec()));
    QAP::from_variable_matrices(num_of_public_inputs, &l, &r, &o)
}

fn main() {
    println!("groth16 test(s) running...\n");

    // Toxic waste
    println!("Toxic waste values");
    println!("-----------------");
    let toxic_waste_val = sample_fr_elem();
    println!("Toxic waste value: {}\n", toxic_waste_val);

    // Vitalik test
    println!("Vitalik test");
    println!("------------");
    let qap = vitalik_qap();
    let (prover_key, verifying_key) = lambdaworks_groth16::setup(&qap);
    println!("Generated keys for Vitalik QAP :");
    //TODO: pass as a reference?
    println!("{}", print_utils::prover_key_to_string(&prover_key, 2));
    println!("{}", print_utils::verifying_key_to_string(&verifying_key, 2));

    for witness in [
        //  1      x     out    x*x     x**3   x**3+x
        ["0x1", "0x3", "0x23", "0x9", "0x1b", "0x1e"], // x**3 + x + 5 = 35 ( x = 3 )
        ["0x1", "0x2", "0xf", "0x4", "0x8", "0xa"], // x**3 + x + 5 = 15 ( x = 2 )
        ["0x1", "0x1", "0x7", "0x1", "0x1", "0x2"], // x**3 + x + 5 = 7 ( x = 1 )

        ["0x1", "0x3", "0x24", "0x9", "0x1b", "0x1e"], // x**3 + x + 5 = 36 ( x = 3 ) - wrong
    ] {
        println!("Proof for witness = {:?}", witness);
        let witness = witness
            .map(FrElement::from_hex_unchecked)
            .to_vec();

        let serialized_proof =
            lambdaworks_groth16::Prover::prove(&witness, &qap, &prover_key).serialize();
        let deserialized_proof =
            lambdaworks_groth16::Proof::deserialize(&serialized_proof).unwrap();
        println!("Serialized proof: 0x{:02X}", serialized_proof.iter().format(""));

        let accept = lambdaworks_groth16::verify(
            &verifying_key,
            &deserialized_proof,
            &witness[..qap.num_of_public_inputs],
        );
        println!("Verified as : {}\n", accept);
    }


    // Another test
    println!("Another test");
    println!("------------");
    let qap = another_qap();
    let (prover_key, verifying_key) = lambdaworks_groth16::setup(&qap);
    println!("Generated keys for Another QAP :");
    println!("{}", print_utils::prover_key_to_string(&prover_key, 2));
    println!("{}", print_utils::verifying_key_to_string(&verifying_key, 2));

    for witness in [
        // Variable mapping:       ~one,   x, ~out, sym_1,   y, sym_2, sym_3
        //                            1    x   out    x*x    y   2*y   x*x+2*y
        ["0x1", "0x2", "0xd", "0x4", "0x3", "0x6", "0xa"], // x^2 + 2*y + 3 = 13 ( x = 2, y = 3 )
        ["0x1", "0x3", "0x16", "0x9", "0x5", "0xa", "0x13"], // x^2 + 2*y + 3 = 22 ( x = 3, y = 5 )
        ["0x1", "0x1", "0x6", "0x1", "0x1", "0x2", "0x3"], // x^2 + 2*y + 3 = 6 ( x = 1, y = 1 )

        ["0x1", "0x2", "0xe", "0x4", "0x3", "0x6", "0xa"], // x^2 + 2*y + 3 = 14 ( x = 2, y = 3 ) - wrong
    ] {
        println!("Proof for witness = {:?}", witness);
        let witness = witness
            .map(FrElement::from_hex_unchecked)
            .to_vec();

        let serialized_proof =
            lambdaworks_groth16::Prover::prove(&witness, &qap, &prover_key).serialize();
        let deserialized_proof =
            lambdaworks_groth16::Proof::deserialize(&serialized_proof).unwrap();
        println!("Serialized proof: 0x{:02X}", serialized_proof.iter().format(""));

        let accept = lambdaworks_groth16::verify(
            &verifying_key,
            &deserialized_proof,
            &witness[..qap.num_of_public_inputs],
        );
        println!("Verified as : {}\n", accept);
    }
}
