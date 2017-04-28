---
title: "Building a Collision Engine Part 2: 2D Penetration Vectors"
slug: building-a-collision-engine-part-2-2d-penetration-vectors
author: kenton
tags: [Math, Haxe]
published: 2017-05-02
meta-image: /assets/images/collision-engine-2d-detection/meta-preview.jpg
large-meta-image: true
preview-image: /assets/images/collision-engine-2d-detection/2d_yes_slow.gif
preview-summary: ""
---

In my [last post](/posts/building-a-collision-engine-part-1-2d-gjk-collision-detection/), I discussed how to determine if any two convex shapes in two dimensions overlap. The result of this calculation is a boolean state&mdash;either the two are colliding or they aren't. This is great for things such as triggers, but if you want to use the collision engine for any type of physics calculations, you'll need at least one more crucial piece of information: the penetration vector.

The penetration vector of two shapes is a vector which describes the overlap of those two shapes. By adding the penetration vector to one of the shapes, the shapes can be separated so that they are just touching, but not really intersecting. Without any extra information, this vector chosen as the one that separate the two shapes along the shortest path possible, which can lead to some issues at the corners of polygons but is often exactly what we want.

In my collision engine, I've decided to implement the calculation of this penetration vector using the [EPA](http://www.dyn4j.org/2010/05/epa-expanding-polytope-algorithm/) method. This method again makes use of the same support functions we used to calculate whether we were colliding or not. The reason for this is that the penetration vector is the distance from the origin to the Minkowski difference.