#!/usr/bin/env bash

echo "🔧 Copying your local editor configs..."

# Cursor
mkdir -p .cursor
cp -v ~/.cursor/environment.json .cursor/environment.json 2>/dev/null
cp -v ~/.cursor/rules .cursor -R 2>/dev/null

echo "✅ Done!"

