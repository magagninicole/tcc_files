import pathlib

TEMPLATE = """/*
Author: Ben Mezger (github.com/benmezger)
*/\n"""

ROOT_DIR = pathlib.Path(__file__).parent.parent.absolute()


def set_author():
    files = (ROOT_DIR / "src").glob("**/*")
    for file in files:
        if file.is_file() and file.name.endswith(".rs"):
            with open(str(file), "r+") as code:
                content = code.read()
                code.seek(0, 0)

                if TEMPLATE in content:
                    print(f"Skipping {file}")
                    continue

                code.write(TEMPLATE + "\n" + content)


if __name__ == "__main__":
    set_author()
