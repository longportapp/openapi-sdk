use prost_build::Config;

fn main() {
    Config::new()
        .out_dir("./src")
        .compile_protos(
            &[
                "./openapi-protobufs/control/control.proto",
                "./openapi-protobufs/quote/api.proto",
                "./openapi-protobufs/trade/subscribe.proto",
                "./error/error.proto",
            ],
            &["./openapi-protobufs", "./error"],
        )
        .unwrap();
    println!("cargo:rerun-if-changed=openapi-protobufs");
}
