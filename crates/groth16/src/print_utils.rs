use itertools::Itertools;
use lambdaworks_groth16::common::*;

pub fn hash_point_g1(p: &G1Point) -> String {
    let str_rep = format!("({:?})", p);
    let mut hasher = blake3::Hasher::new();
    hasher.update(str_rep.as_bytes());
    format!("G1 {:?}", hasher.finalize())
}

pub fn hash_point_g2(p: &G2Point) -> String {
    let str_rep = format!("({:?})", p);
    let mut hasher = blake3::Hasher::new();
    hasher.update(str_rep.as_bytes());
    format!("G2 {:?}", hasher.finalize())
}

pub fn prover_key_to_string(prover_key: &lambdaworks_groth16::ProvingKey, indent: usize) -> String {
    let mut s = String::new();
    let indent_str = " ".repeat(indent);
    s.push_str(&format!("{}ProvingKey {{\n", indent_str));
    s.push_str(&format!(
        "{}  a_1   : {}\n",
        indent_str,
        hash_point_g1(&prover_key.alpha_g1)
    ));
    s.push_str(&format!(
        "{}  b_1   : {}\n",
        indent_str,
        hash_point_g1(&prover_key.beta_g1)
    ));
    s.push_str(&format!(
        "{}  b_2   : {}\n",
        indent_str,
        hash_point_g2(&prover_key.beta_g2)
    ));
    s.push_str(&format!(
        "{}  d_1   : {}\n",
        indent_str,
        hash_point_g1(&prover_key.delta_g1)
    ));
    s.push_str(&format!(
        "{}  d_2   : {}\n",
        indent_str,
        hash_point_g2(&prover_key.delta_g2)
    ));
    let vec_indent_str = format!("\n{}          ", indent_str);
    s.push_str(&format!(
        "{}  l_t_1 : [{}{}{}]\n",
        indent_str,
        vec_indent_str,
        prover_key
            .l_tau_g1
            .iter()
            .map(hash_point_g1)
            .collect::<Vec<String>>()
            .iter()
            .format(&vec_indent_str),
        vec_indent_str
    ));
    s.push_str(&format!(
        "{}  r_t_1 : [{}{}{}]\n",
        indent_str,
        vec_indent_str,
        prover_key
            .r_tau_g1
            .iter()
            .map(hash_point_g1)
            .collect::<Vec<String>>()
            .iter()
            .format(&vec_indent_str),
        vec_indent_str
    ));
    s.push_str(&format!(
        "{}  r_t_2 : [{}{}{}]\n",
        indent_str,
        vec_indent_str,
        prover_key
            .r_tau_g2
            .iter()
            .map(hash_point_g2)
            .collect::<Vec<String>>()
            .iter()
            .format(&vec_indent_str),
        vec_indent_str
    ));
    s.push_str(&format!(
        "{}  k_t_1 : [{}{}{}]\n",
        indent_str,
        vec_indent_str,
        prover_key
            .prover_k_tau_g1
            .iter()
            .map(hash_point_g1)
            .collect::<Vec<String>>()
            .iter()
            .format(&vec_indent_str),
        vec_indent_str
    ));
    s.push_str(&format!(
        "{}  z_t_1 : [{}{}{}]\n",
        indent_str,
        vec_indent_str,
        prover_key
            .z_powers_of_tau_g1
            .iter()
            .map(hash_point_g1)
            .collect::<Vec<String>>()
            .iter()
            .format(&vec_indent_str),
        vec_indent_str
    ));
    s.push_str(&format!("{}}}\n", indent_str));
    s
}

pub fn verifying_key_to_string(
    verifier_key: &lambdaworks_groth16::VerifyingKey,
    indent: usize,
) -> String {
    let mut s = String::new();
    let indent_str = " ".repeat(indent);
    s.push_str(&format!("{}VerifyingKey {{\n", indent_str));
    let mut a_1_x_b_2_str = format!("{:?}", verifier_key.alpha_g1_times_beta_g2);
    let mut hasher = blake3::Hasher::new();
    hasher.update(a_1_x_b_2_str.as_bytes());
    a_1_x_b_2_str = format!("FE {:?}", hasher.finalize());
    s.push_str(&format!("{}  a_1-b_2 : {}\n", indent_str, a_1_x_b_2_str));
    s.push_str(&format!(
        "{}  d_2     : {}\n",
        indent_str,
        hash_point_g2(&verifier_key.delta_g2)
    ));
    s.push_str(&format!(
        "{}  g_2     : {}\n",
        indent_str,
        hash_point_g2(&verifier_key.gamma_g2)
    ));
    let vec_indent_str = format!("\n{}          ", indent_str);
    s.push_str(&format!(
        "{}  k_t_1 : [{}{}{}]\n",
        indent_str,
        vec_indent_str,
        verifier_key
            .verifier_k_tau_g1
            .iter()
            .map(hash_point_g1)
            .collect::<Vec<String>>()
            .iter()
            .format(&vec_indent_str),
        vec_indent_str
    ));
    s.push_str(&format!("{}}}\n", indent_str));
    s
}
