REM https://github.com/KyleMayes/clang-sys/blob/87b674e9274c50995a6a4601e1160165490cb0e7/ci/install.bat

ps: Start-FileDownload "https://static.rust-lang.org/dist/rust-nightly-${env:TARGET}.exe"
rust-nightly-%TARGET%.exe /VERYSILENT /NORESTART /DIR="C:\Program Files (x86)\Rust"
set PATH=%PATH%;C:\Program Files (x86)\Rust\bin

curl -sSf http://releases.llvm.org/%LLVM_VERSION%/LLVM-%LLVM_VERSION%-win32.exe -o LLVM.exe
7z x LLVM.exe -oC:\LLVM
set PATH=%PATH%;C:\LLVM\bin
set LIBCLANG_PATH=C:\LLVM\bin

if defined MSYS_BITS set PATH=C:\msys64\mingw%MSYS_BITS%\bin;C:\msys64\usr\bin;%PATH%
set CARGO_TARGET_DIR=%APPVEYOR_BUILD_FOLDER%\target
rustc -V
cargo -V

set RUST_BACKTRACE=1
