use prost_build::compile_protos;

fn main() {
    compile_protos(&[
        "protos/common.proto",
        "protos/data.proto",
        "protos/debug.proto",
        "protos/error.proto",
        "protos/query.proto",
        "protos/raw.proto",
        "protos/sc2api.proto",
        "protos/score.proto",
        "protos/spatial.proto",
        "protos/ui.proto",
    ],
    &["protos/"]).unwrap();
}