#!/usr/bin/env bash


read -p "Enter tag version (e.g., v1.0.0): " version

if [ -z "$version" ]; then
  echo "Error: Tag version cannot be empty."
  exit 1
fi

git tag -a "$version" -m "Release $version"
git push origin "$version"