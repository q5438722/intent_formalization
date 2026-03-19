cd /home/chentianyu/LLaMA-Factory-Verus && /home/chentianyu/miniconda3/bin/python3 spec/spec_proof_gen.py \
  --input /home/chentianyu/verus-proof-synthesis/benchmarks/VeruSAGE-Bench/tasks.jsonl \
  --output_dir /home/chentianyu/data/spec_proof_gen_verusage \
  --models claude-opus-4.5 gpt-5.2 \
  --max_workers 4 \
  --start 0 --end -1 \
  --timeout 600
