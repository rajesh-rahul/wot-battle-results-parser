from pathlib import Path
import json
from versions import VERSIONS


from replay_unpack.core.entity_def.definitions import Definitions
from replay_unpack.core.entity_def import EntityFlags, data_types, EntityDef
from replay_unpack.core.entity_def.base_definition import Property
from replay_unpack.core.entity_def.entity_description import EntityMethod

from utils import write_struct, Iden, Enum

entity_name_enum_builder = set()

def main():
    for version in VERSIONS:
        final_json = build_json(version)
        with open(f"./replay_parser/src/wot_data/v{version}.rs", "w") as f:
            final_json_to_rust_code(version, final_json, f)

    with open(f"./replay_parser/src/wot_data/mod.rs", "w") as f:
        f.write("mod structures;\nmod methods;\npub(crate) use structures::*;\npub(crate) use methods::*;\n")
        for version in VERSIONS:
            f.write(f"mod v{version};\n")
        f.write("\n\n")
        f.write("pub const WOT_DATA_ALL_VERSIONS: phf::Map<&'static str, WotDataForVersion> = phf::phf_map! {\n")
        for version in VERSIONS:
            f.write(f"\t\"{version}\" => v{version}::DATA_{version},\n")
        f.write("};")
        f.write("\n\n")

        f.write("pub const VERSIONS: &'static [[u16; 4]] = &[")
        for version in VERSIONS:
            version_arr = version.split("_")
            f.write(f"[{version_arr[0]}, {version_arr[1]}, {version_arr[2]}, {version_arr[3]}],")
        f.write("];")

        f.write("\n\n#[derive(Debug, Copy, Clone, strum::Display, PartialEq, Eq, serde::Serialize)]\npub enum EntityType {")
        for entity_name in entity_name_enum_builder:
            f.write(entity_name + ",")
        f.write("}")

def build_json(version):
    defs = Definitions(Path("./wot_xml") / version / "sources" / "res")

    final_json = {}

    final_json["special_formats"] = {}
    final_json["special_formats"]["struct"] = "SpecialFormat"


    entities = [None] * len(defs._entity_defs_by_index)
    for idx, entity in defs._entity_defs_by_index.items():
        entity_json = {}
        
        entity_json["ty"] = Iden(entity.get_name())

        if entity.get_name() == "Avatar":
            final_json["special_formats"]["create_avatar"] = create_avatar_event_data(entity)
        entity_name_enum_builder.add(entity_json["ty"])
        entity_json["struct"] = "EntityData"

        props = entity.properties().get_properties_by_flags(
            EntityFlags.ALL_CLIENTS |
            EntityFlags.BASE_AND_CLIENT |
            EntityFlags.OTHER_CLIENTS |
            EntityFlags.OWN_CLIENT |
            EntityFlags.CELL_PUBLIC_AND_OWN |
            EntityFlags.ALL_CLIENTS,
            exposed_index=True
        )
        entity_json["exposed_props"] = [prop_to_dict(prop) for prop in props]

        methods = entity._client_methods.get_exposed_index_map()
        for i, m in enumerate(methods):
            if m._name == "updateArena":
                print(version, m, i)
        entity_json["exposed_methods"] = [method_to_dict(method) for method in methods]


        entities[idx] = entity_json
    
    final_json["packet_map"] = get_packet_mappings(version)
    final_json["entities"] = entities
    final_json["struct"] = "WotDataForVersion"

    return final_json



def prop_to_dict(prop: Property):
    prop_json = {}

    prop_json["name"] = prop.get_name()
    prop_json["struct"] = "PropData"
    prop_json["default_value"] = prop.get_default_value()
    prop_json["datatype"] = prop._type
    prop_json["size"] = prop.get_size_in_bytes()

    return prop_json

def method_to_dict(method: EntityMethod):
    method_json = {}

    method_json["name"] = method.get_name()
    method_json["struct"] = "MethodData"
    method_json["args"] = []
    method_json["size"] = method.get_size_in_bytes()
    for arg in method._arguments:
        # arg_json = {}
        # arg_json["name"] = arg.name or "Unknown"
        # arg_json["struct"] = "MethodArgData"
        # arg_json["datatype"]= arg.type

        method_json["args"].append(arg.type)

    return method_json


def get_packet_mappings(version):
    mappings = {}
    version_arr = list(map(lambda x: int(x), version.split("_")))

    mappings[Iden("0x00u32")] = Iden("PacketName::CreateAvatar")
    mappings[Iden("0x05u32")] = Iden("PacketName::CreateEntity")
    # mappings[0x07] = "EntityPropertyUpdate"
    mappings[Iden("0x08u32")] = Iden("PacketName::EntityMethod")
    mappings[Iden("0x0Au32")] = Iden("PacketName::Position")

    if version_arr <= [0, 9, 13, 0]:
        mappings[Iden("0x14u32")] = Iden("PacketName::GameVersion")
    else:
        mappings[Iden("0x18u32")] = Iden("PacketName::GameVersion")

    mappings[Iden("0x23u32")] = Iden("PacketName::Chat")
    mappings[Iden("0x3Du32")] = Iden("PacketName::CryptoKey")

    return mappings



def final_json_to_rust_code(version, final_json, f):
    f.write("use super::*;\n\nuse EntityType::*;\nuse crate::PacketName;")
    f.write(f"pub(crate) const DATA_{version}: WotDataForVersion = ")
    write_struct(final_json, f)
    f.write(";")


def create_avatar_event_data(avatar_entity: EntityDef):
    props = avatar_entity.properties().get_properties_by_flags(
            EntityFlags.BASE_AND_CLIENT
    )

    return [prop_to_dict(prop) for prop in props]
        


if __name__ == "__main__":
    main()
