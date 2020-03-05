use nalgebra::dimension::*;
use toy_quant::{
    gates::{binary::gates::cnot, unitary::gates::h},
    qubit::Qubit,
    registers::quantum::QuantumRegister,
};

fn entangle_qubits(
    ket_a: Qubit,
    ket_b: Qubit,
) -> QuantumRegister<U4> {
    let ket_a = h().run(ket_a);
    let merged = QuantumRegister::from_2_qubits(ket_a, ket_b);
    cnot().apply(merged)
}

fn main() {
    let bell = entangle_qubits(Qubit::one(), Qubit::one());
    let mut states = [0, 0, 0, 0];
    for _ in 0..100 {
        states[bell.collapse().bits as usize] += 1;
    }
    println!("{:?}", states);
}
