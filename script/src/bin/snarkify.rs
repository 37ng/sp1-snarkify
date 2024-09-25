use std::io;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use snarkify_sdk::prover::ProofHandler;
use sp1_sdk::{ProverClient, SP1ProofWithPublicValues, SP1Stdin};

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const FIBONACCI_ELF: &[u8] = include_bytes!("../../../elf/riscv32im-succinct-zkvm-elf");

struct MyProofHandler;

#[derive(Deserialize)]
struct MyInput {
    /// Define your own input fields here.
    public_input: u32,
}

#[derive(Serialize)]
struct MyOutput {
    /// Define your own output fields here.
    proof: SP1ProofWithPublicValues,
}

#[async_trait]
impl ProofHandler for MyProofHandler {
    type Input = MyInput;
    type Output = MyOutput;
    type Error = ();

    async fn prove(data: Self::Input) -> Result<Self::Output, Self::Error> {
        // Setup the prover client.
        let client = ProverClient::new();

        // Setup the program for proving.
        let (pk, _vk) = client.setup(FIBONACCI_ELF);

        // Setup the inputs.
        let mut stdin = SP1Stdin::new();
        stdin.write(&data.public_input); // hardcoded input

        // Generate the proof
        let proof = client
            .prove(&pk, stdin)
            .run()
            .expect("failed to generate proof");

        return Ok(MyOutput { proof });
    }
}

fn main() -> Result<(), io::Error> {
    snarkify_sdk::run::<MyProofHandler>()
}
