import * as glob from "glob";
import * as path from "path";
import semver, { SemVer } from "semver";

interface Package {
  path: string;
  pkgJson: any;
}

interface Dependency {
  name: string;
  version: SemVer;
}

const lernaJson = require("../lerna.json");

const packagePaths = lernaJson.packages.flatMap((pattern: string) =>
  glob.sync(`${pattern}/package.json`)
);
console.log("Going to validate packages:", packagePaths.join(", "));
const packageJsons = [
  { path: path.join(__dirname, ".."), pkgJson: require("../package.json") },
  ...packagePaths.map((packageDir: string) => ({
    path: path.join(__dirname, "..", packageDir),
    pkgJson: require("../" + packageDir),
  })),
];

function getDependencies(pkg: Package): Dependency[] {
  const allDependencyEntries: [string, string][] = Object.entries(
    pkg.pkgJson.dependencies ?? {}
  )
    .concat(Object.entries(pkg.pkgJson.devDependencies ?? {}))
    .concat(Object.entries(pkg.pkgJson.peerDependencies ?? {})) as [
    string,
    string
  ][];

  return allDependencyEntries
    .map(([name, version]) => ({
      name,
      version: semver.parse(
        version.replace("^", "").replace("~", "")
      ) as SemVer,
    }))
    .filter(({ version }) => version != null);
}

const dependencyList = packageJsons.flatMap((pkg) => getDependencies(pkg));
const dependencyVersions: { [name: string]: SemVer[] } = dependencyList.reduce(
  (memo: { [name: string]: SemVer[] }, { name, version }) => {
    if (memo[name] == null) {
      memo[name] = [];
    }
    memo[name].push(version);
    return memo;
  },
  {}
);

const duplicateVersions = Object.entries(dependencyVersions).filter(
  ([_name, versions]) => {
    const formattedVersions = versions.map((version: SemVer) =>
      version.format()
    );
    const versionSet = new Set(formattedVersions);
    return versionSet.size > 1;
  }
);

duplicateVersions.forEach(([name, versions]) => {
  const formattedVersions = versions.map((version) => version.format());
  const versionSet = new Set(formattedVersions);
  console.log(
    `${name} has ${versionSet.size} versions: ${formattedVersions.join(", ")}`
  );
  const hasDifferentMajorVersion =
    new Set(versions.map((version) => version.major)).size > 1;
  if (hasDifferentMajorVersion) {
    console.log(
      `${name} has more than one major version. This will break things.`
    );
    process.exit(1);
  }

  const bestVersion = formattedVersions.sort((version1, version2) => {
    const v1 = semver.parse(version1) as any;
    const v2 = semver.parse(version2) as any;
    return v2.compare(v1);
  })[0];
  console.log(`npm install ${name}@${bestVersion}`);
});
