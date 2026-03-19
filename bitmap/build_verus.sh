# Two ways to build verus

# 1. Build verus from source
git clone https://github.com/verus-lang/verus.git
cd verus/source/
bash ./tools/get-z3.sh
source ../tools/activate
vargo build --release
VERUS_PATH=$(pwd)/target-verus/release
# It is recommended to add VERUS_PATH to .bashrc

# 2. Download pre-built verus from docker
# docker pull ghcr.io/verus-lang/verus:0.2024.10.31.141c73e