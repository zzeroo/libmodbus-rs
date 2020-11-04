rem Matrix-driven Appveyor CI script for libmodbus-rs
rem Currently only does MSYS2 builds.
rem https://www.appveyor.com/docs/installed-software#mingw-msys-cygwin
rem Needs the following vars:
rem     MSYS2_ARCH: x86_64 or i686
rem     MSYSTEM:    MINGW64 or MINGW32
rem     target:     x86_64-pc-windows-gnu, i686-pc-windows-gnu
rem     channel:    stable, beta, nightly
rem This script based on the work of the libmypaint team.

rem Download rust
appveyor DownloadFile https://win.rustup.rs/ -FileName rustup-init.exe
rustup-init -yv --default-toolchain %channel% --default-host %target%
set PATH=%PATH%;%USERPROFILE%\.cargo\bin
rustc -vV
cargo -vV

rem Set the paths appropriately
PATH C:\msys64\%MSYSTEM%\bin;C:\msys64\usr\bin;%PATH%

rem Upgrade the MSYS2 platform
bash -lc "pacman --noconfirm --sync --refresh --refresh pacman"
bash -lc "pacman --noconfirm --sync --refresh --refresh --sysupgrade --sysupgrade"

rem Install required tools
bash -xlc "pacman --noconfirm -S --needed base-devel"

rem Install the relevant native dependencies
bash -xlc "pacman --noconfirm -S --needed mingw-w64-%MSYS2_ARCH%-clang"

rem Invoke subsequent bash in the build tree
cd %APPVEYOR_BUILD_FOLDER%
set CHERE_INVOKING=yes

rem Build/test scripting
bash -xlc "set pwd"
bash -xlc "env"

cargo test --verbose