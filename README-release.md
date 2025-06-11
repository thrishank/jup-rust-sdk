# Making a New Release

To create and publish a new release of the crate to crates.io:

## 1. Update the Version

- Update the version in `jup-ag-sdk/Cargo.toml` following [semantic versioning](https://semver.org/), e.g., `0.1.5` → `0.2.0` for a minor update.

## 2. Commit the Changes

- Commit and push the version update to the main branch:
  ```sh
  git add jup-ag-sdk/Cargo.toml
  git commit -m "Bump version to vX.Y.Z"
  git push origin main
  ```

## 3. Create a Tag

- Tag the commit with the new version:
  ```sh
  git tag vX.Y.Z
  git push origin vX.Y.Z
  ```

  Replace `vX.Y.Z` with the actual version number (e.g., `v0.2.0`).

## 4. GitHub Actions Workflow

- When the tag is pushed, the `publish.yml` GitHub Actions workflow will automatically:
    - Verify that the version in the tag matches the version in Cargo.toml
    - Build the package using Cargo
    - Publish the crate to crates.io

## 5. Confirm the Release

- Check [crates.io](https://crates.io/crates/jup-ag-sdk) to ensure the new version has been published successfully.

## Troubleshooting

- If the automated publishing fails, check the GitHub Actions logs for details
- Ensure your CRATES_IO_TOKEN is properly set in the repository secrets
- For manual publishing (if needed):
  ```sh
  cargo login
  cargo publish -p jup-ag-sdk/ --dry-run  # Test run first
  cargo publish -p jup-ag-sdk/            # Actual publish
  ```