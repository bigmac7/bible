#!/bin/bash

# A script to download multiple SQLite bible files and merge them into a single 'bible.db' file.
# It merges data for tables with the same name and adds new tables for those that don't exist.

BASE_URL="https://raw.githubusercontent.com/scrollmapper/bible_databases/master/formats/sqlite/"
MAIN_DB="bible.db"

# --- Input Validation ---
if [ "$#" -lt 1 ]; then
  echo "❌ Error: You must provide at least one database filename to download."
  echo "Usage: $0 <db1.db> [db2.db] ..."
  echo "Example: $0 KJV.db ASV.db"
  exit 1
fi

# --- Main Script ---

# If bible.db doesn't exist, create it from the first argument.
if [ ! -f "$MAIN_DB" ]; then
    echo "ℹ️  '${MAIN_DB}' not found. Creating it from the first argument: $1."
    FIRST_DB="$1"
    shift # The rest of the arguments will be merged in the main loop.

    echo "➡️  Attempting to download ${FIRST_DB}..."
    wget -q --show-progress "${BASE_URL}${FIRST_DB}"
    if [ ! -f "$FIRST_DB" ]; then
      echo "❌  Error: Download of ${FIRST_DB} failed. Cannot create ${MAIN_DB}."
      exit 1
    fi
    # Rename the first downloaded DB to be our main DB.
    mv "$FIRST_DB" "$MAIN_DB"
    echo "✅  Success! '${MAIN_DB}' has been created from '${FIRST_DB}'."
fi

# Loop through all arguments to merge them into bible.db
for DB_TO_MERGE in "$@"; do
    # Skip if the file to merge is the main db itself (can happen if bible.db is passed as arg)
    if [ "$DB_TO_MERGE" == "$MAIN_DB" ]; then
        continue
    fi

    echo "➡️  Attempting to download ${DB_TO_MERGE}..."
    wget -q --show-progress "${BASE_URL}${DB_TO_MERGE}"
    if [ ! -f "$DB_TO_MERGE" ]; then
      echo "❌  Error: Download of ${DB_TO_MERGE} failed. Skipping."
      continue
    fi
    echo "✅  '${DB_TO_MERGE}' has been saved."

    echo "🔄  Merging ${DB_TO_MERGE} into ${MAIN_DB}..."

    # Get table lists as arrays
    read -r -a MAIN_TABLES <<< "$(sqlite3 "$MAIN_DB" ".tables")"
    read -r -a MERGE_TABLES <<< "$(sqlite3 "$DB_TO_MERGE" ".tables")"

    # Prepare SQL commands for the merge logic
    SQL_COMMANDS="ATTACH DATABASE '$DB_TO_MERGE' AS toMerge; BEGIN;"

    for TABLE in "${MERGE_TABLES[@]}"; do
        # Check if table exists in MAIN_DB
        if [[ " ${MAIN_TABLES[@]} " =~ " ${TABLE} " ]]; then
            # Table exists, append data. Use INSERT OR IGNORE to skip duplicate rows.
            SQL_COMMANDS+="INSERT OR IGNORE INTO main.$TABLE SELECT * FROM toMerge.$TABLE;"
        else
            # Table does not exist, create it from the source.
            SQL_COMMANDS+="CREATE TABLE main.$TABLE AS SELECT * FROM toMerge.$TABLE;"
        fi
    done

    SQL_COMMANDS+="COMMIT; DETACH DATABASE toMerge;"

    # Execute the merge commands
    sqlite3 "$MAIN_DB" "$SQL_COMMANDS"

    if [ $? -eq 0 ]; then
        echo "✅  Merge successful."
        rm "$DB_TO_MERGE"
        echo "🗑️  Removed temporary file '${DB_TO_MERGE}'."
    else
        echo "❌  Error: Merge of ${DB_TO_MERGE} failed."
    fi
done

echo "🎉 All databases merged into '${MAIN_DB}'."
