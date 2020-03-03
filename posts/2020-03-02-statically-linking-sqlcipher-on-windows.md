---
title: "Statically Linking SQLCipher on Windows (x64)"
slug: statically-linking-sqlcipher-on-windows
author: kenton
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

## Edit SQLCipher‘s Makefile

Start by cloning SQLCipher somewhere: