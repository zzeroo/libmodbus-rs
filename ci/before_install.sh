set -e

# Install clang-3.9.0
# https://github.com/KyleMayes/clang-sys/blob/87b674e9274c50995a6a4601e1160165490cb0e7/ci/before_install.sh

export LLVM_VERSION_TRIPLE="3.9.0"
export LLVM=clang+llvm-${LLVM_VERSION_TRIPLE}-x86_64-$1

wget http://llvm.org/releases/${LLVM_VERSION_TRIPLE}/${LLVM}.tar.xz
mkdir llvm
tar xf ${LLVM}.tar.xz -C llvm --strip-components=1

export LLVM_CONFIG_PATH=`pwd`/llvm/bin/llvm-config

if [ "${TRAVIS_OS_NAME}" == "linux" ]; then
    llvm_download linux-gnu-ubuntu-14.04
    export LD_LIBRARY_PATH=`pwd`/llvm/lib:$LD_LIBRARY_PATH
else
    llvm_download apple-darwin
    export DYLD_LIBRARY_PATH=`pwd`/llvm/lib:$DYLD_LIBRARY_PATH
fi
