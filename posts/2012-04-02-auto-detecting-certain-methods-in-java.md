---
title: Auto-Detecting Certain Methods in Java
slug: auto-detecting-certain-methods-injava
author: kenton
published: 2012-04-02T00:00:00-07:00
tags: [Java]
summary: Although I still have reservations about using Java for publishing large projects (I find these inevitably become slow and clunky due to the JVM), Java is great from a programming standpoint. One thing I especially love about Java is the ability to do run-time "reflections" which allow you to hook into all the loaded classes at run time and do all kinds of crazy things. Combine this with custom Java annotations, and you get a very easy way to write a scriptable interface for a program for example.
---

Although I still have reservations about using Java for publishing large projects (I find these inevitably become slow and clunky due to the JVM), Java is great from a programming standpoint. One thing I especially love about Java is the ability to do run-time "reflections" which allow you to hook into all the loaded classes at run time and do all kinds of crazy things. Combine this with custom Java annotations, and you get a very easy way to write a scriptable interface for a program for example.

This is exactly what I needed / wanted for a project I'm currently working on—I want the user to be able to call certain internal java functions at will through a console or script. One way you might do this is to write an interface that takes all the functions you want the user to be able to access, keeps them in a mapped list, then when the user calls a function do a lookup an call the method a certain command is linked to. As it turns out, this is long, boring, and can be especially cumbersome when I want to add a new function: I would have to go back into that mapping for each function I add an add a call definition. What I want is just to mark each function as I write is as "scriptable" and let my program take care of the scripting business from there. If you don't get what I mean, here is an example function that will be auto-detected by my code, and will be callable by the user (**all** I would have to include is the following!):

```java
@ScriptInfo(alias = "quit", args = {}, description = "exits the game")
public static boolean quit() {
	Game.quit();
	return true;
}
```

Here, the `@ScriptInfo` bit is our marker for the function. `args` will describe the argument names (since Java reflection can only get argument types, not their names), and `description` is a short description of the function for when the user calls for help about the function.

In order to get this up and running then, we first need a list of all classes that are in our classpath that match our package. This can be easily done with the following code:

```java
@SuppressWarnings("rawtypes")
private static Class[] getClasses(String packageName) throws ClassNotFoundException, IOException {
	ClassLoader classLoader = Thread.currentThread().getContextClassLoader();
	assert classLoader != null;
	String path = packageName.replace('.', '/');
	Enumeration resources = classLoader.getResources(path);
	List dirs = new ArrayList();
	while (resources.hasMoreElements()) {
		URL resource = resources.nextElement();
		dirs.add(new File(resource.getFile()));
	}
	ArrayList classes = new ArrayList();
	for (File directory : dirs) {
		classes.addAll(findClasses(directory, packageName));
	}
	return classes.toArray(new Class[classes.size()]);
}

@SuppressWarnings("rawtypes")
private static List findClasses(File directory, String packageName) throws ClassNotFoundException {
	List classes = new ArrayList();
	if (!directory.exists()) {
		return classes;
	}
	File[] files = directory.listFiles();
	for (File file : files) {
		if (file.isDirectory()) {
			assert !file.getName().contains(".");
			classes.addAll(findClasses(file, packageName + "." + file.getName()));
		} else if (file.getName().endsWith(".class")) {
			classes.add(Class.forName(packageName + '.' + file.getName().substring(0, file.getName().length() - 6)));
		}
	}
	return classes;
}
```

Once we have a list of all classes in our package, we need to filter them based on whether they inherit our `Scriptable` class. We could skip this and just search through all classes, but I want all of my user-callable methods to have access to a `println` function which will (somewhat obviously) print data out to the console. This `println` function lives in a higher class called `Scriptable`, then all classes which contain user-callable methods can simply extend the `Scriptable` class and have access to the `println` function. This filtering is very simple to do:

```java
// get all classes in our desired package
Class[] classes = getClasses("your.package.here");
// check out our classes
for(int i = 0; i < classes.length; i++) {
	// see if it's scriptable
	// make sure it isn't the "Scriptable" class itself
	// then make sure that it inherits from the "Scriptable" class
	if(!classes[i].getSimpleName().equals("Scriptable") && Scriptable.class.isAssignableFrom(classes[i])) {
		// yup, we found a scriptable class!
		// now go through and register all methods in the class marked as callable methods
		registerCommands(classes[i]);
	}
}
```

Now that we have filtered our list of classes, we call `registerCommands()` on each class. `registerCommands()` is a function which explores a given class and searches through all of it's methods for methods that have been marked by us as user-callable. We do this by iterating over the run-time annotations that each method has, and if we found our annotation then we store that method. The following code which does this is fairly well-documented, and should be easy to follow along with:

```java
@SuppressWarnings("rawtypes")
public static void registerCommands(Class cls) {
	// loop over all the methods in the class
	Method[] methods = cls.getMethods();
	for(int i = 0; i < methods.length; i++) {
		// get the method's annotations
		Annotation[] annotations = methods[i].getAnnotations();
		for(int j = 0; j < annotations.length; j++) {
			if(annotations[j] instanceof ScriptInfo) {
				// we found our annotation
				ScriptInfo si = (ScriptInfo)annotations[j];

				// create the command name
				String commandName = si.alias();
				// get the command args
				Class[] params = methods[i].getParameterTypes();
				for(int k = 0; k < params.length; k++) {
					// append the arguments to it so that each one is unique
					commandName += ":" + params[k].getSimpleName();
				}

				// create the command info
				CommandInfo ci = new CommandInfo();
				ci.alias = si.alias();
				ci.argNames = si.args();
				ci.argDescriptions = si.argDescriptions();
				ci.description = si.description();
				if(si.longDescription().equals("")) {
					ci.longDescription = si.description();
				}
				else {
					ci.longDescription = si.longDescription();
				}
				ci.method = methods[i];

				// and add the command!
				commands.put(commandName, ci);
			}
		}
	}
}
```

Note that here I am storing each command in a hashmap with a key that includes both the function name and it's argument signature (since there can be multiple functions with the same name, but different argument signatures). I then store all the information about the function in a container class so I can get information about it later.

Note that after calling all of this, I have a list of all functions in my package that have been marked as user-callable at their definition. Nowhere did I have to explicitely define functions—if I want to change the description of a function, I change it at the definition of the function; if I want to add an extra user-callable function, I make sure that the class it exists in extends `Scriptable` then I add the `@Scriptinfo` annotation definition before the method definition, and I'm done!

In order to get this running properly, you'll need a couple more things including actually parsing what the user types in and calling the appropriate method (I'll put this into a separate post later), and the ScriptInfo annotation definition and CommandInfo container definitions, both of which are included below:

```java
import java.lang.reflect.Method;

public class CommandInfo {
	public String alias;
	public String[] argNames;
	public String[] argDescriptions;
	public String description;
	public String longDescription;
	public Method method;
}
```

```java
import java.lang.annotation.ElementType;
import java.lang.annotation.Retention;
import java.lang.annotation.RetentionPolicy;
import java.lang.annotation.Target;

@Retention(RetentionPolicy.RUNTIME)
@Target(ElementType.METHOD)
public @interface ScriptInfo {
	public String alias();
	public String[] args() default {};
	public String[] argDescriptions() default {};
	public String description();
	public String longDescription() default "";
}
```
