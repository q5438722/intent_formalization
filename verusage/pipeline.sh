find ./source-projects -wholename "*/verified/*.rs" | while read file; do
  echo "Processing $file"
  copilot --model "claude-opus-4.6-1m" -p "$(cat prompts/test_gen_revised.md)
   The target file is $file"  --allow-all
   sleep 150s
done