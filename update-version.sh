#!/bin/bash

# Get the current version from Cargo.toml
current_version=$(grep '^version' Cargo.toml | cut -d '"' -f2)

# Prompt the user for the new version
read -p "Enter the new version: " new_version

# Update the version in Cargo.toml
sed -i "s/version = \"$current_version\"/version = \"$new_version\"/" Cargo.toml

# Update the version in PKGBUILD
sed -i "s/pkgver=$current_version/pkgver=$new_version/" PKGBUILD

echo "Version updated to $new_version"