#!/bin/bash

# A flexible script to download any SQLite bible file from the specified GitHub folder.

# The base URL of the directory where the files are stored.
BASE_URL="https://raw.githubusercontent.com/scrollmapper/bible_databases/master/formats/sqlite/"

# --- Input Validation ---
# Check if the user provided exactly one argument. The variable '$#' holds the count of arguments.
if [ "$#" -ne 1 ]; then
  echo "❌ Error: You must provide a filename to download."
  echo "Usage: $0 <filename.db>"
  echo "Example: $0 ASV.db"
  exit 1
fi

# --- Main Script ---
# The filename is the first argument passed to the script ($1).
FILENAME="$1"
FULL_URL="${BASE_URL}${FILENAME}"

echo "➡️  Attempting to download ${FILENAME}..."

# Use wget to download the file specified by the user.
wget "$FULL_URL"

# Check if the file was downloaded successfully.
if [ -f "$FILENAME" ]; then
  echo "✅  Success! '${FILENAME}' has been saved to your current directory."
else
  echo "❌  Error: Download failed."
  echo "   Please check that the filename '${FILENAME}' is correct and exists in the repository."
  exit 1
fi
