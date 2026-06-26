import {
  assertWorkspaceVersionsSync,
  fail,
  readWorkspaceVersions,
} from "./release-utils.mjs";

const tag = process.argv[2] ?? process.env.GITHUB_REF_NAME ?? "";
const version = tag.replace(/^v/, "");

if (!/^\d+\.\d+\.\d+$/.test(version)) {
  fail(`Release tag must look like vX.Y.Z; got ${tag || "<empty>"}`);
}

const versions = readWorkspaceVersions();
assertWorkspaceVersionsSync(versions);

if (
  versions.core !== version ||
  versions.adapter !== version ||
  versions.adapterDependency !== version
) {
  fail(
    [
      `Release tag ${tag} does not match workspace versions:`,
      `  crates/typeship: ${versions.core}`,
      `  crates/typeship-ts-rs: ${versions.adapter}`,
      `  typeship-ts-rs dependency on typeship: ${versions.adapterDependency}`,
    ].join("\n"),
  );
}

console.log(`Release tag ${tag} matches workspace version ${version}.`);
