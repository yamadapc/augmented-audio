from glob import glob
import os


ignored_files = [
    "midir",
    "target"
]


def any(param):
    for el in param:
        if el:
            return True
    return False


def find_preamble(target):
    target_parent = os.path.dirname(target)
    parent_files = os.listdir(target_parent)
    for f in parent_files:
        if f == "LICENSE_PREAMBLE":
            return os.path.join(target_parent, "LICENSE_PREAMBLE")
    return find_preamble(target_parent)


def find_files():
    return [
        f
        for f in glob("./crates/**/*.rs", recursive=True)
        if "vendor" not in f and "midir" not in f
    ]


def add_preamble(target):
    preamble_path = find_preamble(target)
    if preamble_path is None:
        return

    preamble_contents = None
    with open(preamble_path, 'r') as fh:
        preamble_contents = "".join(fh.readlines())

    print(f">> Processing {target}")
    data = None
    with open(target, 'r') as fh:
        data = "".join(fh.readlines())

    if "Copyright (c)" not in data and "= copyright ==" not in data:
        print(f"  {target} doesn't have contents")
        commented_contents = "\n".join([("// " + l).strip() for l in preamble_contents.splitlines()])
        new_data = commented_contents + "\n\n" + data
        print(f"Preamble for {target} found at {preamble_path}")
        with open(target, 'w') as fh:
            fh.writelines(new_data)


def main():
    files = find_files()
    for f in files:
        add_preamble(f)


if __name__ == "__main__":
    main()
