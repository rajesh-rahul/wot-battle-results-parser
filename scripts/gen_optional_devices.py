from pathlib import Path
from pathlib import Path
import inflection

from replay_unpack.core.entity_def.definitions import Definitions
from replay_unpack.core.entity_def import EntityFlags, data_types, EntityDef
from replay_unpack.core.entity_def.base_definition import Property
from replay_unpack.core.entity_def.entity_description import EntityMethod

from lxml import etree
WOT_SRC = "../wot-src/"
OPTIONAL_DEVICES_XML = WOT_SRC + "sources/res/scripts/item_defs/vehicles/common/optional_devices.xml"

WRITE_PATH = "replay_parser/src/wot_data/optional_device.rs"

if __name__ == "__main__":
    xml = etree.parse(OPTIONAL_DEVICES_XML, parser=etree.XMLParser(remove_comments=True, recover=True))
    root = xml.getroot()

    optional_devices = {}

    for child in root:
        if child.tag.startswith("xmlns"):
            continue

        id_ = child.find("id").text.strip()
        optional_devices[id_] = child.tag
    

    with open(WRITE_PATH, "w") as f:
        f.write("#[repr(i32)]\n#[derive(PartialEq, Hash, Eq, Copy, Clone, Debug, strum::Display, serde::Serialize)]\n")
        f.write("pub enum OptionalDevice {")
        for id_, name in optional_devices.items():
            f.write(f"{inflection.camelize(name)} = {id_},")
        
        f.write("}\n\n\n")
        f.write("""
                impl  OptionalDevice {

                    pub fn from_number(num: impl Into<i32>) -> Option<OptionalDevice> {
                        match num.into() {""")
        for id_, name in optional_devices.items():
            f.write(f"{id_} => Some(OptionalDevice::{inflection.camelize(name)}),\n")
        f.write("""_ => None\n}}}""")