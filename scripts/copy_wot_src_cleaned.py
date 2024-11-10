from pathlib import Path
import shutil
import re
import git

from versions import VERSIONS_FROM_NEW_REPO

WOT_SRC = Path("../wot-src")
TARGET_DIR = Path("./wot_xml")

def main():
    repo = git.Repo(WOT_SRC)
    repo.git.checkout("EU")
    repo.git.pull()
    commits = list(repo.iter_commits())

    for version in VERSIONS_FROM_NEW_REPO:
        # -1 is used to emit the last number, which is not significant to us TODO: it turns out you do need micropatches
        commit_msg_prefix = "v." + ".".join(version.split("_")[:-1])

        print(f"Prefix: {commit_msg_prefix}")
        
        # There may be multiple matches but we will take the latest one of a particular patch
        # Later NOTE: it turns out you do need micropatches
        for c in commits:
            if c.message.strip().startswith(commit_msg_prefix):
                repo.git.checkout(c)
                target_path = TARGET_DIR / version

                shutil.copytree(
                    src=WOT_SRC, 
                    dst=target_path, 
                    dirs_exist_ok=True,
                    ignore=lambda src, names: [name for name in names if Path(name).suffix not in [".xml", ".def", ""]]
                 )

                for dir_entry in TARGET_DIR.rglob("*"):
                    if dir_entry.suffix in [".xml", ".def"]:
                        clean_file(dir_entry)
                
                break

def clean_file(file_path):
    with open(file_path, "r+") as file:
        xml_str = file.read()
        modified = remove_namespaces_from_xml(xml_str)
        file.seek(0)
        file.write(modified)
        file.truncate()
    


def remove_namespaces_from_xml(xml_str):
    REMOVE_PATTERNS = [
        r"(<xmlns:xsi>.*?</xmlns:xsi>)\n",
        r"(<xsi:noNamespaceSchemaLocation>.*?</xsi:noNamespaceSchemaLocation>)\n",
    ]
    
    for match in REMOVE_PATTERNS:
        xml_str = re.sub(match, "", xml_str)

    return xml_str


if __name__ == "__main__":
    main()
