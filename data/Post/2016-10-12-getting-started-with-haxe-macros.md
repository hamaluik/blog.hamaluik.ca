---
title: Getting Started With Haxe Macros
slug: getting-started-with-haxe-macros
summary: "Like has been said many times before, Haxe macros are incredibly powerful. They don't always have the best documentation however, and I find a lot of people forgo their use entirely (instead doing things such as created nodejs scripts to copy files around for building). Hopefully I can help shed some light on how to build your own macros for those who are new to the language, or macros in general. I'll cover three macros I use on a regular basis, one each of the three types listed in the manual: an initialization macro for copying files to the build folder; a build macro for providing easy auto-completion of asset filenames (a la HaxeFlixel's AssetPaths); and an expression macro for grabbing the build date as a Date object."
author: kenton
category: programming
tags: Haxe, Haxe Macros
published: 2016-10-12
---

Like has been said many times before, [Haxe](http://haxe.org/) macros are incredibly powerful. They don't always have the best documentation however, and I find a lot of people forgo their use entirely (instead doing things such as created [nodejs](https://nodejs.org/) scripts to copy files around for building). Hopefully I can help shed some light on how to build your own macros for those who are new to the language, or macros in general. I'll cover three macros I use on a regular basis, one each of the three types listed in the [manual](https://haxe.org/manual/macro.html):

1. An [**initialization macro**](#initmacros) for copying files to the build folder
2. A [**build macro**](#buildmacros) for providing easy auto-completion of asset filenames (a la [HaxeFlixel](http://haxeflixel.com/)'s `AssetPaths`)
3. An [**expression macro**](#exprmacros) for grabbing the build date as a `Date` object.

Before I dive into the macros, it may help to define exactly what Haxe macros are&mdash;basically, Haxe macros are just Haxe code that gets run at compile time, rather than run time. Because the code is executed during your project's compilation phase, macros thus have the ability to transform the code that is getting compiled (generally by modifying the abstract syntax tree). Macros allow you to create, programmatically, anything that you could create manually in normal Haxe source code files. For example, if you are really ambitious, you could create macros to create entire classes based off of a `.json` (or whatever other format floats your boat) description file. Or, you can automatically inject function calls into your code to create a [rudimentary profiler](https://hamaluik.com/posts/creating-a-code-profiler-in-haxe-using-macros/), or even implement [aspect-oriented programming](https://en.wikipedia.org/wiki/Aspect-oriented_programming). Or you can just use them to translate some definition variables into a string that gets used without runtime overhead. Haxe macros are similar-_ish_ to [C/C++ Macros](https://msdn.microsoft.com/en-us/library/503x3e3s.aspx), just _orders of magnitude_ more powerful.

## <a name="initmacros"></a>Initialization Macros

Initialization macros are just functions that you call in your `.hxml` file by using the `--macro` [parameter](https://haxe.org/manual/compiler-usage-flags.html) (note that this means you can easily include them in libraries using [extraParams.hxml](https://haxe.org/manual/haxelib-extraParams.html)!). In order to better explain these macros to you, I will go through creating the macro that I often use to copy files from one directory to another. This is very useful for things such as games, where you want to copy the production-ready versions of assets from a "source" directory, into your binary directory so that when you run the game, it has access to those assets.

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

## <a name="buildmacros"></a>Build Macros

Build macros are special macros that automatically get executed by the compiler when compiling `class`es, `enum`s, and `abstract`s. Their purpose is generally to modify the structure of the compiled code as it is compiled&mdash;think adding, removing, and changing the fields of a class. I previously wrote about build macros in my post about [creating a code profiler](https://hamaluik.com/posts/creating-a-code-profiler-in-haxe-using-macros/), but in short a build macro could dynamically convert a class that looks like this:

```haxe
package;

@:build(macros.MyTransformer.transform())
class MyClass {
    public var a:String;
    private var b:Int;
}
```
into this:
```haxe
package;

class MyClass {
    public static var b:Int;

    public function squared():Int x*x;
}
```
(in this example we removed `a`, changed `b` to be public and static, and added a function named `squared`).

Aside from contrived examples such as this, I find that I most commonly use build macros in my own projects to tie into the asset copying system described above. When developing game code, I found myself often needing to include specific asset files, and ended up with constants like `public static var enemySpriteFileName:String = "assets/sprites/enemy.png";` littered throughout my code. When you start getting a lot of assets this very quickly becomes not feasible. Further, you don't get any protection from all-too-common things like mistyping that one file name and inexplicably having some broken system. To remedy some of these issues, HaxeFlixel has a handy utility which I fell in love with and copy in all of my projects these days. Basically, a macro adds a list of `public static` strings to a class which point to specific asset file names. No more typos, and you even benefit from auto-completion.

To do this in your own project, get started by creating an empty class which will hold the file name strings:

```haxe
package;

@:build(macros.Assets.addAssetList())
class AssetFiles {}
```

The only unique thing here is the line `@:build(macros.Assets.addAssetList())`. This is some custom [metadata](https://haxe.org/manual/lf-metadata.html) which tells the compiler to run the `addAssetList()` function while _typing_ the `AssetFiles` class. We should create that function now:

```haxe
package macros;

import haxe.macro.Context;
import haxe.macro.Expr;
import Sys;
import haxe.io.Path;

class Assets {
    public static function addAssetList():Array<Field> {
        // get all the fields in the class at this point
        // this is an array describing the variables, properties,
        // and methods in the class.
        var fields:Array<Field> = Context.getBuildFields();

        // TODO: modify the fields

        // and we're done
        return fields;
    }
}
```

Right now this class will get the list of build fields, do nothing to it, and return. Basically, it does nothing. What we want it do is change the `AssetFiles` class into the following (assuming we have the files `sprites/enemy.png` and `sounds/hit.ogg` in our assets folder):

```haxe
package;

class AssetFiles {
   public static var asset___sprites___enemy__png:String = "assets/sprites/enemy.png";
   public static var asset___sounds___hut__ogg:String = "assets/sounds/hit.ogg"; 
}
```

Let's get started by listing all our files:

```haxe
public static function addAssetList():Array<Field> {
    // get all the fields in the class at this point
    // this is an array describing the variables, properties,
    // and methods in the class.
    var fields:Array<Field> = Context.getBuildFields();

    // recursively get a list of all files in our assets folder
    var assetSrcFolder = Path.join([Sys.getCwd(), "src", "assets"]);
    var files:Array<String> = listFiles(assetSrcFolder);

    // add the fields to the class
    for(file in files) {
        var relativePath:String = file.substr(assetSrcFolder.length + 1);
        // map characters not allowed in variable names to ones that are
        var name:String = "asset___" + relativePath.split("/").join("___").split("-").join("_").split(".").join("__");
        relativePath = "assets/" + relativePath;

        // TODO: add a public static var string field called `name` with a value of `relativePath`
    }

    // and we're done
    return fields;
}
```

Now that we have an entry for each file, let's construct the fields. When I was first getting started with macros, I found this bit to be by far the most confusing. Thankfully, the [api docs](http://api.haxe.org/haxe/macro/) have improved a bit since then, but the main thing to remember is that the fields at this point are basically just anonymous structures filled with [`enum` values](https://haxe.org/manual/types-enum-instance.html). Specifically, we need to fill out the following typedef:

```haxe
typedef Field = {
    var name:String;
    @:optional var doc:Null<String>;
    @:optional var access:Array<Access>;
    var kind:FieldType;
    var pos:Position;
    @:optional var meta:Metadata;
}
```

In the `Field` typedef,

<dl>
    <dt><code>name</code></dt>
    <dd>refers to the variable / property / function name. In this case, it's the sanitized file name (<code>asset___sprites___enemy__png</code> in the above example.)</dd>
    <dt><code>doc</code></dt>
    <dd>is an optional documentation string used for autocomplete and such</dd>
    <dt><code>access</code></dt>
    <dd>is an array of <a href="http://api.haxe.org/haxe/macro/Access.html">access modifier enums</a> describing whether the field is private / public, etc.</dd>
    <dt><code>kind</code></dt>
    <dd>is the meat of the field, and is an enum describing the field: be it a variable, property, or function, along with its value (which is itself an "expression" [which is just code])</dd>
    <dt><code>pos</code></dt>
    <dd>is a variable describing where in your file the field is. If you get an error, this describes the file and line number it occurs, for instance.</dd>
    <dt><code>meta</code></dt>
    <dd>is an array of <a href="http://api.haxe.org/haxe/macro/MetadataEntry.html">metadata entries</a> for the field</dd>
</dl>

Here's how we can construct the field (note that we don't include a `meta` field; this is because it is `@:optional` in the typedef, and we don't need it&mdash;not including it is equivalent to passing `null` for it):

```haxe
// add the fields to the class
for(file in files) {
    var relativePath:String = file.substr(assetSrcFolder.length + 1);
    // map characters not allowed in variable names to ones that are
    var name:String = "asset___" + relativePath.split("/").join("___").split("-").join("_").split(".").join("__");
    relativePath = "assets/" + relativePath;

    fields.push({
        name: name,
        doc: 'Relative path for file ${file}',
        access: [Access.APublic, Access.AStatic, Access.AInline],
        pos: Context.currentPos(),
        kind: FieldType.FVar(macro: String, macro: $v{relativePath})
    });
}
```

Now we're basically done, however I think the `kind` field deserves a bit more attention before we move on. The `kind` field is of type [`FieldType`](http://api.haxe.org/haxe/macro/FieldType.html), which is an `enum` which can take the following types:

* `FVar` (variable)
* `FFun` (function)
* `FProp` (property)

Using Haxe's [`enum instances`](https://haxe.org/manual/types-enum-instance.html), each of these gets their own constructor:

* `FVar(type:ComplexType, expression:Expr)`
* `FFun(function:Function)`
* `FProp(get:String, set:String, type:ComplexType, expression:Expr)`

In our case, we're creating variables (we could create read-only properties, but I'll leave that as an exercise to the reader), so create an `FVar`. We provide the type through [class reification](https://haxe.org/manual/macro-reification-class.html) (we basically just say use the `String` class / type). We then provide the initialization expression using [expression reification](https://haxe.org/manual/macro-reification-expression.html) (we just use the compile-time value of `relativePath`).

So that's that. To use our new superpowers, just reference the `AssetFiles` class:

```haxe
var enemySprite:Sprite = loadSprite(AssetFiles.asset___sprites___enemy__png);
// equivalent to:
var enemySprite:Sprite = loadSprite("assets/sprites/enemy.png");
```

## <a name="exprmacros"></a>Expression Macros

Expression macros are certainly the easiest type of macros to grasp, however that doesn't mean they're not worth much. Expression macros are just functions that get called by the compiler at compile time, with their output substituted into your code in place of the function call.

In fact, there's a decent chance you have used an expression macro from a library before without ever knowing it. Here is what calling our build date expression macro looks like:

```haxe
var date:Date = MacroTools.dateBuilt();
```

Which is normal, everyday code. Except when you compile it, it is the same as writing:

```haxe
var date:Date = new Date(2016, 10, 12, 21, 05, 47);
```

So let's get down to writing our `dateBuilt` function:

```haxe
package macros;

import haxe.macro.Expr;
import haxe.macro.Context;

class MacroTools {
    macro public static function dateBuilt():ExprOf<Date> {
        return macro Date.now();
    }
}
```

Simple, right! There's a few things going on:

1. The function is prefixed by `macro`, which indicates its status as an expression macro
2. The function returns a type called `ExprOf<Date>`. Expression macros must return [expressions](http://api.haxe.org/haxe/macro/Expr.html) (which can easily be created through [expression reification](https://haxe.org/manual/macro-reification-expression.html)). `ExprOf<Date>` just means an expression that is constrained to the `Date` type (we might as well help the type system as much as we can!).
3. Since we need to return an expression, we use reification to convert our `Date.now()` call into an expression (using the `macro` keyword).

There's just one major problem with the above: instead of inserting the time that the project was compiled, we instead just insert the `Date.now()` expression, which will never align with our build time as it is a run-time call.

Once compiled, it will look like this:

```haxe
var date:Date = Date.now();
```

instead of what we want:

```haxe
var date:Date = new Date(2016, 10, 12, 21, 05, 47);
```

What we need to do is construct that last expression, so lets do that:

```haxe
macro private static function dateBuilt():ExprOf<Date> {
    // get the date at compile time
    var date:Date = Date.now();

    // use the values from the compile-time date to construct
    // a run-time expression
    return macro new Date(
        $v{date.getFullYear()}, $v{date.getMonth()}, $v{date.getDay()},
        $v{date.getHours()}, $v{date.getMinutes()}, $v{date.getSeconds()}
    );
}
```

Now when we run it, it will work as expected. We had to use the `$v{}` syntax to generate an expression from the `date.get_()` function calls. This is the shorthand equivalent of calling:

```haxe
$v{date.getFullYear()}
// is the same as:
Context.makeExpr(date.getFullYear(), Context.currentPos())
```

## Conclusions

Well, hopefully this rather long post helped introduce you to Haxe macros, or at least cleared things up a little bit. Please feel free to use any of the examples I've provided in your own code, and as always&mdash;don't hesitate to ask if you have any questions!