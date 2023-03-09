use protobuf_codegen_pure::Customize;

fn main() {
    let customizer = Customize {
        gen_mod_rs: Some(true),
        lite_runtime: Some(true),
        ..Default::default()
    };

    protobuf_codegen_pure::Codegen::new()
        .out_dir("src/proto")
        .inputs([
            "third_party/proto/cosmos/base/v1beta1/coin.proto",
            "proto/injective/exchange/v1beta1/exchange.proto",
            "proto/injective/exchange/v1beta1/tx.proto",
            "proto/injective/oracle/v1beta1/oracle.proto",
            "third_party/proto/cosmos/distribution/v1beta1/distribution.proto",
        ])
        .includes(["proto", "third_party/proto"])
        .customize(customizer)
        .run()
        .expect("Protobuf codegen failed.");
}
