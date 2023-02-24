fn main() {
    protobuf_codegen_pure::Codegen::new()
        .out_dir("src/proto")
        .inputs(["proto/order_reply.proto", "proto/trade_data.proto"])
        .include("proto")
        .run()
        .expect("Protobuf codegen failed.");
}
