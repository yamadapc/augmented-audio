use lib_flutter_rust_bridge_codegen::{
    config_parse, frb_codegen, get_symbols_if_no_duplicates, RawOpts,
};

const RUST_INPUT: &str = "src/api.rs";

fn main() {
    println!("cargo:rerun-if-changed={}", RUST_INPUT);
    let raw_opts = RawOpts {
        rust_input: vec![RUST_INPUT.to_string()],
        dart_output: vec!["./lib/bridge_generated.dart".to_string()],
        ..RawOpts::default()
    };
    let configs = config_parse(raw_opts);
    let all_symbols = get_symbols_if_no_duplicates(&configs).unwrap();
    for config in configs.iter() {
        frb_codegen(config, &all_symbols).expect("FRB codegen failed");
    }
}
