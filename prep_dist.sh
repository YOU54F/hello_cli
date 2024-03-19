#!/bin/bash

if [ -z "$1" ]; then
    echo "Usage: $0 <target> [--binary-name <name>]"
    exit 1
fi

TARGET=$1
BINARY_NAME="pact_cli"
OUTPUT_DIR="dist"

while [[ $# -gt 0 ]]; do
    key="$1"
    case $key in
        --binary-name)
            BINARY_NAME="$2"
            shift
            shift
            ;;
        --output-dir)
            OUTPUT_DIR="$2"
            shift
            shift
            ;;
        *)
            shift
            ;;
    esac
done
# Rename targets to friendlier names end user names
DIST_TARGET_NAME=${TARGET}
DIST_TARGET_NAME=${DIST_TARGET_NAME//-unknown-/-}
DIST_TARGET_NAME=${DIST_TARGET_NAME//-pc-/-}
DIST_TARGET_NAME=${DIST_TARGET_NAME//-apple-darwin/-macos}

echo "DIST_TARGET_NAME: ${DIST_TARGET_NAME}"
mkdir -p ${OUTPUT_DIR}
## Proces executables
echo "Processing executables"
    cp target/${TARGET}/release/${BINARY_NAME} ${OUTPUT_DIR}/${BINARY_NAME}-${DIST_TARGET_NAME}

## Process shared libs
echo "Processing shared libraries"
for file in target/${TARGET}/release/*.{a,so,dll,dll.lib,dylib}; do
    echo "Processing $file for $DIST_TARGET_NAME"
    # get file extension
    extension="${file##*.}"
    # get filename without extension
    filename="${file%.*}"
    # remove both extensions if filename ends with .dll.lib
    if [[ $file == *".dll.lib" ]]; then
        filename="${filename%.*}"
        extension="dll.${extension}"
    fi
    DIST_TARGET_FILE="${filename}-${DIST_TARGET_NAME}.${extension}"
    echo "Renaming $file to $DIST_TARGET_FILE"
    cp "$file" "${DIST_TARGET_FILE}"
    # get full filename without base path
    new_file_name="${DIST_TARGET_FILE##*/}"
    mv "${DIST_TARGET_FILE}" "${OUTPUT_DIR}/${new_file_name}"
done