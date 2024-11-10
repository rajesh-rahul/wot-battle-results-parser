use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use pretty_assertions::assert_eq;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use wot_replay_parser::{PacketName, ReplayParser};

const TEST_INPUT_DIR: &'static str = "./tests/parser_tests/test_input";
const TEST_OUTPUT_DIR: &'static str = "./tests/parser_tests/test_output";


pub fn get_file_data_for_packet_name(
    expected_pkt_name: PacketName, count: Option<usize>,
) -> HashMap<PathBuf, String> {
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

    let maps: Vec<_> = replays
        .par_iter()
        .map(|replay| {
            let mut file_data = HashMap::new();
            let parser = ReplayParser::parse_file(&replay).unwrap();

            let mut overall_counter = 0;
            let mut accepted_packet_counter = 0;
            for event in parser.event_stream().unwrap() {
                let event = event.unwrap();
                let pkt_name = event.corresponding_pkt_name();

                if count.is_some_and(|c| accepted_packet_counter >= c) {
                    break;
                }

                if pkt_name == expected_pkt_name && (overall_counter % 5 == 0 || count.is_none()) {
                    let output_file_path = Path::new(TEST_OUTPUT_DIR)
                        .join(pkt_name.to_string())
                        .join(format!("{}.jsonl", replay.file_stem().unwrap().to_string_lossy()));

                    let data = file_data.entry(output_file_path).or_insert(String::new());

                    data.push_str(&serde_json::to_string(&event).unwrap());
                    data.push_str("\n");
                    accepted_packet_counter += 1;
                }

                overall_counter += 1;
            }

            file_data
        })
        .collect();

    let file_data = maps.into_iter().flatten().collect();


    file_data
}

pub fn check_output(file_data: HashMap<PathBuf, String>) {
    let parent_dir = file_data.iter().next().unwrap().0.parent().unwrap();

    assert_eq!(file_data.len(), std::fs::read_dir(parent_dir).unwrap().count());

    for (file_path, actual_output) in file_data {
        let expected_output = std::fs::read_to_string(file_path).unwrap();

        assert_eq!(actual_output, expected_output);
    }
}

#[test]
fn test_replay_version_event() {
    let data = get_file_data_for_packet_name(PacketName::GameVersion, None);
    check_output(data);
}

#[test]
fn test_create_avatar_event() {
    let data = get_file_data_for_packet_name(PacketName::CreateAvatar, None);
    check_output(data);
}

#[test]
// NOTE: depends on test_output_writer.rs
fn test_position_event() {
    let data = get_file_data_for_packet_name(PacketName::Position, Some(10));
    check_output(data);
}
