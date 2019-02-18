---
title: A Platform Agnostic Thread Pool for Haxe / OpenFL
slug: a-platform-agnostic-thread-pool-for-haxe-openfl
author: kenton
published: 2014-10-07 19:44:11
tags: [Haxe]
preview_summary: With modern hardware utilizing multiple cores, it can be highly advantageous to do as much parallel processing as possible. I think the most elegant way of doing this is to use thread pools which allocate tasks to a limited number of threads. Unfortunately, multi-threading support isn’t fully implemented in Haxe—but it is on the neko and cpp targets, so I wrote a simple thread pool to take advantage of multi-threading on those platforms!
---

With modern hardware utilizing multiple cores, it can be highly advantageous to do as much [parallel processing](http://en.wikipedia.org/wiki/Parallel_computing) as possible. I think the most elegant way of doing this is to use [thread pools](http://en.wikipedia.org/wiki/Thread_pool_pattern) which allocate tasks to a limited number of threads. Unfortunately, multi-threading support isn't fully implemented in Haxe&mdash;but it is on the neko and cpp targets, so I wrote a simple thread pool to take advantage of multi-threading on those platforms!

<!-- PELICAN_END_SUMMARY -->

The `ThreadPools` class is pretty easy to use, and I've added some features which enable you to leave it in your source code despite what platform you're compiling against&mdash;if you compile against neko or cpp, full multi-threading will be enabled; if you compile against anything else, the class will still work & compile fine, it just won't run multi-threaded.

Here's some sample code which runs two tasks which take a different amount of time to run:

```haxe
// this will create a thread pool with 8 threads on neko and cpp platforms
// on all other platforms, no threads will be created
// and the pool will use the main thread
var threadPool:ThreadPool = new ThreadPool(8);
 
// add a task that will take a while to complete
threadPool.addTask(function(x:Dynamic):Int {
    var li:Int = 0;
    for (i in 0...10)
    {
        li += i;
        for(n in 0...10000) {}
    }
    return li;
}, null, onFinish);
 
// add a task that returns right away
threadPool.addTask(function(x:Dynamic):String {
    return "herp derp";
}, null, onFinish);

// this is a blocking call that will run all the tasks
// across the pool's threads
// or just in the main thread if not on neko or cpp
threadPool.blockRunAllTasks();
 
// ...
 
// report the results of the above tasks
private function onFinish(x:Dynamic):Void
{
    trace(x);
}
```

On neko or cpp, this will output:

```
herp derp
45
```

Since the "herp derp"-returning class will finish much sooner than the summing task when they're both running in parallel.

On all other platforms, this will output:

```
45
herp derp
```

Since the tasks will be executed in the order they were added.

The code is available in a Gist: [hamaluik / ThreadPool.hx](https://gist.github.com/hamaluik/80fb81f84ecedbe2a6af).