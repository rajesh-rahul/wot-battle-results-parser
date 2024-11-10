from pathlib import Path
from pathlib import Path
import json
from versions import VERSIONS
import utils
import inflection

from replay_unpack.core.entity_def.definitions import Definitions
from replay_unpack.core.entity_def import EntityFlags, data_types, EntityDef
from replay_unpack.core.entity_def.base_definition import Property
from replay_unpack.core.entity_def.entity_description import EntityMethod

TEST_REPLAYS_FOLDER = Path("./replay_parser/tests/parser_tests/test_input")
METHOD_MAPS_OUTPUT_PATH = "./replay_parser/src/wot_data/methods.rs"


def extract_version(path):
    return str(path).split("-")[1]

VERSIONS = list(set(map(extract_version, TEST_REPLAYS_FOLDER.glob("*.wotreplay"))))
VERSIONS.sort(key=lambda v: list(map(int, v.split("_"))))


ENTITIES_IN_REPLAY = ["Avatar", "Vehicle"] # TODO: There are more..

if __name__ == "__main__":
    json_output = {}
    method_enum_json = {}

    for version in VERSIONS:
        try:
            defs = Definitions(Path("./wot_xml") / version / "sources" / "res")
        except Exception:
            version_major_patch_only = version[:-1] + "0" # zero out the micro patch identifier
            defs = Definitions(Path("./wot_xml") / version_major_patch_only / "sources" / "res")



        for idx, entity in defs._entity_defs_by_index.items():
            entity_name = entity.get_name()
            if entity_name not in ENTITIES_IN_REPLAY:
                continue
            
            json_output.setdefault(entity_name, {})
            method_enum_json.setdefault(entity_name, set())
            json_output[entity_name][version] = []

            for m in entity._client_methods.get_exposed_index_map():
                method = utils.Iden(inflection.camelize(m.get_name()))
                method_enum_json[entity_name].add(method)
                json_output[entity_name][version].append(method)

    with open(METHOD_MAPS_OUTPUT_PATH, "w") as f:
        f.write("use super::*;\n\n")
        
        for entity_name, methods in method_enum_json.items():
            f.write(f"#[derive(strum::Display, Clone, Copy)]\npub enum {entity_name}Method {{")
            for m in methods:
                f.write(f"\t{m},")
            f.write("}\n\n")

        for entity_name, version_data in json_output.items():
            f.write(f"pub(crate) const {entity_name.upper()}_METHODS: phf::Map<&'static str, &'static[{entity_name}Method]> = {{\nuse {entity_name}Method::*;\n")
            utils.write_item(version_data, f, False)
            f.write("}")
            f.write(";")
