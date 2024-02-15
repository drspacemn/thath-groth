use lambdaworks_groth16::{common::*, QuadraticArithmeticProgram as QAP};

fn vitalik_qap() -> QAP {
    // x^3 + x + 5 = 35
    let num_of_public_inputs = 1;
    let [l, r, o] = [
        [
            ["0", "0", "0", "5"], // 1
            ["1", "0", "1", "0"], // x
            ["0", "0", "0", "0"], // ~out
            ["0", "1", "0", "0"], // sym_1
            ["0", "0", "1", "0"], // y
            ["0", "0", "0", "1"], // sym_2
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

fn main() {
    println!("groth16 stub");

    // Vitalik test
    let qap = vitalik_qap();
}
