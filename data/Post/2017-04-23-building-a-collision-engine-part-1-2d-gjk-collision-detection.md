---
title: "Building a Collision Engine Part 1: 2D GJK Collision Detection"
slug: building-a-collision-engine-part-1-2d-gjk-collision-detection
author: kenton
tags: [Math, Haxe]
published: 2017-04-23
meta-image: /assets/images/collision-engine-2d-detection/meta-preview.jpg
large-meta-image: true
preview-image: /assets/images/collision-engine-2d-detection/2d_yes_slow.gif
preview-summary: "I've previously written about using the Minkowski Difference to detect collisions of 2D AABBs, but I now want to expand this into creating a fully fleshed out and flexible collision engine for my own purposes. The engine will detect collisions using the GJK method, and calculate intersections using the EPA method. This post details how 2D GJK works, which will serve as a basis for getting the rest of the engine up and running."
---

I've previously written about [using the Minkowski Difference to detect collisions of 2D AABBs](http://hamaluik.com/posts/simple-aabb-collision-using-minkowski-difference/), but I now want to expand that into creating a fully fleshed out and _flexible_ collision engine for my own purposes (in [Haxe](http://haxe.org/) of course!). I recommend you read up on the [Minkowski difference](https://en.wikipedia.org/wiki/Minkowski_addition) and the overall technique of using to calculate the intersection of shapes before diving into things here as hopefully things will make much more sense then. Full credit also goes to [William Bittle](https://github.com/wnbittle) who created [dyn4j](http://www.dyn4j.org/) (which is a collision detection and physics engine written in Java) for his blog posts on [GJK](http://www.dyn4j.org/2010/04/gjk-gilbert-johnson-keerthi/) and [EPA](http://www.dyn4j.org/2010/05/epa-expanding-polytope-algorithm/), which this work is heavily based on.

Before I get any further, let me define what I want this collision engine to do:

1. Detect whether a collision occurred or not
2. Calculate a penetration vector if a collision did occur
3. Operate on any pair of convex objects in both 2D and 3D

The engine will perform the collision detection and penetration calculation steps completely separately, and at your discretion, meaning for every pair of objects you want to determine the collision of, you will have to call something to the effect of:

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

The GJK algorithm is a way of determining if two shapes are intersecting (meaning their Minkowski difference overlaps with the origin), without having to calculate the entire Minkowski difference like I did in my [previous posts](http://hamaluik.com/posts/simple-aabb-collision-using-minkowski-difference/). When you're just colliding AABBs with each other, the Minkowski difference is also an AABB and is very simple and quick to calculate, so you don't need this "shortcut", and can just calculate the entire thing and be on your way. When shapes start to rotate or have "weird" geometries however, this becomes less tenuous.

<figure>
    <img src="/assets/images/collision-engine-2d-detection/md_aabb_vs_polys.svg">
    <figcaption>The Minkowski difference of two AABBs is itself an AABB, while the Minkowski difference of a rectangle and a triangle is a generic polygon.</figcaption>
</figure>

GJK works by trying to find a triangle (or tetrahedron in 3D) which fits inside of the Minkowski difference and encapsulates the origin. If the triangle (or tetrahedron) contains the origin, and it also fits inside of the Minkowski difference, then the Minkowski difference must also contain the origin!

<figure>
    <img src="/assets/images/collision-engine-2d-detection/triangle-in-md.svg">
    <figcaption>If we can find a triangle which fits entirely within the Minkowski difference and also captures the origin, we can be confident the larger Minkowski difference captures the origin.</figcaption>
</figure>

The basis of finding a triangle inside of the Minkowski difference uses two concepts:

1. Support functions
2. Simplexes

### Support Functions

A support function for a convex shape is just a function that returns a point on the boundary of a shape that is the furthest in a given arbitrary direction. If multiple points are at the same distance, any of the points are acceptable. If you can fully define a support function for a shape, then you can use it to collide with things.

<figure>
    <img src="/assets/images/collision-engine-2d-detection/convex-shapes.svg">
    <figcaption>Any convex shape can be used in this algorithm, so long as you can define a support function for it.</figcaption>
</figure>

Note that due to the properties of Minkowski differences and support functions, the support function of a Minkowski difference of two shapes is equal to the difference of the support functions of two shapes. This is what allows us to not calculate an entire Minkowski difference, but rather just the difference in support functions for the two shapes! Basically, given the support functions we can easily call:

```haxe
public function getSupport(direction:Vec2):Vec2 {
    return shapeA.support(direction) - shapeB.support(-1 * direction);
}
```

<figure>
    <img src="/assets/images/collision-engine-2d-detection/md-supports.svg">
    <figcaption>The support function of a Minkowski difference of two shapes is equal to the difference in support functions for the two shapes.</figcaption>
</figure>

Here are some support functions for two common shapes:

#### Circle

```haxe
class Circle implements Shape2D {
    public var centre:Vec2;
    public var radius:Float;

    public function new(centre:Vec2, radius:Float) {
        this.centre = centre;
        this.radius = radius;
    }

    public function support(direction:Vec2):Vec2 {
        return centre + radius * direction.normalized();
    }
}
```

#### Polygon

```haxe
class Polygon implements Shape2D {
    public var vertices:Array<Vec2>;

    public function new(vertices:Array<Vec2>) {
        this.vertices = vertices;
    }

    public function support(direction:Vec2):Vec2 {
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

A [simplex](https://en.wikipedia.org/wiki/Simplex) is a somewhat special shape in the dimension we're working in. For a given dimension \\(k\\), the simplex in that dimension is a shape with \\(k + 1\\) vertices. Or, in the real world: In 2D, a simplex is a **triangle** and in 3D, a simplex is a **tetrahedron**. The simplex represents the most basic solid shape that can exist in a dimension, which is helpful for calculating whether it covers the origin or not.

<figure>
    <img src="/assets/images/collision-engine-2d-detection/simplex.svg">
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
    <img src="/assets/images/collision-engine-2d-detection/simplex-v1.svg">
    <figcaption>The first vertex of the simplex is the support of the Minkowski difference in the direction of the shapes' separation.</figcaption>
</figure>

```haxe
public function evolveSimplex() {
    switch(vertices.length) {
        case 0: {
            direction = shapeB.centre() - shapeA.centre();
            vertices.push(shapeA.support(direction) - shapeB.support(-1 * direction));
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
    <img src="/assets/images/collision-engine-2d-detection/simplex-v2.svg">
    <figcaption>The second vertex of the simplex is the support of the Minkowski difference in the opposite direction as before.</figcaption>
</figure>

```haxe
public function evolveSimplex():Void {
    switch(vertices.length) {
        case 0: {
            direction = shapeB.centre() - shapeA.centre();
            vertices.push(shapeA.support(direction) - shapeB.support(-1 * direction));
        }
        case 1: {
            // flip the direction
            direction *= -1;
            vertices.push(shapeA.support(direction) - shapeB.support(-1 * direction));
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

The third vertex should be chosen as the support in the direction [perpendicular](https://en.wikipedia.org/wiki/Perpendicular) to the line formed by the first two vertices, in the direction of the origin, again maximizing the size of the simplex so that we can either complete or fail as early as possible.

<figure>
    <img src="/assets/images/collision-engine-2d-detection/simplex-v3.svg">
    <figcaption>The third vertex of the simplex is the support of the Minkowski difference in the direction of the origin parallel to the line formed by the first two vertices.</figcaption>
</figure>

```haxe
public function evolveSimplex():Void {
    switch(vertices.length) {
        case 0: {
            direction = shapeB.centre() - shapeA.centre();
            vertices.push(shapeA.support(direction) - shapeB.support(-1 * direction));
        }
        case 1: {
            // flip the direction
            direction *= -1;
            vertices.push(shapeA.support(direction) - shapeB.support(-1 * direction));
        }
        case 2: {
            var b:Vec2 = vertices[1];
            var c:Vec2 = vertices[0];
            
            // line cb is the line formed by the first two vertices
            var cb:Vec2 = b - c;
            // line c0 is the line from the first vertex to the origin
            var c0:Vec2 = c * -1;

            // use the triple-cross-product to calculate a direction perpendicular to line cb
            // in the direction of the origin
            direction = tripleProduct(cb, c0, cb);
            vertices.push(shapeA.support(direction) - shapeB.support(-1 * direction));
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

To determine whether our simplex triangle contains the origin or not, we actually test whether the origin is on the "inside" or "outside" of each line segment of the triangle. We have three line segments composing the triangle, so three tests have to be done. If the origin is on the "outside" of any line segment, then we know the triangle doesn't include the origin; on the other hand if all three tests say the origin is on the inside, we know we contain the origin.

<figure>
    <img src="/assets/images/collision-engine-2d-detection/inside-outside.svg">
    <figcaption>Testing if a point is contained in a triangle is done by testing if it's on the "inside" or "outside" of each line segment.</figcaption>
</figure>

We can actually streamline this process a little bit however, and only do two tests instead of the full three. Since we chose the third vertex of the simplex to be in the direction of the origin, we **know** that origin is definitely on the inside of the first line segment (formed by our first and second simplex vertices). All that's left to do is test the lines \\(v_2 \to v_1\\) and \\(v_2 \to v_0\\).

To check if a point is "inside" or "outside" the line segment, we can use the [triple cross product](https://en.wikipedia.org/wiki/Triple_product) to generate a perpendicular line which points away from the vertex of the triangle which **isn't** being used by this line segment. Then we test if that perpendicular line is in the same direction as the line from the new vertex to the origin using the [dot product](https://en.wikipedia.org/wiki/Dot_product). Recall that if the dot product of two vectors is \\(> 0\\) then we know that they are in the same direction, and in opposite directions if the dot product is \\(< 0\\).

```haxe
        case 3: {
            // calculate if the simplex contains the origin
            var a:Vec2 = vertices[2];
            var b:Vec2 = vertices[1];
            var c:Vec2 = vertices[0];

            var a0:Vec2 = a * -1; // v2 to the origin
            var ab:Vec2 = b - a; // v2 to v1
            var ac:Vec2 = c - a; // v2 to v0

            var abPerp:Vec2 = tripleProduct(ac, ab, ab);
            var acPerp:Vec2 = tripleProduct(ab, ac, ac);

            if(abPerp.dot(a0) > 0) {
                // the origin is outside line ab
                // TODO: evolve the simplex
            }
            else if(acPerp.dot(a0) > 0) {
                // the origin is outside line ac
                // TODO: evolve the simplex
            }
            else {
                // the origin is inside both ab and ac,
                // so it must be inside the triangle!
                containsOrigin = true;
            }
        }
```

If we found that we did indeed contain the origin, we're done! Otherwise, we have to evolve the simplex by removing one of the simplex vertices and adding a new one somewhere else.

### Evolving the Simplex

If we found that the origin lies on the outside of one of the line segments, we know that the third vertex not participating in the line segment is useless, so we should remove it. We can add the next support in the "outside" perpendicular direction we calculated earlier to continue our search.

<figure>
    <img src="/assets/images/collision-engine-2d-detection/outside-ab.svg">
    <figcaption>If the origin is on the "outside" of the `ab` line, vertex `c` should be removed so we can reform our simplex to hopefully overlap the origin.</figcaption>
</figure>

```haxe
        case 3: {
            // calculate if the simplex contains the origin
            var a:Vec2 = vertices[2];
            var b:Vec2 = vertices[1];
            var c:Vec2 = vertices[0];

            var a0:Vec2 = a * -1; // v2 to the origin
            var ab:Vec2 = b - a; // v2 to v1
            var ac:Vec2 = c - a; // v2 to v0

            var abPerp:Vec2 = tripleProduct(ac, ab, ab);
            var acPerp:Vec2 = tripleProduct(ab, ac, ac);

            if(abPerp.dot(a0) > 0) {
                // the origin is outside line ab
                // get rid of c and add a new support in the direction of abPerp
                vertices.remove(c);
                direction = abPerp;
                vertices.push(shapeA.support(direction) - shapeB.support(-1 * direction));
            }
            else if(acPerp.dot(a0) > 0) {
                // the origin is outside line ac
                // get rid of b and add a new support in the direction of acPerp
                vertices.remove(b);
                direction = acPerp;
                vertices.push(shapeA.support(direction) - shapeB.support(-1 * direction));
            }
            else {
                // the origin is inside both ab and ac,
                // so it must be inside the triangle!
                containsOrigin = true;
            }
        }
```

Once we've reformed the simplex this way, it's time to check it again. If our simplex covers the origin, great&mdash;we're done! Otherwise, evolve the simplex again and try again. And again. And again. And&hellip; wait. How do we exit if we **aren't** colliding, meaning our simplex will _never_ contain the origin?

### Determining if the Shapes _Aren't_ Colliding

From geometry we know that if two shapes _aren't_ colliding, the Minkowski difference _will not_ contain the origin. Up until this point, we've assumed that the shape we're testing _does_ contain the origin&mdash;we need to change that. Thankfully, this is a relatively easy change.

Whenever we add a new vertex to the simplex, we can test whether the vertex **went past** the origin or not. If it did, we're ok and can continue processing. If it **didn't**, we know we won't be intersecting at all, and can exit early. How do we know this? Because we're using support functions. When we add a vertex in a given direction using the support functions, we **know** that that vertex is the vertex which lies the furthest in our direction. If that vertex _doesn't_ go past the origin, we will _never_ get a simplex which goes past the origin, meaning it will _never_ contain it.

We can calculate whether the new vertex passes the origin by determining if `direction` and the line `a0` point in the same direction, meaning their dot product is \\(> 0\\).

<figure>
    <img src="/assets/images/collision-engine-2d-detection/past-origin.svg">
    <figcaption>If the new vertex <em>doesn't</em> pass the origin, no simplex we can create will contain the origin, meaning we <b>are not</b> intersecting.</figcaption>
</figure>

To add this into our code, I'm going to modify things a little bit. Instead of blindly having a function `evolveSimplex` which never returns anything useful outside of itself, I will make it return some useful information&mdash;either we can't possibly intersect, we need to keep evolving the simplex, or we found an intersection. We can also reduce a bit of the repetition by partitioning the new vertex addition into its own function which does the check for us.

```haxe
enum EvolveResult {
    NoIntersection;
    FoundIntersection;
    StillEvolving;
}

// ...

private function addSupport(direction:Vec2):Bool {
    var newVertex:Vec2 = shapeA.support(direction) - shapeB.support(-1 * direction);
    vertices.push(newVertex);
    return Vec2.dot(direction, newVertex) >= 0;
}

public function evolveSimplex():EvolveResult {
    switch(vertices.length) {
        case 0: {
            direction = shapeB.centre() - shapeA.centre();
        }
        case 1: {
            // flip the direction
            direction *= -1;
        }
        case 2: {
            var b:Vec2 = vertices[1];
            var c:Vec2 = vertices[0];
            
            // line cb is the line formed by the first two vertices
            var cb:Vec2 = b - c;
            // line c0 is the line from the first vertex to the origin
            var c0:Vec2 = c * -1;

            // use the triple-cross-product to calculate a direction perpendicular to line cb
            // in the direction of the origin
            direction = tripleProduct(cb, c0, cb);
        }
        case 3: {
            // calculate if the simplex contains the origin
            var a:Vec2 = vertices[2];
            var b:Vec2 = vertices[1];
            var c:Vec2 = vertices[0];

            var a0:Vec2 = a * -1; // v2 to the origin
            var ab:Vec2 = b - a; // v2 to v1
            var ac:Vec2 = c - a; // v2 to v0

            var abPerp:Vec2 = tripleProduct(ac, ab, ab);
            var acPerp:Vec2 = tripleProduct(ab, ac, ac);

            if(abPerp.dot(a0) > 0) {
                // the origin is outside line ab
                // get rid of c and add a new support in the direction of abPerp
                vertices.remove(c);
                direction = abPerp;
            }
            else if(acPerp.dot(a0) > 0) {
                // the origin is outside line ac
                // get rid of b and add a new support in the direction of acPerp
                vertices.remove(b);
                direction = acPerp;
            }
            else {
                // the origin is inside both ab and ac,
                // so it must be inside the triangle!
                return EvolveResult.FoundIntersection;
            }
        }
        case _: throw 'Can\'t have simplex with ${vertices.length} verts!';
    }

    return addSupport(direction) ? EvolveResult.StillEvolving : EvolveResult.NoIntersection;
}
```

## Putting it All Together

What we have so far is basically one function for checking the simplex, which needs to be called a variable number of times and can return various things. What we want is a simple function to check if two shapes are colliding. We can do that with a simple utility function, which is the only the thing the end user really needs to interact with. Here's the whole kit and kaboodle so you can see how it all fits together:

```haxe
import collision.twod.GJK2D;
import collision.twod.Circle2D;
import collision.twod.Polygon2D;

class Main {
    public static function main() {
        var a:Circle = new Circle2D(new Vec2(0, 0), 0.5);
        var b:Polygon2D = new Polygon2D([
            new Vec2(0.25, 0.25), new Vec2(5, 5)
        ]);
        var c:Circle = new Circle2D(new Vec2(5, 10), 1);

        var gjk:GJK2D = new GJK2D();
        trace(gjk.test(a, b)); // true
        trace(jgk.test(a, c)); // false
    }
}
```

#### Shape2D.hx

```haxe
package collision.twod;

import glm.Vec2;

interface Shape2D {
    public function centre():Vec2;
    public function support(direction:Vec2):Vec2;
}
```

#### GJK2D.hx

```haxe
package collision.twod;

using glm.Vec2;
import glm.Vec3;
import collision.twod.Shape2D;

enum EvolveResult {
    NoIntersection;
    FoundIntersection;
    StillEvolving;
}

class GJK2D {
    private var vertices:Array<Vec2>;
    private var direction:Vec2;
    private var shapeA:Shape2D;
    private var shapeB:Shape2D;

    public function new() {}

    private function addSupport(direction:Vec2):Bool {
        var newVertex:Vec2 = shapeA.support(direction) - shapeB.support(-1 * direction);
        vertices.push(newVertex);
        return Vec2.dot(direction, newVertex) > 0;
    }

    function tripleProduct(a:Vec2, b:Vec2, c:Vec2):Vec2 {
        var A:Vec3 = new Vec3(a.x, a.y, 0);
        var B:Vec3 = new Vec3(b.x, b.y, 0);
        var C:Vec3 = new Vec3(c.x, c.y, 0);

        var first:Vec3 = Vec3.cross(A, B, new Vec3());
        var second:Vec3 = Vec3.cross(first, C, new Vec3());

        return new Vec2(second.x, second.y);
    }

    private function evolveSimplex():EvolveResult {
        switch(vertices.length) {
            case 0: {
                direction = shapeB.centre() - shapeA.centre();
            }
            case 1: {
                // flip the direction
                direction *= -1;
            }
            case 2: {
                var b:Vec2 = vertices[1];
                var c:Vec2 = vertices[0];
                
                // line cb is the line formed by the first two vertices
                var cb:Vec2 = b - c;
                // line c0 is the line from the first vertex to the origin
                var c0:Vec2 = c * -1;

                // use the triple-cross-product to calculate a direction perpendicular
                // to line cb in the direction of the origin
                direction = tripleProduct(cb, c0, cb);
            }
            case 3: {
                // calculate if the simplex contains the origin
                var a:Vec2 = vertices[2];
                var b:Vec2 = vertices[1];
                var c:Vec2 = vertices[0];

                var a0:Vec2 = a * -1; // v2 to the origin
                var ab:Vec2 = b - a; // v2 to v1
                var ac:Vec2 = c - a; // v2 to v0

                var abPerp:Vec2 = tripleProduct(ac, ab, ab);
                var acPerp:Vec2 = tripleProduct(ab, ac, ac);

                if(abPerp.dot(a0) > 0) {
                    // the origin is outside line ab
                    // get rid of c and add a new support in the direction of abPerp
                    vertices.remove(c);
                    direction = abPerp;
                }
                else if(acPerp.dot(a0) > 0) {
                    // the origin is outside line ac
                    // get rid of b and add a new support in the direction of acPerp
                    vertices.remove(b);
                    direction = acPerp;
                }
                else {
                    // the origin is inside both ab and ac,
                    // so it must be inside the triangle!
                    return EvolveResult.FoundIntersection;
                }
            }
            case _: throw 'Can\'t have simplex with ${vertices.length} verts!';
        }

        return addSupport(direction)
            ? EvolveResult.StillEvolving
            : EvolveResult.NoIntersection;
    }

    public function test(shapeA:Shape2D, shapeB:Shape2D):Bool {
        // reset everything
        this.vertices = new Array<Vec2>();
        this.shapeA = shapeA;
        this.shapeB = shapeB;

        // do the actual test
        var result:EvolveResult = EvolveResult.StillEvolving;
        while(result == EvolveResult.StillEvolving) {
            result = evolveSimplex();
        }
        return result == EvolveResult.FoundIntersection;
    }
}
```

#### Circle2D.hx

```haxe
package collision.twod;

import glm.Vec2;
import collision.twod.Shape2D;

class Circle2D implements Shape2D {
    public var centre:Vec2;
    public var radius:Float;

    public function new(centre:Vec2, radius:Float) {
        this.centre = centre;
        this.radius = radius;
    }

    public function support(direction:Vec2):Vec2 {
        return centre + radius * direction.normalized();
    }
}
```

#### Polygon2D.hx

```haxe
package collision.twod;

import glm.Vec2;
import collision.twod.Shape2D;

class Polygon2D implements Shape2D {
    public var vertices:Array<Vec2>;

    public function new(vertices:Array<Vec2>) {
        this.vertices = vertices;
    }

    public function support(direction:Vec2):Vec2 {
        if(this.vertices == null || this.vertices.length < 1)
            throw 'Can\'t have a polygon with 0 vertices!';

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

In my [next post](/posts/building-a-collision-engine-part-2-2d-penetration-vectors/), I will talk about how we can extend this to calculate the penetration vector of two intersecting shapes which is crucial in collision resolution.

## Demo

<figure>
    <iframe style="width: 100%; height: 200px; border: 0;" src="/assets/images/collision-engine-2d-detection/demo.html"></iframe>
</figure>

## Headbutt

I've started rolling this code into it's own library, tentatively called _Headbutt_, which you can follow along with if you're interested on Github: [https://github.com/FuzzyWuzzie/headbutt](https://github.com/FuzzyWuzzie/headbutt).
