use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

use wot_replay_parser::{PacketName, ReplayError, ReplayParser};

const TEST_INPUT_DIR: &'static str = "./replay_parser/tests/parser_tests/test_input";
const TEST_OUTPUT_DIR: &'static str = "./replay_parser/tests/parser_tests/test_output";

pub fn main() {
    let replays: Vec<_> = std::fs::read_dir(TEST_INPUT_DIR)
        .unwrap()
        .flatten()
        .flat_map(|it| {
            if it.path().extension().and_then(OsStr::to_str) == Some("wotreplay") {
                Some(it.path())
            } else {
                None
            }
        })
        .collect();

    let mut file_data = HashMap::new();
    for (idx, replay) in replays.iter().enumerate() {
        println!("Parsing ({}/{}) - {:?}", idx + 1, replays.len(), replay);

        let parser = ReplayParser::parse_file(&replay).unwrap();

        let max_position_packets = 10;
        let position_packet_count = 0;
        let mut counter = 0;

        let output_path = |pkt_name| {
            Path::new(TEST_OUTPUT_DIR)
                .join(pkt_name)
                .join(format!("{}.jsonl", replay.file_stem().unwrap().to_string_lossy()))
        };

        for event in parser.event_stream().unwrap() {
            match event {
                Err(
                    err @ ReplayError::PacketParseError {
                        packet_name: packet_name @ PacketName::EntityMethod,
                        ..
                    },
                ) => {
                    let output_file_path = output_path(packet_name.to_string());

                    let data = file_data.entry(output_file_path).or_insert(String::new());

                    data.push_str(&err.to_string());
                    data.push_str("\n");
                }
                Ok(event) => {
                    let packet_name = event.corresponding_pkt_name();

                    match packet_name {
                        PacketName::EntityMethod => {
                            let output_file_path = output_path(packet_name.to_string());

                            let data = file_data.entry(output_file_path).or_insert(String::new());

                            data.push_str(&serde_json::to_string(&event).unwrap());
                            data.push_str("\n");
                        }
                        // PacketName::Position if position_packet_count < 10 && counter % 5 == 0 => {
                        //     let output_file_path = Path::new(TEST_OUTPUT_DIR)
                        //     .join(pkt_name.to_string())
                        //     .join(format!("{}.jsonl", replay.file_stem().unwrap().to_string_lossy()));

                        //     let data = file_data.entry(output_file_path).or_insert(String::new());

                        //     data.push_str(&serde_json::to_string(&event).unwrap());
                        //     data.push_str("\n");
                        //     position_packet_count += 1;
                        // }
                        _ => {}
                    }
                }
                _ => {}
            }
            counter += 1;
        }
    }

    for (idx, (k, v)) in file_data.iter().enumerate() {
        println!("Writing ({}/{}) - {:?}", idx + 1, file_data.len(), k);

        std::fs::create_dir_all(k.parent().unwrap()).unwrap();
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(k)
            .unwrap();

        write!(file, "{}", v).unwrap();
    }
}
