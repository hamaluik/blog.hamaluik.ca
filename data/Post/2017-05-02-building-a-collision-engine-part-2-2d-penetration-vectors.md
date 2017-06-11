---
title: "Building a Collision Engine Part 2: 2D Penetration Vectors"
slug: building-a-collision-engine-part-2-2d-penetration-vectors
author: kenton
tags: [Math, Haxe]
meta-image: https://unsplash.it/300/?random
large-meta-image: true
preview-image: https://unsplash.it/300/?random
preview-summary: ""
---

In my [last post](/posts/building-a-collision-engine-part-1-2d-gjk-collision-detection/), I discussed how to determine if any two convex shapes in two dimensions overlap. The result of this calculation is a boolean state&mdash;either the two are colliding or they aren't. This is great for things such as triggers, but if you want to use the collision engine for any type of physics calculations, you'll need at least one more crucial piece of information: the penetration vector.

The penetration vector of two shapes is a vector which describes the overlap of those two shapes. By adding the penetration vector to one of the shapes, the shapes can be separated so that they are just touching, but not really intersecting. Without any extra information, this vector chosen as the one that separate the two shapes along the shortest path possible, which can lead to some issues at the corners of polygons but is often exactly what we want.

In my collision engine, I've decided to implement the calculation of this penetration vector using the [EPA](http://www.dyn4j.org/2010/05/epa-expanding-polytope-algorithm/) method. This method again makes use of the same support functions we used to calculate whether we were colliding or not. The reason for this is that the penetration vector is the distance from the origin to the Minkowski difference, whether the Minkowski difference contains the origin or not.

<figure>
    <img src="https://unsplash.it/300/?random">
    <figcaption>The penetration vector of two overlapping shapes is the distance of the Minkowski difference to the origin.</figcaption>
</figure>

### The EPA Method

The EPA (Expanding Polytope Algorithm) works well with Minkowski differences because it uses many of the same concepts&mdash;namely support functions and simplexes. However, instead of trying to use the support functions to construct a simplex which encases the origin, EPA seeks to use the support functions to find the boundaries of the Minkowski difference in an attempt to find the boundary that is closest to the origin. In order to do this, the algorithm starts with a simplex contained within the Minkowski difference (we might as well use the one from the previous step!), then "expands" it into higher vertex count polygons (gradually approaching the true Minkowski difference polygon) until one of the edges is found to be the closest to the origin.

<figure>
    <img src="https://unsplash.it/300/?random">
    <figcaption>EPA "searches" for the edge of the polytope which is closest to the origin.</figcaption>
</figure>

We start by finding the closest edge in our simplex to the origin:

```haxe
private function findClosestEdge(winding:PolygonWinding):Edge {
    var closestDistance:Float = Math.POSITIVE_INFINITY;
    var closestNormal:Vec2 = new Vec2();
    var closestIndex:Int = 0;
    var edge:Vec2 = new Vec2();

    // loop through all the vertices (each edge will be vertex i -> vertex j)
    for(i in 0...vertices.length) {
        var j:Int = i + 1;
        if(j >= vertices.length) j = 0; // wrap

        // calculate the edge as v[j] - v[i]
        vertices[j].copy(edge);
        edge.subtractVec(vertices[i], edge);

        // quickly calculate the outward-facing normal of the edge
        // (make use of the polygon winding to do this in 2D easily)
        var norm:Vec2 = switch(winding) {
            case PolygonWinding.Clockwise:
                new Vec2(edge.y, -edge.x);
            case PolygonWinding.CounterClockwise:
                new Vec2(-edge.y, edge.x);
        }
        norm.normalize(norm);

        // calculate how far away the edge is from the origin
        var dist:Float = norm.dot(vertices[i]);
        // we're looking for the closest edge!
        if(dist < closestDistance) {
            closestDistance = dist;
            closestNormal = norm;
            closestIndex = j;
        }
    }

    return new Edge(closestDistance, closestNormal, closestIndex);
}
```

Once we have the edge in our polygon which is the closest to the origin, try to find a support point in the direction of the edge's normal direction to see if there might be something closer to the origin (that exists in the Minkowski difference, but not our simplex yet):

```haxe
var edge:Edge = findClosestEdge(simplexWinding);
var support:Vec2 = calculateSupport(edge.normal);
var distance:Float = support.dot(edge.normal);

if(Math.abs(distance - edge.distance) <= EPSILON) {
    // we found the edge in the Minkowski difference that is closest to the origin!
}
else {
    // there's likely an edge in the Minkowski difference that is closer to the origin..
}
```

<figure>
    <img src="https://unsplash.it/300/?random">
    <figcaption>If there is a support point sufficiently far away from our edge in it's normal direction, we haven't found the edge closest to the origin.</figcaption>
</figure>

If we _did_ find the edge closest to the origin