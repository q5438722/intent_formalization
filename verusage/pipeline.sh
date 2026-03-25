#!/usr/bin/env bash
set -euo pipefail

WORKSPACE_DIR="./workspace"
PROMPT_FILE="prompts/test_gen_consistency.md"
SLEEP_SECS=20

total=0
skipped=0
processed=0

find ./source-projects -wholename "*/verified/*.rs" | while read -r file; do
  total=$((total + 1))
  task_name="$(basename "$file" .rs)"
  task_dir="${WORKSPACE_DIR}/${task_name}"

  # Skip if task directory exists and contains summary.md (completed)
  if [[ -d "$task_dir" && -f "${task_dir}/summary.md" ]]; then
    skipped=$((skipped + 1))
    continue
  fi

  processed=$((processed + 1))
  echo "[${processed}] Processing $file (task: ${task_name})"

  if [[ -d "$task_dir" ]]; then
    echo "  -> Incomplete task detected, retrying..."
  fi

  copilot --model "claude-opus-4.6-1m" -p "$(cat "$PROMPT_FILE")
   The target file is $file" --allow-all

  sleep "${SLEEP_SECS}s"
done

echo "Done. Skipped: ${skipped}, Processed: ${processed}, Total: ${total}"