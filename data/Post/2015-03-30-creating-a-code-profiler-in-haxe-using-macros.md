---
title: Creating a Code Profiler in Haxe Using Macros
slug: creating-a-code-profiler-in-haxe-using-macros
author: kenton
published: 2015-03-30 22:21:55
category: programming
tags: Haxe, Haxe Macros
preview_summary: Haxe macros are said by many to be “black magic”, and in some ways they are—macros in Haxe are actual Haxe code (rather than macros in C/C++ for example, which are just fancy pre-processor directives). Macros are undoubtedly the most advanced feature of Haxe and probably the hardest to grasp, however I think a major reason for that is a lack of really solid documentation on the subject. To me, the Haxe docs regarding macros are somewhat obfuscated and leave something to be desired, and unfortunately there’s not a lot of other resources online. This is what led me to embark on my own journey of learning Haxe macros, and hopefully starting to shed a little more light on their mystery.
---

[Haxe macros](http://haxe.org/manual/macro.html) are said by many to be "black magic", and in some ways they are&mdash;macros in Haxe are actual Haxe code (rather than macros in C/C++ for example, which are just fancy pre-processor directives). Macros are undoubtedly the most advanced feature of Haxe and probably the hardest to grasp, however I think a major reason for that is a lack of really solid documentation on the subject. To me, the Haxe docs regarding macros are somewhat obfuscated and leave something to be desired, and unfortunately there's not a lot of other resources online. This is what led me to embark on my own journey of learning Haxe macros, and hopefully starting to shed a little more light on their mystery.

I had some trouble getting this example off the ground, so I would like to thank [ousado](https://github.com/ousado) for helping me out on the [Haxe IRC](http://webchat.freenode.net/?channels=haxe). Haxe is truly a great and supportive community, and if you're ever stuck with something I strongly encourage you to pop on the IRC&mdash;you're bound to have someone help you out!

<!-- PELICAN_END_SUMMARY -->

I think I first realized the potential of Haxe macros after reading [underscorediscovery](http://underscorediscovery.com/)'s [post](http://notes.underscorediscovery.com/haxe-compile-time-macros/) on the subject&mdash;before that I had largely ignored macros as I got on just fine without them. Then I saw something that caught my eye: _profiling and instrumentation_. I'm currently working on formalizing my personal game engine ([Woolli](https://github.com/BlazingMammothGames/Woolli)) that I've somewhat developed during the course of some projects. The engine is an entity-component-system engine. Right now there is very limited profiling support through inheritence on the systems that are used (but _none_ of the engine code is profiled). Although this isn't much of a problem yet as things _seem_ to run smoothly, I just know that one day things are going to break down and when that day comes, a profiler will be an invaluable tool to have. Anyway: back to [underscorediscovery](http://underscorediscovery.com/)'s post. In it, he mentions the possibility for doing such a thing but declines to dig into any code. What a shame! Naturally, I've attempted to bridge that gap by writing some code which I will present here.

Since I also want this to be a bit of a learning experience rather than just a code dump, I will attempt to walk you through what I achieved, how I achieved it, and why I did the things that I did. I'll also show you some of my false starts so you can hopefully learn from my mistakes!

## A Brief Introduction to Haxe Macros

There are three main types of Haxe macros:

1. Expression macros (these are simple functions that can be placed pretty much anywhere in your code for macro-ey goodness)
2. Build macros (these are applied at compilation time using the `@:build` metadata and are generally used for modifying code in-place)
3. Initialization macros (these use the `--macro` command line parameter)

So far I've only used expression and build macros, so I can't comment much on initialization macros. Expression and build macros are easy to set up&mdash;the trick lies in thinking like the compiler to get things done properly. For the profiling code I'm writing, a **build** macro is the most suitable as I want to go through all of a class' functions and modify them (rather than have to explicitely call a function like I would with an expression macro).

### Build Macros

Build macros get applied to your classes generally through one of two ways: using an `@:build` or an `@:autoBuild` [metadata](http://haxe.org/manual/cr-metadata.html). The difference between these two macros is that `@:build` macros get applied only to the class they're written on top of, whereas `@:autoBuild` macros get applied only to the descendets of the class they're written above. See the example below to see what I mean:

```haxe
/**
 * This will be the class that applies the build macro to other classes
 */
class Profiler
{
	macro public static function profile():Array<Field>
	{
		// do nothing for now
		return Context.getBuildFields();
	}
}

/**
 * When this class gets compiled, the Profiler.profile() function will be
 * called (with the context within the class)
 */
@:build(Profiler.profile())
class ProfiledClassA
{
	public function new() {}

	public function doSomething():Float { return 3.141592; }
}

/**
 * When this class gets compiled, the Profiler.profile() WON'T be called on it
 * (though it will be called on its descendents)
 */
@:autoBuild(Profiler.profile())
class ProfiledClassB
{
	public function new() {}

	public function doSomethingElse():Float { return 5; }
}

/**
 * When this class gets compiled, the Profiler.profile() will be called on it
 * (since it inherits from ProfiledClassB which has `@:autoBuild` on it)
 */
class ProfiledClassC extends ProfiledClassB
{
	public function new() {}
}

/**
 * When this class or any class that inherits from it gets compiled, the 
 * Profiler.profile() function will be called
 */
@:build(Profiler.profile())
@:autoBuild(Profiler.profile())
class ProfiledClassD
{
	public function new() {}

	public function doSomethingElseEntirely():Float { return 42; }
}
```

Hopefully that makes sense. If not, please leave a comment below and I'll try to help you out!

## Profiling

In order to profile my code I want to be able to measure the execution time of each function (as well as how many times any given function was called). If I were to do this manually, I would change something like this:

```haxe
class SomeClass
{
	public function new() {}
	public function doSomething():Float
	{
		var x:Float = 0;
		for(i in 0...100000)
			x += 0.1;
		return x;
	}
}
```

into something like:

```haxe
class SomeClass
{
	public function new() {}
	public function doSomething():Float
	{
		Profiler.startProfile('SomeClass', 'doSomething');
		var x:Float = 0;
		for(i in 0...100000)
			x += 0.1;
		Profiler.endProfile('SomeClass', 'doSomething');
		return x;
	}
}
```

Here, the `startProfile` function would be responsible for recording a timestamp of when the function began and `endProfile` would be responsible for recording a timestamp of when the function finished and adding the time difference between the two to the function's elapsed time. Here is a profiling class that does exactly that:

```haxe
typedef MethodProfile = {
	var calls:Int;
	var startTime:Float;
	var elapsedTime:Float;
}

class Profiler
{
	private static var profiles:StringMap<StringMap<MethodProfile>> = new StringMap<StringMap<MethodProfile>>();
	
	/**
	 * Reset all the profiling information. Doing this before reading / printing the information will
	 * cause all the data collected since the beginning (or last reset) to be lost
	 */
	public static function reset()
	{
		profiles = new StringMap<StringMap<MethodProfile>>();
	}
	
	/**
	 * Called at the start of a function to record when in time the method was called. This must always
	 * be called BEFORE an endProfile() call is made
	 * @param	className	the fully-qualified class name of the method's class
	 * @param	methodName	the name of the method being profiled
	 */
	public static function startProfile(className:String, methodName:String)
	{
		// make sure the profiles exist
		if (!profiles.exists(className))
			profiles.set(className, new StringMap<MethodProfile>());
		if (!profiles.get(className).exists(methodName))
			profiles.get(className).set(methodName, { calls: 0, startTime: 0, elapsedTime: 0 } );
		
		#if DEBUG_PROFILING Lib.println("> Starting " + className + "." + methodName); #end
		
		profiles.get(className).get(methodName).calls++;
		profiles.get(className).get(methodName).startTime = Sys.cpuTime();
	}
	
	/**
	 * Called at the end of a function to calculate the method's execution time. This must always
	 * be called AFTER a startProfile() call
	 * @param	className	the fully-qualified class name of the method's class
	 * @param	methodName	the name of the method being profiled
	 */
	public static function endProfile(className:String, methodName:String)
	{
		var t:Float = Sys.cpuTime();
		
		if (!profiles.exists(className) || !profiles.get(className).exists(methodName))
			throw "EndProfile was called on a function that was never started!";
			
		profiles.get(className).get(methodName).elapsedTime += t - profiles.get(className).get(methodName).startTime;
		#if DEBUG_PROFILING Lib.println("< Ending " + className + "." + methodName); #end
	}
	
	/**
	 * Just a utility function to print the profiling data, separated by class.
	 */
	public static function printProfiles():Void
	{
		var totalTime:Float = 0;
		for (className in profiles.keys())
		{
			var classTime:Float = 0;
			Lib.println(className + ":");
			for (methodName in profiles.get(className).keys())
			{
				Lib.println("  ." + methodName + ": " + profiles.get(className).get(methodName).elapsedTime + "s (" + profiles.get(className).get(methodName).calls + " calls)");
				classTime += profiles.get(className).get(methodName).elapsedTime;
			}
			Lib.println("  ---");
			Lib.println("  " + classTime + "s");
			totalTime += classTime;
		}
		
		Lib.println("");
		Lib.println("Total time: " + totalTime + "s");
	}
}
```

I made the above fairly verbose so it's easy to see what's going on. The `startProfile` and `endProfile` functions should work exactly as I've described them to, and we could use them exactly like I've already mentioned: calling `startProfile` at the start of the function and `endProfile` at the end of the function&mdash;but that would be tedious and prone to errors (what if we forgot to include those function calls somewhere else?). It would also tend to clutter our code up something fierce and then the profiling code would still be there on a release build (which would be entirely unnecessary!). This is where a Haxe macro will come very much in handy, as it will automatically transform our code for us to insert those profiling calls for debug builds and do nothing for release builds.

## Bringing in the Macro

What we want the macro to do is loop through every function in a given class, and before any other statement in the class, we want to add the [expression](http://haxe.org/manual/expression.html) `Profiler.startProfile(className, methodName);`. The end the profiling, we have to be a little bit more careful about how we deal with return statements&mdash;if we were to simply tack on a call to `Profiler.endProfile(className, methodName);` at the end of the function it would never be reached for any function that has a return statement anywhere, for example:

```haxe
/**
 * The base function we want to profile
 */
function example(x:Bool):Float
{
	if(x)
		return 42;
	return 39;
}

/**
 * The "naively" profiled function
 */
function example(x:Bool):Float
{
	Profiler.startProfile("example", "example"); // this would be injected at the start of the function by our macro
	if(x)
		return 42; // if we get here, the function will immediately return and _not_ call `endProfile`
	return 39;
	Profiler.endProfile("example", "example"); // this code will _never_ be reached (the function returns before we get to it!)
}
```

What we instead want to do is transform any return statements into a block that looks like:

```haxe
{
	var ___tempProfilingReturnValue = $oldReturnExpression;
	Profiler.endProfile("example", "example");
	return ___tempProfilingReturnValue;
}
```

This way we can not only capture the execution time of the return expression (this would be important if you did something like `return someReallySlowFunction();`), but also ensure that our `endProfile` function gets called no matter where the function exits. Note that we will also have to make sure that if the final statement in a function isn't `return ...` then we still need to tack on the `endProfile` statement onto the end of the function.

Thus we would want to transform our example function into the following:

```haxe
function example(x:Bool):Float
{
	Profiler.startProfile("example", "example");
	if(x)
		{
			var ___tempProfilingReturnValue = 42;
			Profiler.endProfile("example", "example");
			return ___tempProfilingReturnValue;
		}
	{
		var ___tempProfilingReturnValue = 39;
		Profiler.endProfile("example", "example");
		return ___tempProfilingReturnValue;
	}
}
```

Once this logic makes sense to you, we can move on to actually implementing the macro.

### Implementing the Macro

First things first, we need to define our macro function. The syntax for a build macro function is as follows:

```haxe
macro public static fuction profile():Array<Field>
{
	// get the fields of the class
	var fields:Array<Field> = Context.getBuildFields();

	// transform those fields
	// ...

	// return the transformed fields
	return fields;
}
```

What this skeleton does is get an array of [fields](http://haxe.org/manual/class-field.html) which make up the class and allow you to transform them (for example you could completely nuke a variable for release builds and create special variables for debug builds). Since we're not adding or removing any fields, we can just loop through all of the fields and look for methods:

```haxe
for(field in fields)
{
	// look for methods
	switch(field.kind)
	{
		// yup, found a method!
		case FFun(func):
		{
			// get the name of the function
			var methodName:String = field.name;
		}

		// ignore variables and properties
		default: {}
	}
}
```

The above skeleton finds each method in a class and grabs its name (which isn't terribly useful on its own, obviously).

### Injecting the `startProfile` Code

Since all we need for the `startProfile` code is for it to be prepended to the function definition, we can redefine the function using a macro:

```haxe
// yup, found a method!
case FFun(func):
{
	// get the name of the method
	var methodName:String = field.name;
	
	// prepend the start code to the function
	func.expr = macro {
		Profiler.startProfile($v { clsName }, $v { methodName } );
		$ { func.expr };
	};
}
```

There's a couple things going on here that need explaining. Firstly, the `macro` statement: using the `macro` statement before an expression is called "[reification](http://haxe.org/manual/macro-reification.html)", which is a fancy way of saying that the expression will be compiled into code. The expression can be any valid Haxe code, and within that expression you can use a variety of "[escapes](http://haxe.org/manual/macro-reification-expression.html)" (think using a `\"` in a string to escape a quote character.) In fact, the `$v{clsName}` and `$v{methodName}` shown above are examples of those escapes which will get replaced by the values of the `clsName` and `methodName` variables. Thus, the macro expression

```haxe
macro { Profiler.startProfile($v{clsName}, $v{methodName}); }
```

will be converted at compile-time to the following:

```haxe
Profiler.startProfile("exampleClass", "example");
```

(assuming of course that `clsName == "exampleClass"` and `methodName == "example"`).

Similarly, the `${func.expr};` line will get replaced by the original function expression, so if the function was originally defined as:

```haxe
function example(x:Bool):Float
{
	if(x)
		return 42;
	return 39;
}
```

It will now be defined as:

```haxe
function example(x:Bool):Float
{
	Profiler.startProfile("exampleClass", "example");
	{
		if(x)
			return 42;
		return 39;
	}
}
```

### Injecting the `endProfile` Code

Injecting the `endProfile` code is a little bit more complicated than simply prepending (or in the "end" case: appending) a statement as we did for the `startProfile` code, though the principle is largely the same. The difference is that now we have to loop through the remaining expressions and replace return expressions by the special `endProfile` block we defined earlier. However this involves more than simply iterating through the expressions&mdash;some expressions within the function will more than likely be blocks, or if statements, or some other encapsulating expression that we will need to step into to search for a return statement. This is a perfect candidate for [recursion](http://en.wikipedia.org/wiki/Recursion). To perform this recursive search, we will create a function called `replaceExpressionReturn` (I'm terrible at names, I get it). It will be called to replace the entire function's expression as such:

```haxe
// yup, found a method!
case FFun(func):
{
	// get the name of the method
	var methodName:String = field.name;
	
	// prepend the start code to the function
	func.expr = macro {
		Profiler.startProfile($v { clsName }, $v { methodName } );
		$ { func.expr };
	};

	// start the recursive expression transformation
	func.expr = replaceExpressionsReturn(clsName, methodName, func.expr);
}
```

#### Recursively Searching An Expression for Return Statements

Now that we have our function signature down we can start actually defining the function:

```haxe
/**
 * This is a recursive function which will tunnel into a function's expressions and replace any
 * occurrances of a return expression with a custom profiling return expressions
 * @param	clsName	a string representing the host class name
 * @param	methodName a string representing the method name
 * @param	expr	the current expression to operate on (starting at the function definition)
 * @return	the transformed expression
 */
private static function replaceExpressionsReturn(clsName:String, methodName:String, expr:Expr):Expr
{
	// look for specific expression types
	switch(expr.expr)
	{
		// our beloved return expression
		case EReturn(retExpr):
		{
			// TODO: transform the return expression!
		}
		
		// don't transform anything else
		default: { }
	}
	return expr;
}
```

So far this function is very basic, and doesn't actually transform the expression at all. All we do is check the type of the current expression that we're investigation (`expr`), and branch the macro depending on that type. So far all we're looking for is a return statement. Once we've found a return statement, we can create a new expression using our profiling block and use it to replace the return expression:

```haxe
case EReturn(retExpr):
{
	// we found a return expression, change it to our special block!
	return macro {
		var ___tempProfilingReturnValue = ${retExpr};
		Profiler.endProfile($v { clsName }, $v { methodName } );
		return ___tempProfilingReturnValue;
	};
}
```

Now, same as before, we're constructing a `macro` to use as the expression. This time, the macro is three lines of code (rather than a single line as before), but that doesn't really matter. Assuming you understood what happened before with the `startProfile` code, this should all be fairly self-explanatory. The only real "new" thing is the use of `${retExpr}`, which is simply an escape that will print out the old return expression, so something like:

```haxe
return 5 * Math.sin(2 * Math.PI * f) + 42;
```

would be transformed into:

```haxe
{
	var ___tempProfilingReturnValue = 5 * Math.sin(2 * Math.PI * f) + 42;
	Profiler.endProfile("exampleClass", "exampleMethod");
	return ___tempProfilingReturnValue;
}
```

If you run this code as-is, you'll note that the `endProfile` code will never be injected, even if you have a function that _only_ has a return statement:

```haxe
function example():Float
{
	return 42;
}
```

Since the `startProfile` code has already been injected, the above would currently transform into:

```haxe
function example():Float
{
	Profiler.profile("exampleClass", "example");
	{
		return 42;
	}
}
```

Why is this? We check for a return expression and when we find it, we change it? As it turns out, the first expression in the function definition would actually be a [Block](http://haxe.org/manual/expression-block.html) (an [EBlock](http://api.haxe.org/haxe/macro/ExprDef.html#EBlock) [ExprDef](http://api.haxe.org/haxe/macro/ExprDef.html)). So, we should handle that case:

```haxe
// a block ({})
case EBlock(blockExprs):
{
	var i = 0;
	while (i < blockExprs.length)
	{
		blockExprs[i] = replaceExpressionsReturn(clsName, methodName, blockExprs[i]);
		i++;
	}
	return macro $b { blockExprs };
}
```

According to the API documentation, a block is represented by [EBlock](http://api.haxe.org/haxe/macro/ExprDef.html#EBlock), which is basically just `Array<Expr>`. Thus, we can iterate over the each expression in the block and transform it using our recursive function. Once all the expressions in the block are processed, we can return a macro representing the block, using the `$b{}` [escape pattern](http://haxe.org/manual/macro-reification-expression.html) to convert the array of expressions into a block expression.

Filling out the block expression case helps us solve our problem above, but we still have several other cases when a `return` expression could be nested away. Namely, this is a problem in the following classes:

* [Blocks](http://haxe.org/manual/expression-block.html): [EBlock](http://api.haxe.org/haxe/macro/ExprDef.html#EBlock)
* [For loops](http://haxe.org/manual/expression-for.html): [EFor](http://api.haxe.org/haxe/macro/ExprDef.html#EFor)
* [While loops](http://haxe.org/manual/expression-while.html) / [Do-While loops](http://haxe.org/manual/expression-do-while.html): [EWhile](http://api.haxe.org/haxe/macro/ExprDef.html#EWhile)
* [Ifs](http://haxe.org/manual/expression-if.html): [EIf](http://api.haxe.org/haxe/macro/ExprDef.html#EIf)
* [Switches](http://haxe.org/manual/expression-switch.html): [ESwitch](http://api.haxe.org/haxe/macro/ExprDef.html#ESwitch)
* [Try / Catches](http://haxe.org/manual/expression-try-catch.html): [ETry](http://api.haxe.org/haxe/macro/ExprDef.html#ETry)

All that's left to do to deal with these situations is to handle their cases and recursively call the function on each expression found within each situation. I won't go through the details of doing this for every sitatution as it's all pretty much the same as the block case. I'll just list how I handled those cases here:

```haxe
// a block ({})
case EBlock(blockExprs):
{
	var i = 0;
	while (i < blockExprs.length)
	{
		blockExprs[i] = replaceExpressionsReturn(clsName, methodName, blockExprs[i]);
		i++;
	}
	return macro $b { blockExprs };
}

// a for loop
case EFor(it, forExpr):
{
	forExpr = replaceExpressionsReturn(clsName, methodName, forExpr);
	return macro  {
		for ($ { it } )
		{
			$ { forExpr };
		}
	};
}

// a while loop
case EWhile(cond, whileExpr, _):
{
	whileExpr = replaceExpressionsReturn(clsName, methodName, whileExpr);
	return macro  {
		while ($ { cond } )
		{
			$ { whileExpr };
		}
	};
}

// an if statement
case EIf(cond, ifExpr, elseExpr):
{
	ifExpr = replaceExpressionsReturn(clsName, methodName, ifExpr);
	if (elseExpr != null)
		elseExpr = replaceExpressionsReturn(clsName, methodName, elseExpr);
	if (elseExpr == null)
	{
		return macro {
			if ($ { cond } )
			{
				$ { ifExpr };
			}
		};
	}
	else
	{
		return macro {
			if ($ { cond } )
			{
				$ { ifExpr };
			}
			else
			{
				$ { elseExpr };
			}
		};
	}
}

// a switch statement
case ESwitch(switchExpr, cases, defaultExpr):
{
	for (cas in cases)
	{
		cas.expr = replaceExpressionsReturn(clsName, methodName, cas.expr);
	}
	if(defaultExpr != null)
		defaultExpr = replaceExpressionsReturn(clsName, methodName, defaultExpr);
	return macro {
		$expr;
	}
}

// try / catch statements
case ETry(tryExpr, catches):
{
	tryExpr = replaceExpressionsReturn(clsName, methodName, tryExpr);
	for (cat in catches)
	{
		cat.expr = replaceExpressionsReturn(clsName, methodName, cat.expr);
	}
	return macro {
		$expr;
	}
}
```

Now we're most of the way&mdash;just two relatively small things stand in our way:

1. Void functions with a return statement but no expression, ie `return;`.
   * This will cause an error the way things are currently handled as we don't account for the fact that `retExpr` may be null.
2. Void functions without a return statement
   * This is a problem because we're only modifying return statements. If there isn't a return statement to modify, our `endProfile` code will never get called.

#### Dealing With Empty Return Statements

This is a relatively easy fix. All we have to do is check to see if `retExpr` is `null`, and if so, don't deal with returning anything fancy. The `EReturn` handling case can be modified as such to fix the problem:

```haxe
// our beloved return expression
case EReturn(retExpr):
{
	if (retExpr == null)
	{
		return macro {
			Profiler.endProfile($v { clsName }, $v { methodName } );
			return;
		};
	}
	else
	{
		return macro {
			var ___tempProfilingReturnValue = ${retExpr};
			Profiler.endProfile($v { clsName }, $v { methodName } );
			return ___tempProfilingReturnValue;
		};
	}
}
```

#### Dealing With Void Functions Without A Return Statement

This is going to be a little bit trickier. After a decent amount of "guess-and-test", I came up with a relatively simple, if tedious, solution: add a variable which keeps track of whether or not the most recently processed statement was a return statement or not. If we finish transforming the function and the most recent statement wasn't a return statement, then add the `endProfile` code; otherwise, do nothing.

I implemented the variable as a static class-level variable (`var lastWasReturn:Bool = false;`). In each of the `replaceExpressionsReturn` function's cases I then set this variable to false, with the exception of the return case. Then, in the main `profile()` function I add the following statement:

```haxe
if (!lastWasReturn)
{
	func.expr = macro {
		$ { func.expr };
		Profiler.endProfile($v { clsName }, $v { methodName } );
		return;
	}
}
```

I know that describing things this way isn't ideal, but instead of essentially directly copying all the code over again, I will just let you browse the entire working project as a Gist: [https://gist.github.com/FuzzyWuzzie/412e2109a5f5fbcf12e1](https://gist.github.com/FuzzyWuzzie/412e2109a5f5fbcf12e1).

## Caveats With This Profiler

There are a couple issues with this profiler that are still outstanding, namely:

* It only deals with class-level functions
* It provides no mechanism for investigating _parts_ of a function (though I would argue that you shouldn't have a function with multiple "parts" anyway)
* It doesn't profile the getters and setters of properties (which should be fairly insubstantial anyway)
* It relies on the `Sys.cpuTime()` function to calculate run time, which for many fast functions will return `0` (even when it was't truly non-zero) due to the resolution of the function. I'm not sure how to get higher resolution timing information in Haxe yet.

## Conclusions

Hopefully this gave a decent introduction to macros in Haxe (or at least, _build_ macros!) while creating a handy tool to use in future projects. If you have any questions, comments, or concerns, I'd love to hear from you in the comments. And thanks again to [ousado](https://github.com/ousado) and the [Haxe IRC](http://webchat.freenode.net/?channels=haxe) for helping me to learn the black magic that is Haxe macros!