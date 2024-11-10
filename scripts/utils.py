
from replay_unpack.core.entity_def.definitions import Definitions
from replay_unpack.core.entity_def import EntityFlags, data_types, EntityDef
from replay_unpack.core.entity_def.base_definition import Property
from replay_unpack.core.entity_def.entity_description import EntityMethod
import json

class Iden(str):
    pass

class Enum(dict):
    pass


def write_item(item, f, use_wrapper):
    if isinstance(item, data_types.DataType):
        write_datatype(item, f)
    elif isinstance(item, Iden):
        if use_wrapper:
            raise Exception("Unexpected!")
        f.write(f"{item}")
    # Iden must come before str
    elif isinstance(item, str):
        if use_wrapper:
            f.write("WotXmlDefaultValue::Str(")
        f.write(f"\"{item}\"")
        if use_wrapper:
            f.write(")")
    elif isinstance(item, float):
        if use_wrapper:
            f.write("WotXmlDefaultValue::Float(")
        f.write(f"{item}")
        if use_wrapper:
            f.write(")")
    # NOTE: bool must come before int
    elif isinstance(item, bool):
        if use_wrapper:
            f.write("WotXmlDefaultValue::Bool(")
        if item:
            f.write("true")
        else:
            f.write("false")
        if use_wrapper:
            f.write(")") 
    elif isinstance(item, int):
        if use_wrapper:
            f.write("WotXmlDefaultValue::Int(")
        f.write(f"{item}")
        if use_wrapper:
            f.write(")")

    elif isinstance(item, list):
        if use_wrapper:
            f.write("WotXmlDefaultValue::List(")
        f.write("&[")
        for i in item:
            write_item(i, f, use_wrapper)
            f.write(",")
        f.write("]")
        if use_wrapper:
            f.write(")")
    
    elif isinstance(item, dict):
        if "struct" in item:
            write_struct(item, f)
        elif use_wrapper:
            f.write("WotXmlDefaultValue::EmptyDict")
        else:
            f.write("phf::phf_map! {\n")
            for k, v in item.items():
                f.write("\t")
                write_item(k, f, use_wrapper=False)
                f.write(" => ")
                write_item(v, f, use_wrapper=False)
                f.write(",")
                f.write("\n")
            f.write("}\n\n")
    elif item is None and use_wrapper:
        f.write("WotXmlDefaultValue::Null")
    else:
        if use_wrapper:
            f.write("WotXmlDefaultValue::Json(")
        f.write('"' + json.dumps(item) + '"')
        if use_wrapper:
            f.write(")")

def write_struct(item, f):
    f.write(f"{item["struct"]} {{")
    for k, v in item.items():
        if k == "struct":
            continue

        f.write(f"{k}:")
    
        write_item(v, f, use_wrapper=k=="default_value")
        f.write(",")
    f.write("}")

def write_datatype(ty: data_types.DataType, f):
    if isinstance(ty, data_types.Array):
        f.write("WotXmlType::Array {")
        f.write("inner: &")
        write_datatype(ty.type, f)
        f.write(",")
        if ty.array_size is not None:
            f.write(f"size:Some({ty.array_size})")
        else:
            f.write("size: None")
        f.write("}")
    elif isinstance(ty, data_types.FixedDict):
        f.write("WotXmlType::FixedDict{ inner: &[")
        for k, v in ty.attributes.items():
            f.write(f"(\"{k}\", ")
            write_datatype(v, f)
            f.write("),")
        f.write("],")
        f.write(f"allow_none: {"true" if ty.allow_none else "false"}}}")

    else:
        f.write(f"WotXmlType::{ty.__class__.__name__}")