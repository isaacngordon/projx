
fn main() {
    cynic_codegen::register_schema("waveapp")
        .from_sdl_file("./schemas/wave_api_schema_20240728.graphql")
        .unwrap()
        .as_default()
        .unwrap();
}
