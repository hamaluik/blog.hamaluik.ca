---
title: Getting Started With Haxe Macros
slug: getting-started-with-haxe-macros
summary: "Like has been said many times before, Haxe macros are incredibly powerful. They don't always have the best documentation however, and I find a lot of people forgo their use entirely (instead doing things such as created nodejs scripts to copy files around for building). Hopefully I can help shed some light on how to build your own macros for those who are new to the language, or macros in general. I'll cover three macros I use on a regular basis, one each of the three types listed in the manual: an initialization macro for copying files to the build folder; a build macro for providing easy auto-completion of asset filenames (a la HaxeFlixel's AssetPaths); and an expression macro for grabbing and formatting the build version."
author: kenton
category: programming
tags: Haxe, Haxe Macros
published: 2016-09-18
---

Like has been said many times before, [Haxe](http://haxe.org/) macros are incredibly powerful. They don't always have the best documentation however, and I find a lot of people forgo their use entirely (instead doing things such as created [nodejs](https://nodejs.org/) scripts to copy files around for building). Hopefully I can help shed some light on how to build your own macros for those who are new to the language, or macros in general. I'll cover three macros I use on a regular basis, one each of the three types listed in the [manual](https://haxe.org/manual/macro.html): an initialization macro for copying files to the build folder; a build macro for providing easy auto-completion of asset filenames (a la [HaxeFlixel](http://haxeflixel.com/)'s `AssetPaths`); and an expression macro for grabbing and formatting the build version.

Before I dive into the macros, it may help to define exactly what Haxe macros are&mdash;basically, Haxe macros are just Haxe code that gets run at compile time, rather than run time. Because the code is executed during your project's compilation phase, macros thus have the ability to transform the code that is getting compiled (generally by modifying the abstract syntax tree). Macros allow you to create, programmatically, anything that you could create manually in normal Haxe source code files. For example, if you are really ambitious, you could create macros to create entire classes based off of a `.json` (or whatever other format floats your boat) description file. Or, you can automatically inject function calls into your code to create a [rudimentary profiler](https://hamaluik.com/posts/creating-a-code-profiler-in-haxe-using-macros/), or even implement [aspect-oriented programming](https://en.wikipedia.org/wiki/Aspect-oriented_programming). Or you can just use them to translate some definition variables into a string that gets used without runtime overhead. Haxe macros are similar-_ish_ to [C/C++ Macros](https://msdn.microsoft.com/en-us/library/503x3e3s.aspx), just _orders of magnitude_ more powerful.

## Initialization Macros

Initialization macros are just functions that you call in your `.hxml` file by using the `--macro` [parameter](https://haxe.org/manual/compiler-usage-flags.html). In order to better explain these macros to you, I will go through creating the macro that I often use to copy files from one directory to another. This is very useful for things such as games, where you want to copy the production-ready versions of assets from a "source" directory, into your binary directory so that when you run the game, it has access to those assets.

I like to keep my projects organized in namespaces describing groups of functionality, so I usually end up with a `macros` package. For this example, let's create the `macros` package:

```bash
mkdir src
mkdir src/macros
```

And in that package, create a file called `AssetManagement.hx`, which will be a container class for several macro functions dealing with copying asset files around:

```haxe
package macros;

class AssetManagement {
    public static function copyProjectAssets() {
        trace("Hello from copyProjectAssets()!");
    }
}
```

Now, to call this function in your `.hxml` file, simply include the line `--macro macros.AssetManagement.copyProjectAssets()`:

```hxml
-cp src

--macro macros.AssetManagement.copyProjectAssets()

-neko bin/init.n
```

Compiling this now won't do much other than notify you that we did indeed run the function:

```bash
$ haxe init.hxml 
src/macros/AssetManagement.hx:5: Hello from copyProjectAssets()!
```

It's important to note that while our macro is executing, it has access to pretty much the entire Haxe standard library. We can use that standard library to do things like interact with the file system, which is what we care about in this example. Let's get started by using `Sys` and `Path` to figure out our source and destination folders:

```haxe
package macros;

import Sys;
import haxe.io.Path;

class AssetManagement {
    public static function copyProjectAssets() {
        var cwd:String = Sys.getCwd();
        var assetSrcFolder = Path.join([cwd, "src", "assets"]);
        var assetsDstFolder = Path.join([cwd, "bin", "assets"]);

        Sys.println("I am going to copy files from:");
        Sys.println("  " + assetSrcFolder);
        Sys.println("to:");
        Sys.println("  " + assetsDstFolder);
    }
}
```

Which when compiled, results in:

```bash
$ haxe init.hxml 
I am going to copy files from:
  /home/kenton/Projects/macro-demos/src/assets
to:
  /home/kenton/Projects/macro-demos/bin/assets
```

Our function can also call other static functions in the class&mdash;in this case, a recursive file copy function (which simply uses the standard library to copy an entire directory somewhere, including all subdirectories):

```haxe
private static function copy(sourceDir:String, targetDir:String):Int {
    var numCopied:Int = 0;

    if(!FileSystem.exists(targetDir))
        FileSystem.createDirectory(targetDir);

    for(entry in FileSystem.readDirectory(sourceDir)) {
        var srcFile:String = Path.join([sourceDir, entry]);
        var dstFile:String = Path.join([targetDir, entry]);

        if(FileSystem.isDirectory(srcFile))
            numCopied += copy(srcFile, dstFile);
        else {
            File.copy(srcFile, dstFile);
            numCopied++;
        }
    }
    return numCopied;
}
```

Using this function, we can modify our original macro to copy from our source to build destinations whenever we build the project:

```haxe
package macros;

import Sys;
import haxe.io.Path;

class AssetManagement {
    private static function copy(sourceDir:String, targetDir:String):Int {
        var numCopied:Int = 0;

        if(!FileSystem.exists(targetDir))
            FileSystem.createDirectory(targetDir);

        for(entry in FileSystem.readDirectory(sourceDir)) {
            var srcFile:String = Path.join([sourceDir, entry]);
            var dstFile:String = Path.join([targetDir, entry]);

            if(FileSystem.isDirectory(srcFile))
                numCopied += copy(srcFile, dstFile);
            else {
                File.copy(srcFile, dstFile);
                numCopied++;
            }
        }
        return numCopied;
    }

    public static function copyProjectAssets() {
        var cwd:String = Sys.getCwd();
        var assetSrcFolder = Path.join([cwd, "src", "assets"]);
        var assetsDstFolder = Path.join([cwd, "bin", "assets"]);

        // make sure the assets folder exists
        if(!FileSystem.exists(assetsDstFolder))
            FileSystem.createDirectory(assetsDstFolder);

        // copy it!
        var numCopied = copy(assetSrcFolder, assetsDstFolder);
        Sys.println('Copied ${numCopied} project assets to ${assetsDstFolder}!');
    }
}
```

And there we go! Now whenever we build, our assets folder will be copied in full:

```bash
$ haxe init.hxml 
Copied 5 project assets to /home/kenton/Projects/macro-demos/bin/assets!
```

## Build Macros

