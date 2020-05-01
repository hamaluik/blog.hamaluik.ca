---
title: "Build Your Own Static Site Generator"
slug: build-your-own-static-site-generator
author: kenton
tags: [Programming]
published: 2020-01-05T16:33:00-07:00
summary: "Static site generators are fairly popular tools for creating websites, and there are a glut of them available to choose from. But writing your own generator for each site is relatively easy and straightforward and allows you significant more flexibility and ease-of-use."
section: Articles
---

[Static site generators](https://www.staticgen.com/) are fairly popular tools these days for creating websites. Static site generators typically work by taking a collection of markdown files and converting them into HTML files using templates, and are generally geared towards blogs (although they can be used to create just about any site or portion thereof). These generators represent a middle ground between manually creating an entire website from scratch in HTML and using a [CMS](https://en.wikipedia.org/wiki/Content_management_system) such as [Wordpress](https://wordpress.com/) while retaining significant advantages over each option:

1. As their name implies, they generate _static_ websites, meaning the files can be served and cached very easily by servers with virtually no processing required
2. They don't come with security issues (unlike wordpress) because there isn't any code being executed on the server (other than delivering the raw files of course)
3. They make it much easier to author content than writing HTML directly, as you typically write in [Markdown](https://daringfireball.net/projects/markdown/) instead of HTML
4. They make styling much easier than writing HTML directly, as you can write templates (for example using [Jinja](https://palletsprojects.com/p/jinja/) or [Handlebars](https://handlebarsjs.com/) so you only have to edit one file to change the layout of all the pages on the site).

## Why Write Your Own Generator?

If you take a look at https://www.staticgen.com/ you'll see there are already a ton of static website generators out there—so why bother writing your own? For me, this glut of generators gives me two problems:

1. With thousands to choose from, how do I narrow it down to one that works exactly how I want without investing a few weeks evaluating a bunch of the options?
2. These generators were all written with other peoples' use cases in mind—option A may feature Jinja templates but not syntax highlighting while option B may require a custom template language but does have syntax highlighting, etc.

Again, given just how many static site generators exist, you could probably find one that works well enough for you. The thing is—you don't have to, as it turns out its rather easy to write your own static site generator that is built exactly for your exact use case. Even though I have developed tons of projects of this scope (and larger), it never occurred to me to write my own generator until I got curious about how the excellent [Game Programming Patterns](http://gameprogrammingpatterns.com/) website was generated and took a look at the [source](https://github.com/munificent/game-programming-patterns/). I encourage you to take a look at the repository—everything is straight-to-the-point, and only what is absolutely required to build the site is present.

Ultimately, the thing I like the most about writing your own generator is that you can customize it for yourself exactly the way you want! For example, my generator renders [KaTeX](https://katex.org/) equations at compile time so my site works 100% without Javascript, it compiles plantuml diagrams from inline code in the source to inline `svg` elements, it renders syntax using [Pygments](https://pygments.org/), and it uses [Commonmark](https://commonmark.org/) with several extensions instead of plain Markdown, and it uses [Tera templates](https://tera.netlify.com/). I wrote the generator with all its bells and whistles in a few hours, which is way less time than I would have spent browsing https://www.staticgen.com/ to find something with those exact features that I wanted.

## Writing Your Own Generator

The script to generate the website for _Game Programming Patterns_ is in [format.py](https://github.com/munificent/game-programming-patterns/blob/master/script/format.py), and aside from a few custom things thrown in in order to build navigation and wrangle the styling, the process boils down to the following:

1. Find all files that end in `.markdown` in the source folder
2. Format each `.markdown` file into an `.html` file
3. Do secondary tasks such as re-compile the stylesheets and calculate a word count for the entire book

Really, all we need to do is get a list of files in a directory, get the contents of each file (both things that will be included in the standard library of pretty much any programming language), and format each file using your markdown library of choice. While Bob Nystrom used _Python_ for generating _Game Programming Patterns_, if you're writing your own generator you can use any language you want! For this website, I am currently using [Rust](https://www.rust-lang.org/) (because why not?), and the process is basically the same (with a few bells and whistles thrown in along the way):

1. [I get the list of `.md` files in the `posts` folder](https://github.com/hamaluik/blog.hamaluik.ca/blob/e3aece2aff03eb283457855aa883beec3ee08086/src/main.rs#L7-L31)
2. [I format each file into a `.html` version and save it to disk](https://github.com/hamaluik/blog.hamaluik.ca/blob/e3aece2aff03eb283457855aa883beec3ee08086/src/main.rs#L44-L57)
3. [I generate the index by rendering an index template](https://github.com/hamaluik/blog.hamaluik.ca/blob/e3aece2aff03eb283457855aa883beec3ee08086/src/main.rs#L71-L83)
4. [I copy over all static assets to the output directory](https://github.com/hamaluik/blog.hamaluik.ca/blob/e3aece2aff03eb283457855aa883beec3ee08086/src/main.rs#L87-L116)

In my sources, I make use of [YAML](https://yaml.org/) frontmatter—that is, each `.md` post starts with a block of text which describes meta-data about the post (this is fairly common in static site generators). It is fairly easy to [parse](https://github.com/hamaluik/blog.hamaluik.ca/blob/e3aece2aff03eb283457855aa883beec3ee08086/src/post/mod.rs#L34-L62), but using frontmatter certainly isn't required—the magic of writing your own generator is that you get to do whatever you want!

## Downsides To Creating Your Own Generator

Of course, creating your own generator for each site isn't the be-all, end-all solution to all of your problems. The first, most obvious downside is that you actually have to write the damned thing in the first place (rather than just downloading some framework and using that). I would argue that unless you have no programming experience, this isn't as big a hurdle as you might expect (I would further argue that writing a generator is a great programming project to introduce you to programming in general, or for picking up a new language).

Going down this route can also lend itself to a rather excessive amount of [yak shaving](https://en.wiktionary.org/wiki/yak_shaving) if you're not careful. This can be somewhat mitigated by ensuring you only include features you _absolutely_ need right now (not what you _think_ you might need someday in the future). In other words, just don't shave the yak, you dummy 😉.

Another downside is that since you've effectively started a new project, there is _no one else_ to maintain it—you can't just run your package manager to pull in new features and bug fixes. You're not just the boss, but the accountant and the janitor and everyone in between. This shouldn't be a huge deal given the rather small scope and complexity of a static site generator, but it is something to be aware of.

There are other downsides that you may run into if you decide to go down this path, but in my opinion these don't outweigh the positives of doing so. The best judge will be yourself, and I encourage you to take an afternoon to whip up something simple for your existing blog or whatever content you have laying around to get a sense of just how straightforward this can be.

## A Template To Start From

To help you get started writing your own generator, here is a simple Python script that can serve as a jumping off point:

```python
#!/usr/bin/env python3
import os
import glob
import markdown

if not os.path.exists('public'):
    os.mkdir('public')

for f in glob.iglob('book/*.md'):
    with open(f, 'r') as file:
        raw = file.read()
        html = markdown.markdown(raw)

    file_name = os.path.basename(f)
    destination = os.path.join("public", os.path.splitext(file_name)[0] + ".html")

    with open(destination, 'w') as file:
        file.write(r'''<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <title>My Great Site</title>
</head>
<body>
''')
        file.write(html)
        file.write(r'''
</body>
</html>''')
```

This script does the bare minimum:

1. It collects each `.md` file in the `books` directory
2. It parses the markdown and converts it to html
3. It writes the result out with a minimal template to the `public` directory

There are a ton of places you can take this small starting point—you can use a proper templating engine (though you don't have to), generate an index, manage assets, add metadata to each file, etc. The nice thing is that you only need to add what you want!

