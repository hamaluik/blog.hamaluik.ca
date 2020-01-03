---
title: "Building a Collision Engine Part 2: 2D Penetration Vectors"
slug: building-a-collision-engine-part-2-2d-penetration-vectors
author: kenton
tags: [Math, Haxe]
published: 2017-06-11T00:00:00-07:00
summary: "In my last post, I discussed how to determine if any two convex shapes in two dimensions overlap. The result of this calculation is a boolean state—either the two are colliding or they aren't. This is great for things such as triggers, but if you want to use the collision engine for any type of physics calculations, you'll need at least one more crucial piece of information: the penetration vector. This post discusses how to calculate the penetration vector using the EPA algorithm."
---

In my [last post](/posts/building-a-collision-engine-part-1-2d-gjk-collision-detection/), I discussed how to determine if any two convex shapes in two dimensions overlap. The result of this calculation is a boolean state—either the two are colliding or they aren't. This is great for things such as triggers, but if you want to use the collision engine for any type of physics calculations, you'll need at least one more crucial piece of information: the penetration vector.

The penetration vector of two shapes is a vector which describes the overlap of those two shapes. By adding the penetration vector to one of the shapes, the shapes can be separated so that they are just touching, but not really intersecting. Without any extra information, this vector chosen as the one that separate the two shapes along the shortest path possible, which can lead to some issues at the corners of polygons but is often exactly what we want.

In my collision engine, I've decided to implement the calculation of this penetration vector using the [EPA](http://www.dyn4j.org/2010/05/epa-expanding-polytope-algorithm/) method. This method again makes use of the same support functions we used to calculate whether we were colliding or not. The reason for this is that the penetration vector is the distance from the origin to the Minkowski difference, whether the Minkowski difference contains the origin or not.

<figure>
    <img src="/images/collision-engine-2d-penetration/penetration-vector-md-origin.svg">
    <figcaption>The penetration vector of two overlapping shapes is the distance of the Minkowski difference to the origin.</figcaption>
</figure>

### The EPA Method

The EPA (Expanding Polytope Algorithm) works well with Minkowski differences because it uses many of the same concepts—namely support functions and simplexes. However, instead of trying to use the support functions to construct a simplex which encases the origin, EPA seeks to use the support functions to find the boundaries of the Minkowski difference in an attempt to find the boundary that is closest to the origin. In order to do this, the algorithm starts with a simplex contained within the Minkowski difference (we might as well use the one from the previous step!), then "expands" it into higher vertex count polygons (gradually approaching the true Minkowski difference polygon) until one of the edges is found to be the closest to the origin.

<figure>
    <img src="/images/collision-engine-2d-penetration/search-closest.svg">
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
    <img src="/images/collision-engine-2d-penetration/search-further-supports.svg">
    <figcaption>If there is a support point sufficiently far away from our edge in it's normal direction, we haven't found the edge closest to the origin.</figcaption>
</figure>

If we _did_ find the edge closest to the origin, the support point in the direction of the edge's normal should lie on the edge itself. In this case, we can return the penetration vector as:

```haxe
intersection = edge.normal.copy(intersection);
intersection.multiplyScalar(distance, intersection);
```

On the other hand, if we found that we _didn't_ find the edge closest to the origin, we can **expand** the simplex toward the support point and try again:

```haxe
vertices.insert(edge.index, support);
```

<figure>
    <img src="/images/collision-engine-2d-penetration/search-expanded-simplex.svg">
    <figcaption>Expand the simplex to try again.</figcaption>
</figure>

Once we've expanded our simplex (for the first iteration the simplex will go from having 3 to having 4 vertices), we run the algorithm again, searching for the closest edge and determining if the support point along its normal lies on the edge or not. We can continue this process until we find the _de-facto_ closest edge or we hit some upper limit of iterations that we're willing to try. The code for calculating the intersection now looks like this:

```haxe
public function intersect(shapeA:Shape2D, shapeB:Shape2D):Null<Vec2> {
    var intersection:Vec2 = new Vec2();
    for(i in 0...32) {
        var edge:Edge = findClosestEdge(PolygonWinding.Clockwise);
        var support:Vec2 = calculateSupport(edge.normal);
        var distance:Float = support.dot(edge.normal);

        intersection = edge.normal.copy(intersection);
        intersection.multiplyScalar(distance, intersection);

        if(Math.abs(distance - edge.distance) <= EPSILON) {
            return intersection;
        }
        else {
            vertices.insert(edge.index, support);
        }
    }

    return intersection;
}
```

<figure>
    <img src="/images/collision-engine-2d-penetration/search-support-on-edge.svg">
    <figcaption>Search until we find a support vertex that lies on our closest edge.</figcaption>
</figure>

One last thing we can do is actually calculate the polygon winding of the simplex we're given so that we can pass it into our `findClosestEdge` function. This is relatively straightforward:

```haxe
// calculate the winding of the existing simplex
var e0:Float = (vertices[1].x - vertices[0].x) * (vertices[1].y + vertices[0].y);
var e1:Float = (vertices[2].x - vertices[1].x) * (vertices[2].y + vertices[1].y);
var e2:Float = (vertices[0].x - vertices[2].x) * (vertices[0].y + vertices[2].y);
var winding:PolygonWinding =
    if(e0 + e1 + e2 >= 0) PolygonWinding.Clockwise;
    else PolygonWinding.CounterClockwise;
```

## Putting it All Together

We can now extend the class from the [previous post](/posts/building-a-collision-engine-part-1-2d-gjk-collision-detection/) to include penetration vector calculation by adding the following. What you do with the penetration vector depends on your goal. If you're creating a physics system, you can use these vectors to resolve collision responses (pushing objects apart based on physical properties like mass and elasticity for example).

#### GJK2D.hx

```haxe
// ...

enum PolygonWinding {
    Clockwise;
    CounterClockwise;
}

class Edge {
    public var distance:Float;
    public var normal:Vec2;
    public var index:Int;

    public function new(distance:Float, normal:Vec2, index:Int) {
        this.distance = distance;
        this.normal = normal;
        this.index = index;
    }
}

class GJK2D {
    // ...

    private function findClosestEdge(winding:PolygonWinding):Edge {
        var closestDistance:Float = Math.POSITIVE_INFINITY;
        var closestNormal:Vec2 = new Vec2();
        var closestIndex:Int = 0;
        var line:Vec2 = new Vec2();
        for(i in 0...vertices.length) {
            var j:Int = i + 1;
            if(j >= vertices.length) j = 0;

            vertices[j].copy(line);
            line.subtractVec(vertices[i], line);

            var norm:Vec2 = switch(winding) {
                case PolygonWinding.Clockwise:
                    new Vec2(line.y, -line.x);
                case PolygonWinding.CounterClockwise:
                    new Vec2(-line.y, line.x);
            }
            norm.normalize(norm);

            // calculate how far away the edge is from the origin
            var dist:Float = norm.dot(vertices[i]);
            if(dist < closestDistance) {
                closestDistance = dist;
                closestNormal = norm;
                closestIndex = j;
            }
        }

        return new Edge(closestDistance, closestNormal, closestIndex);
    }

    public function intersect(shapeA:Shape2D, shapeB:Shape2D):Null<Vec2> {
        // first, calculate the base simplex
        if(!test(shapeA, shapeB)) {
            // if we're not intersecting, return null
            return null;
        }

        // calculate the winding of the existing simplex
        var e0:Float = (vertices[1].x - vertices[0].x) * (vertices[1].y + vertices[0].y);
        var e1:Float = (vertices[2].x - vertices[1].x) * (vertices[2].y + vertices[1].y);
        var e2:Float = (vertices[0].x - vertices[2].x) * (vertices[0].y + vertices[2].y);
        var winding:PolygonWinding =
            if(e0 + e1 + e2 >= 0) PolygonWinding.Clockwise;
            else PolygonWinding.CounterClockwise;

        var intersection:Vec2 = new Vec2();
        for(i in 0...32) {
            var edge:Edge = findClosestEdge(winding);
            var support:Vec2 = calculateSupport(edge.normal);
            var distance:Float = support.dot(edge.normal);

            intersection = edge.normal.copy(intersection);
            intersection.multiplyScalar(distance, intersection);

            if(Math.abs(distance - edge.distance) <= 0.000001) {
                return intersection;
            }
            else {
                vertices.insert(edge.index, support);
            }
        }

        return intersection;
    }
}
```

## Demo

<figure>
    <iframe style="width: 100%; height: 200px; border: 0;" src="/images/collision-engine-2d-penetration/demo.html"></iframe>
</figure>

## Headbutt

I've started rolling this code into it's own library, tentatively called _Headbutt_, which you can follow along with if you're interested on Github: [https://github.com/hamaluik/headbutt](https://github.com/hamaluik/headbutt).
