use std::{env, path::PathBuf};
use prost_wkt_build::*;

fn main() {
    let out = PathBuf::from("./src/pb");
    let descriptor_file = out.join("descriptors.bin");
    let mut prost_build = prost_build::Config::new();
    prost_build
        .type_attribute(
            ".",
            "#[derive(serde::Serialize,serde::Deserialize)]"
        )
        .extern_path(
            ".google.protobuf.Any",
            "::prost_wkt_types::Any"
        )
        .extern_path(
            ".google.protobuf.Timestamp",
            "::prost_wkt_types::Timestamp"
        )
        .extern_path(
            ".google.protobuf.Value",
            "::prost_wkt_types::Value"
        )
        .file_descriptor_set_path(&descriptor_file)
        .compile_protos(
            &[
                "proto/sf/antelope/type/v1/type.proto"
            ],
            &["proto/"],
        )
        .unwrap();

    let descriptor_bytes =
        std::fs::read(descriptor_file)
            .unwrap();

    let descriptor =
        FileDescriptorSet::decode(&descriptor_bytes[..])
            .unwrap();

    prost_wkt_build::add_serde(out, descriptor);
}
