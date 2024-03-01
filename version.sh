pnpm changeset version

NEW_VERSION=$(grep -m 1 '"version"' clarity-types-ts/package.json | awk -F '"' '{print $4}')
echo "New version for 'clarity-types-ts/package.json' is $NEW_VERSION"

echo "Updating 'clarity-types-ts/src/bin.ts'"
awk -v new_version="\"$NEW_VERSION\"" '!done {done=sub(".version\\(\"0.1.0\")", ".version\("new_version")")} 1' clarity-types-ts/src/bin.ts \
    > clarity-types-ts/src/bin.temp.ts && mv clarity-types-ts/src/bin.temp.ts clarity-types-ts/src/bin.ts

echo "Updating 'clarity-types/Cargo.toml'"
awk -v new_version="\"$NEW_VERSION\"" '!done {done=sub("version = \".*\"", "version = "new_version"")} 1' clarity-types/Cargo.toml \
    > clarity-types/Cargo.temp.toml && mv clarity-types/Cargo.temp.toml clarity-types/Cargo.toml

# update native version
NATIVE_NEW_VERSION=$(grep -m 1 '"version"' native/package.json | awk -F '"' '{print $4}')
echo "New version for 'native/package.json' is $NATIVE_NEW_VERSION"

echo "Updating 'native/Cargo.toml'"
awk -v new_version="\"$NATIVE_NEW_VERSION\"" '!done {done=sub("version = \".*\"", "version = "new_version"")} 1' native/Cargo.toml \
    > native/Cargo.temp.toml && mv native/Cargo.temp.toml native/Cargo.toml
