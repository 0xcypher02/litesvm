use {litesvm::LiteSVM, solana_pubkey::pubkey, solana_sdk_ids::bpf_loader};

// https://github.com/LiteSVM/litesvm/issues/140
#[test]
fn test_dflow_load() {
    let mut svm = LiteSVM::new();
    let program_bytes =
        include_bytes!("../test_programs/DF1ow3DqMj3HvTj8i8J9yM2hE9hCrLLXpdbaKZu4ZPnz.so");
    svm.add_program(
        &bpf_loader::id(),
        pubkey!("DF1ow3DqMj3HvTj8i8J9yM2hE9hCrLLXpdbaKZu4ZPnz"),
        program_bytes,
    );
}
