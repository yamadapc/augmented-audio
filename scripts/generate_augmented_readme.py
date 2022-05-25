import os.path

import toml
from glob import glob


category_descriptions = {
    'application': 'Stand-alone app functionality',
    'audio': 'Audio app functionality',
    'data': 'Data-structures',
    'dsp': 'DSP',
    'gui': 'GUI',
    'ops': 'Operations',
    'testing': 'Testing',
}


def build_category_readme(category_path):
    if not os.path.isdir(category_path):
        return None
    if 'development' in category_path:
        return None

    category = os.path.basename(category_path)
    if category == 'augmented':
        return None
    output = ""

    augmented_crates = set([
        f
        for f in glob(f"{category_path}/**/Cargo.toml", recursive=True)
        if "target" not in f and "vendor" not in f and "midir" not in f
    ])
    augmented_crates = list(augmented_crates)
    augmented_crates.sort()
    description = category_descriptions[category]
    output += f"# [**{category}** - {description}]({category})\n\n"
    for crate in augmented_crates:
        t = parse_cargo_toml(crate)
        package_name = t['package']['name']
        package_description = t['package'].get('description', None)
        package_description = f" - {package_description}" if package_description is not None else ""
        output += f"* [**{package_name}**{package_description}]({category}/{os.path.basename(os.path.dirname(crate))})\n"

    return output


def build_main_readme():
    categories = glob("./crates/augmented/*")
    categories.sort()
    output = ""
    output += "# augmented\n"
    output += "The augmented audio libraries are separated onto categories:\n\n"
    output += "\n## Summary\n\n"
    for category_path in categories:
        if not os.path.isdir(category_path):
            continue
        if 'development' in category_path:
            continue

        category = os.path.basename(category_path)
        if category == 'augmented':
            continue
        augmented_crates = set([
            f
            for f in glob(f"{category_path}/**/Cargo.toml", recursive=True)
            if "target" not in f and "vendor" not in f and "midir" not in f
        ])
        augmented_crates = list(augmented_crates)
        augmented_crates.sort()
        description = category_descriptions[category]
        output += f"* [**{category}** - {description}]({category})\n"
    output += "\n## All crates\n\n"
    for category_path in categories:
        if not os.path.isdir(category_path):
            continue
        if 'development' in category_path:
            continue

        category = os.path.basename(category_path)
        if category == 'augmented':
            continue
        augmented_crates = set([
            f
            for f in glob(f"{category_path}/**/Cargo.toml", recursive=True)
            if "target" not in f and "vendor" not in f and "midir" not in f
        ])
        augmented_crates = list(augmented_crates)
        augmented_crates.sort()
        description = category_descriptions[category]
        output += f"* [**{category}** - {description}]({category})\n"
        for crate in augmented_crates:
            t = parse_cargo_toml(crate)
            package_name = t['package']['name']
            package_description = t['package'].get('description', None)
            package_description = f" - {package_description}" if package_description is not None else ""
            output += f"  * [**{package_name}**{package_description}]({category}/{os.path.basename(os.path.dirname(crate))})\n"

    output += "\n## Internal tooling\n"
    output += "* [**development** - Development tools](development)\n"

    return output


def parse_cargo_toml(path):
    with open(path, 'r') as f:
        return toml.loads(f.read())


def main():
    readme = build_main_readme()
    with open("./crates/augmented/README.md", 'w') as f:
        f.write(readme)
    categories = glob("./crates/augmented/*")
    categories.sort()
    for category_path in categories:
        readme = build_category_readme(category_path)
        if readme is not None:
            with open(f"{category_path}/README.md", 'w') as f:
                f.write(readme)


if __name__ == '__main__':
    main()
