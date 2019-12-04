#!/usr/bin/env bash
#
# Cross compile for porting.
#
PS4='$(printf "+ \033[38;5;245m%-16s\033[0m " "${BASH_SOURCE[0]}:${LINENO}:")'

# Make sure that we have a known starting point.
SCRIPT_DIR=$(cd $(dirname ${BASH_SOURCE[0]}) && pwd)
cd $SCRIPT_DIR

set -e
for f in Dockerfile* ; do
    # Example: Dockerfile.ubuntu-18.04
    # OsName="ubuntu"
    # OsVersion="18.04"
    OsName=$(echo $f | sed -e 's@^[^\.]*\.@@' | awk -F- '{print $1}')
    OsVersion=$(echo $f | sed -e 's@^[^\.]*\.@@' | awk -F- '{print $2}')
    ImageName="rust-ports/$OsName:$OsVersion"
    set -x
    time docker build -f $f -t $ImageName .
    time docker run -it --rm --init -v $(pwd)/..:/mnt $ImageName \
         bash -ic "pwd && cargo build --target-dir rust-ports/$OsName/$OsVersion --release"
    { set +x; } 2>/dev/null
done
