---
title: "Building a Collision Engine Part 1: 2D Collision Detection"
slug: building-a-collision-engine-part-1-2d-collision-detection
author: kenton
tags: [Math]
published: 2017-04-19
preview-summary: ""
---

I've previously written about [using the Minkowski Difference to detect collisions of 2D AABBs](http://hamaluik.com/posts/simple-aabb-collision-using-minkowski-difference/), but I now want to expand this into creating a fully fleshed out and _flexible_ collision engine for my own purposes. I recommend you read up on the Minkowski difference and the overall technique of using to calculate the intersection of shapes before diving into things here as hopefully things will make much more sense then.

Before I get any further, let me define what I want this collision engine to do:

1. Detect whether a collision occurred or not
2. Calculate a penetration vector if a collision did occur
3. Operate on any pair of convex objects in both 2D and 3D

The engine will perform the two above steps completely separately, and at your discretion, meaning for every pair of objects you want to determine the collision of, you will have to call something to the effect of:

```haxe
var shapeA:Array<Vec2> = [
    new Vec2(-18, -18), new Vec2(-10, -18),
    new Vec2(-10, -13), new Vec2(-18, -13)
];
var shapeB:Array<Vec2> = [
    new Vec2(-14, -14), new Vec2(-5, -16), new Vec2(-12, -8)
];

var colliding:Bool = checkCollision2D(shapeA, shapeB);
trace('Colliding: ${colliding}');

var collision:Collision = calculateCollision2D(shapeA, shapeB);
trace('Colliding: ${collision.colliding}');
trace('Collision normal: ${collision.normal}');
trace('Collision point: ${collision.point}');
```

This engine _will not_ be responsible for:

1. Performing broad-phase collision culling (this is likely to be more engine / environment dependent, though I may one day add it)
2. Applying any physics simulation or collision response (again, this is engine / environment dependent)

Since I want this to be as flexible as possible (while being limited to convex shapes), I'll be developing the collision engine using [GJK (Gilbert-Johnson-Keerthi)](https://en.wikipedia.org/wiki/Gilbert%E2%80%93Johnson%E2%80%93Keerthi_distance_algorithm) for collision detection and [EPA](http://www.dyn4j.org/2010/05/epa-expanding-polytope-algorithm/) for intersection calculation.

With that out of the way, let's get started with the simplest bit first: 2D collision detection using GJK.

The GJK algorithm is a way of determining if two shapes are intersection (meaning their Minkowski difference overlaps with the origin), without having to calculate the entire Minkowski difference like I did in my [previous posts](http://hamaluik.com/posts/simple-aabb-collision-using-minkowski-difference/). When you're just colliding AABBs with each other, the Minkowski difference is also an AABB and is very simple and quick to calculate, so you don't need this "shortcut", and can just calculate the entire thing and be on your way. When shapes start to rotate or have "weird" geometries however, this becomes less tenuous.

<figure>
    <img src="/assets/images/collision-engine-2d-detection/md_aabb_vs_polys.jpg">
    <figcaption>The Minkowski difference of two AABBs is itself an AABB, while the Minkowski difference of a rectangle and a triangle is a generic polygon.</figcaption>
</figure>

GJK works by trying to find a triangle (or tetrahedron in 3D) which fits inside of the Minkowski difference and encapsulates the origin. If the triangle (or tetrahedron) contains the origin, and it also fits inside of the Minkowski difference, then the Minkowski difference must also contain the origin!

<figure>
    <img src="/assets/images/collision-engine-2d-detection/triangle-in-md.jpg">
    <figcaption>If we can find a triangle which fits entirely within the Minkowski difference and also captures the origin, we can be confident the larger Minkowski difference captures the origin.</figcaption>
</figure>

The basis of finding a triangle inside of the Minkowski difference uses two concepts:

1. Support functions
2. Simplexes

### Support Functions

A support function for a convex shape is just a function that returns a point on the boundary of a shape that is the furthest in a given arbitrary direction. If multiple points are at the same distance, any of the points are acceptable. If you can fully define a support function for a shape, then you can use it to collide with things.

<figure>
    <img src="/assets/images/collision-engine-2d-detection/convex-shapes.jpg">
    <figcaption>Any convex shape can be used in this algorithm, so long as you can define a support function for it.</figcaption>
</figure>

Note that due to the properties of Minkowski differences and support functions, the support function of a Minkowski difference of two shapes is equal to the difference of the support functions of two shapes. This is what allows us to not calculate an entire Minkowski difference, but rather just the difference in support functions for the two shapes!

<figure>
    <img src="/assets/images/collision-engine-2d-detection/md-supports.jpg">
    <figcaption>The support function of a Minkowski difference of two shapes is equal to the difference in support functions for the two shapes.</figcaption>
</figure>

Here are some support functions for two common shapes:

#### Circle

<figure>
    <img src="/assets/images/collision-engine-2d-detection/circle-support.jpg">
    <figcaption>The support function of a circle is easily defined using its centre and radius.</figcaption>
</figure>

```haxe
class Circle : Shape2D {
    public var centre:Vec2;
    public var radius:Float;

    public function new(centre:Vec2, radius:Float) {
        this.centre = centre;
        this.radius = radius;
    }

    override public function support(direction:Vec2):Vec2 {
        return centre + radius * direction.normalized();
    }
}
```

#### Polygon

<figure>
    <img src="/assets/images/collision-engine-2d-detection/polygon-support.jpg">
    <figcaption>The support function of a polygon is the vertex furthest in a given direction.</figcaption>
</figure>

```haxe
class Polygon : Shape2D {
    public var vertices:Array<Vec2>;

    public function new(vertices:Array<Vec2>) {
        this.vertices = vertices;
    }

    override public function support(direction:Vec2):Vec2 {
        var furthestDistance:Float = Math.NEGATIVE_INFINITY;
        var furthestVertex:Vec2 = null;

        for(v in vertices) {
            var distance:Float = Vec2.dot(v, direction);
            if(distance > furthestDistance) {
                furthestDistance = distance;
                furthestVertex = v;
            }
        }

        return furthestVertex;
    }
}
```

### Simplexes

A [simplex](https://en.wikipedia.org/wiki/Simplex) is somewhat special a shape in the dimension we're working in. For a given dimension `k`, the simplex in that dimension is a shape with `k + 1` vertices. Or, in the real world: In 2D, a simplex is a **triangle** and in 3D, a simplex is a **tetrahedron**. The simplex represents the most basic solid shape that can exist in a dimension, which is helpful for calculating whether it covers the origin or not.

<figure>
    <img src="/assets/images/collision-engine-2d-detection/simplex.jpg">
    <figcaption>In 2D, the simplex is a triangle and in 3D, the simplex is a tetrahedron.</figcaption>
</figure>

## The GJK Algorithm

The GJK algorithm starts by calculating a triangle that fits within the Minkowski difference. If that first triangle contains the origin, then congrats! You've determined that the shapes are intersecting! Otherwise, see if you can make a new triangle which does contain the origin. Keep going until you've either created a triangle which contains the origin or are confident that there is not way you can create a triangle which contains the origin. This process is called "evolving the simplex", because you keep upgrading the simplex until you get what you want or find if its not possible.

<figure>
    <img src="/assets/images/collision-engine-2d-detection/2d_yes_slow.gif">
    <figcaption>Evolving the simplex until it contains the origin.</figcaption>
</figure>

### Calculating the First Simplex / Triangle

In order to have a triangle, you must have 3 points / vertices. Note that since we're using support functions, a good pick for the first vertex is the support in the direction of separation of the two shapes (though we could pick any arbitrary direction, picking this one tends to make the simplex converge on a result much faster).

<figure>
    <figcaption>The first vertex of the simplex is the support of the Minkowski difference in the direction of the shapes' separation.</figcaption>
</figure>

```haxe
public function evolveSimplex() {
    switch(vertices.length) {
        case 0: {
            direction = (shapeB.centre() - shapeA.centre()).normalized();
            vertices.push(shapeB.support(direction) - shapeA.support(direction));
        }
        case 1: {
            // TODO: add the second vertex
        }
        case 2: {
            // TODO: add the third vertex
        }
        case 3: {
            // TODO: calculate if the simplex contains the origin
            // if it does, we're done!
            // if it doesn't, update the simplex!
        }
        case _: throw 'Can\'t have simplex with ${vertices.length} verts!';
    }
}
```

The second vertex can be chosen as the support in the opposite direction of what we chose in the first place (so as to maximize the size of our simplex).

<figure>
    <figcaption>The second vertex of the simplex is the support of the Minkowski difference in the opposite direction as before.</figcaption>
</figure>

```haxe
public function evolveSimplex():Void {
    switch(vertices.length) {
        case 0: {
            direction = (shapeB.centre() - shapeA.centre()).normalized();
            vertices.push(shapeB.support(direction) - shapeA.support(direction));
        }
        case 1: {
            // flip the direction
            direction *= -1;
            vertices.push(shapeB.support(direction) - shapeA.support(direction));
        }
        case 2: {
            // TODO: add the third vertex
        }
        case 3: {
            // TODO: calculate if the simplex contains the origin
            // if it does, we're done!
            // if it doesn't, update the simplex!
        }
        case _: throw 'Can\'t have simplex with ${vertices.length} verts!';
    }
}
```

The third vertex should be chosen as the support in the direction [perpendicular](https://en.wikipedia.org/wiki/Perpendicular) to the line formed by the first two vertices, in the direction of the origin.

<figure>
    <figcaption>The third vertex of the simplex is the support of the Minkowski difference in the direction of the origin parallel to the line formed by the first two vertices.</figcaption>
</figure>

```haxe
public function evolveSimplex():Void {
    switch(vertices.length) {
        case 0: {
            direction = (shapeB.centre() - shapeA.centre()).normalized();
            vertices.push(shapeB.support(direction) - shapeA.support(direction));
        }
        case 1: {
            // flip the direction
            direction *= -1;
            vertices.push(shapeB.support(direction) - shapeA.support(direction));
        }
        case 2: {
            // line AB is the line formed by the first two vertices
            var AB:Vec2 = vertices[1] - vertices[0];
            // line A0 is the line from the first vertex to the origin
            var A0:Vec2 = vertices[0] * -1;

            // use the triple-cross-product to calculate a direction perpendicular to line AB
            // in the direction of the origin
            direction = tripleProduct(AB, A0, AB);
            vertices.push(shapeB.support(direction) - shapeA.support(direction));
        }
        case 3: {
            // TODO: calculate if the simplex contains the origin
            // if it does, we're done!
            // if it doesn't, update the simplex!
        }
        case _: throw 'Can\'t have simplex with ${vertices.length} verts!';
    }
}
```

### Determining if the Simplex Contains the Origin