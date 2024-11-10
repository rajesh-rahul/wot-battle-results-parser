alias gm := gen_method_mappings

rp:
    cargo run --bin replay_parser_example

dev-simple:
    ../rustc_codegen_cranelift/dist/cargo-clif run --bin replay_parser_simple

dev-rp:
    ../rustc_codegen_cranelift/dist/cargo-clif run --bin replay_parser_example

rpr:
    cargo run --release --bin replay_parser_example

dp:
    cargo run --bin wot_datfile_parser_dev
fmt:
    cargo +nightly fmt

udeps:
    cargo +nightly udeps

data_extract:
    python scripts/wot_data_extract.py

copy_wot_src:
    python scripts/copy_wot_src_cleaned.py

create_test_outputs:
    cargo run --release --bin test_output_writer

int_tests:
    cargo test --test '*'

gen_method_mappings:
    python scripts/gen_method_mappings.py
