import subprocess


def main():
    out = subprocess.check_output("./scripts/dev.sh list-crates --simple".split(" ")).decode().strip().split("\n")
    crates = [
        crate.strip()
        for crate in out
        if crate.strip()
    ]

    print("version: 2")
    print("updates:")
    for crate_str in crates:
        scrate = crate_str.split(" ")
        pth = scrate[0]
        crate = scrate[1]
        print('  - package-ecosystem: "cargo"')
        print('    directory: "%s"' % pth)
        print('    schedule:')
        print('      interval: "daily"')


if __name__ == '__main__':
    main()
