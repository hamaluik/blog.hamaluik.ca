---
title: "Statically Linking SQLCipher on Windows (x64)"
slug: statically-linking-sqlcipher-on-windows
author: kenton
published: 2020-03-03T10:29:00-07:00
tags: [Programming]
summary: "SQLCipher is a handy “extension” to SQLite3 which provides encryption to SQLite3 databases. It is readily accessible as a static library on Linux, but the community edition doesn't provide support for building a static library on Windows. Here are instructions for doing just that."
---

SQLCipher is a handy "extension" to SQLite3 which provides encryption to SQLite3 databases. It is readily accessible as a static library on Linux, but the community edition doesn't provide support for building a static library on Windows. Fortunately, doing so is fairly straightforward, it more or less requires only 3 steps:

1. Obtain a static version of the OpenSSL libraries
2. Edit the supplied `Makefile.msc` to link to static OpenSSL
3. Compile!

## Obtaining Statically-Included OpenSSL Libraries

By default,

Head on over to https://github.com/microsoft/vcpkg and follow the instructions to be able to run `vcpkg`:

```
> cd ~/Downloads
> git clone https://github.com/microsoft/vcpkg.git
> cd vcpkg

PS> .\bootstrap-vcpkg.bat
PS> .\vcpkg integrate install
```

Now you can install the static OpenSSL libraries.

For `x64`:

```
PS> .\vcpkg install openssl:x64-windows-static
```

For `x86`:

```
PS> .\vcpkg install openssl:x86-windows-static
```

This will take a bit of time, but once done, you can find the resulting include and lib files at `Downloads\vcpkg\packages\openssl-windows_x64-windows-static` (for `x64`, `Downloads\vcpkg\packages\openssl-windows_x86-windows-static` for 32-bit).

## Edit _SQLCipher_‘s Makefile

Start by cloning _SQLCipher_ somewhere:

```
> git clone https://github.com/sqlcipher/sqlcipher.git
> cd sqlcipher
```

Make your life a little easier by copying the `openssl-windows_x64-windows-static` and `openssl-windows_x86-windows-static` folders into the _SQLCipher_ folder, or change the paths in the variables below to point to the right point.

Now open `Makefile.msc` in a text editor and replace the lines:

```
# Flags controlling use of the in memory btree implementation
#
# SQLITE_TEMP_STORE is 0 to force temporary tables to be in a file, 1 to
# default to file, 2 to default to memory, and 3 to force temporary
# tables to always be in memory.
#
TCC = $(TCC) -DSQLITE_TEMP_STORE=1
RCC = $(RCC) -DSQLITE_TEMP_STORE=1
```

by the following:

```
# Flags controlling use of the in memory btree implementation
#
# SQLITE_TEMP_STORE is 0 to force temporary tables to be in a file, 1 to
# default to file, 2 to default to memory, and 3 to force temporary
# tables to always be in memory.
#
TCC = $(TCC) -DSQLITE_TEMP_STORE=2
RCC = $(RCC) -DSQLITE_TEMP_STORE=2

TCC = $(TCC) -DSQLITE_HAS_CODEC
RCC = $(RCC) -DSQLITE_HAS_CODEC

!IF "$(PLATFORM)"=="x64"
TCC = $(TCC) -I"openssl-windows_x64-windows-static\include"
RCC = $(RCC) -I"openssl-windows_x64-windows-static\include"
!ELSEIF "$(PLATFORM)"=="x86"
TCC = $(TCC) -I"openssl-windows_x86-windows-static\include"
RCC = $(RCC) -I"openssl-windows_x86-windows-static\include"
!ENDIF

!IF "$(PLATFORM)"=="x64"
LTLIBPATHS = $(LTLIBPATHS) /LIBPATH:"openssl-windows_x64-windows-static\lib"
LTLIBS = $(LTLIBS) libcrypto.lib libssl.lib
!ELSEIF "$(PLATFORM)"=="x86"
LTLIBPATHS = $(LTLIBPATHS) /LIBPATH:"openssl-windows_x86-windows-static\lib"
LTLIBS = $(LTLIBS) libcrypto32.lib libssl32.lib
!ENDIF

# for OpenSSL: https://github.com/openssl/openssl/blob/3d362f190306b62a17aa2fd475b2bc8b3faa8142/NOTES.WIN#L112
LTLIBS = $(LTLIBS) WS2_32.Lib Gdi32.Lib AdvAPI32.Lib Crypt32.Lib User32.Lib
```

Basically, what this replacement does is it changes `SQLITE_TEMP_STORE` to default to store temproary files in memory rather than files (a requirement for _SQLCipher_), tells _SQLite_ that there is a codec (again, a requirement for _SQLCipher_), and then includes the _OpenSSL_ static libraries we obtained earlier. The additional librares (`WS2_32.Lib`, etc) are included because _OpenSSL_ requires them, see [OpenSSL / Linking your application](https://github.com/openssl/openssl/blob/3d362f190306b62a17aa2fd475b2bc8b3faa8142/NOTES.WIN#L112) for more details.

## Compile SQLCipher

With our additions to the makefile we can now compile _SQLCipher_. To do so, launch the _Visual Studio Native Tools_ command prompt from your start menu: either `VS2019 x64 Native Tools Command Prompt` (for 64-bit) or `VS2019 x86 Native Tools Command Prompt` (for 32-bit), then navigate to the _SQLCipher_ folder. Finally, make it:

```
> nmake /f Makefile.msc
```

This will take a bit, but at the end it should spit out all the binaries you need: `libsqlite3.lib`, `sqlite3.dll`, `sqlite3.exe`, etc. Note that most tools expect _SQLCipher_ binaries to replace “sqlite3” by “sqlcipher”, so you may want to rename the files.

## Compile Your Code Using Static SQLCipher

Note that `libsqlite3.lib` requires a few definitions which are found in the `libcrypto.lib` and `libssl.lib` libraries, so in order to link `libsqlite3.lib`, you'll also have to link `libcrypto.lib` and `libssl.lib`. How you do this depends on your language, but for [Rust](https://www.rust-lang.org/), it's as easy as adding the following `build.rs` script beside your package manifest:

```rust
#[cfg(windows)]
fn main() {
    // include libcrypto and libssl for static sqlcipher to work
    println!("cargo:rustc-link-lib=libcrypto");
    println!("cargo:rustc-link-lib=libssl");
}

#[cfg(unix)]
fn main() {}
```

There is probably a way to include just the symbols that _SQLCipher_ needs in `sqlcipher.lib` and not rely on `libcrypto.lib` and `libssl.lib` after compiling `libsqlite3.lib`, but I don't know enough Windows makefile voodoo to make that happen—if you do, please [let me know](mailto:kenton@hamaluik.ca)!
