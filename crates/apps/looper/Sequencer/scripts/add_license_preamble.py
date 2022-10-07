from glob import glob
import subprocess


def is_file_ignored(f):
    if f.startswith("./build"):
        return True
    ignore_out = subprocess.run(["git", "check-ignore", f])
    return ignore_out.returncode == 0


def find_files():
    return [
        f for f in glob("./**/*.swift", recursive=True)
        if not is_file_ignored(f)
    ]


def add_preamble(preamble_contents, target):
    print(f">> Processing {target}")
    data = None
    with open(target, 'r') as fh:
        data = "".join(fh.readlines())

    if "= copyright =====" not in data:
        print(f"  {target} doesn't have contents")
        commented_contents = "\n".join([("// " + l).strip() for l in preamble_contents.splitlines()])
        new_data = commented_contents + "\n" + data
        with open(target, 'w') as fh:
            fh.writelines(new_data)


def main():
    preamble_contents = "".join(open("./LICENSE_PREAMBLE").readlines())
    files = find_files()
    for f in files:
        add_preamble(preamble_contents, f)


if __name__ == "__main__":
    main()
