#!/bin/bash
# Script to extract Memory Mesh zip file
ZIP_FILE="$1"
DEST_DIR="src/services/memory-mesh"

if [ -z "$ZIP_FILE" ]; then
    echo "Usage: $0 <path-to-zip-file>"
    echo "Example: $0 ~/Downloads/CTAS7_Memory_Mesh_v2.0_RC1_Staging.zip"
    exit 1
fi

if [ ! -f "$ZIP_FILE" ]; then
    echo "Error: File not found: $ZIP_FILE"
    exit 1
fi

echo "Extracting $ZIP_FILE to $DEST_DIR..."
mkdir -p "$DEST_DIR"
unzip -q "$ZIP_FILE" -d "$DEST_DIR"
echo "✅ Extraction complete!"
ls -la "$DEST_DIR"
