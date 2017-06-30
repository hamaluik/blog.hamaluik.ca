---
title: "Building a Collision Engine Part 3: 3D GJK Collision Detection"
slug: building-a-collision-engine-part-3-3d-gjk-collision-detection
author: kenton
tags: [Math, Haxe]
published: 2017-06-30
meta-image: /assets/images/collision-engine-3d-detection/meta-preview.jpg
large-meta-image: true
preview-image: /assets/images/collision-engine-3d-detection/meta-preview.jpg
preview-summary: "Now that I've conquered 2D collision detection and intersection calculation, it's time to move onto 3D. Thankfully, GJK is relatively easy to extend into 3D once we have the base framework that we developed before."
---

Now that I've conquered 2D collision detection and intersection calculation, it's time to move onto 3D. Thankfully, GJK is relatively easy to extend into 3D once we have the base framework that we developed before. There are three main changes / additions we have to make to the original collision detection code to get it to work for 3D:

1. Change all the 2D data-types into 3D ones (`Vec2 -> Vec3`)
2. Change the case when the simplex has 3 vertices to add a new vertex instead of checking for collision
3. Add a case when the simplex has 4 vertices to check for collision & evolve the simplex

The first bit is fairly obvious, so I won't go into it here. For the rest, it's important to note that the overall collision detection process is the same as before, we just calculate the normal of a triangle instead of the normal of a line.

For the rest of this post, I'll refer to the following tetrahedral simplex and corresponding points `A`, `B`, `C`, `D` which comprise the simplex:

<figure>
    <img src="/assets/images/collision-engine-3d-detection/3d-simplex.svg">
    <figcaption>The simplex we'll be testing. It is made of 4 points and 4 corresponding planes.</figcaption>
</figure>

This simplex defines 4 planes, each defined by 3 of the points of the simplex: `ABC`, `ABD`, `BCD`, `CAD`.

## Building a 3D Simplex

Whereas in 2D we had a 2D simplex (a triangle), in 3D we have a 3D simplex (a tetrahedron). This means that when we have a simplex with 3 vertices, we're not done building the simplex yet (we have a triangle where we need a tetrahedron). This is easy to rectify—we can simply add another vertex in the support direction of the normal of the triangle in the direction of the origin. That's a bit of a mouthful, so let's unpack it.

We need the normal of the triangle. This can be calculated using the cross product of two edges of the triangle:

```haxe
var ac:Vec3 = vertices[2] - vertices[0];
var ab:Vec3 = vertices[1] - vertices[0];
direction = ac.cross(ab, new Vec3());
```

The normal should be in the direction of the origin. The direction of the normal we calculated above depends on which edges we used and how they were oriented. We can ensure that the normal we calculated above points in the direction of the origin by checking it's dot product with a line to the origin and flipping it if the result is negative, as so:

```haxe
if(direction.dot(a0) < 0) direction *= -1;
```

And that's all we need for evolving our simplex from a triangle to a tetrahedron! The new code for evolving the simplex looks like this:

```haxe
switch(vertices.length) {
    case 0: {
        direction = shapeB.centre - shapeA.centre;
    }
    case 1: {
        // flip the direction
        direction *= -1;
    }
    case 2: {
        // line cb is the line formed by the first two vertices
        var ab:Vec3 = vertices[1] - vertices[0];
        // line a0 is the line from the first vertex to the origin
        var a0:Vec3 = vertices[0] * -1;

        // use the triple-cross-product to calculate a direction perpendicular
        // to line ab in the direction of the origin
        var tmp:Vec3 = ab.cross(a0, new Vec3());
        direction = tmp.cross(ab, direction);
    }
    case 3: {
        var ac:Vec3 = vertices[2] - vertices[0];
        var ab:Vec3 = vertices[1] - vertices[0];
        direction = ac.cross(ab, new Vec3());

        // ensure it points toward the origin
        var a0:Vec3 = vertices[0] * -1;
        if(direction.dot(a0) < 0) direction *= -1;
    }
    case 4: {
        // TODO
    }
    case _: throw 'Can\'t have simplex with ${vertices.length} verts!';
}
```

## Checking for a Collision in 3D

The process for checking for a collision here is basically identical to the [2D version](/posts/building-a-collision-engine-part-1-2d-gjk-collision-detection/#determining-if-the-simplex-contains-the-origin), with the exception that we have to check plane (triangle) normals instead of edge normals, and we have to check 3 things instead of two (as we have one more dimension). Similar to before, we can ignore the base plane as if we've made it this far, we've ensure that the origin is on the simplex side of the plane `ABC`.

<figure>
    <img src="/assets/images/collision-engine-3d-detection/inside-outside-triangle.svg">
    <figcaption>We need to check whether the origin is 'inside' or 'outside' plane <code>ABD</code> (then again for <code>BCD</code> and <code>CAD</code>).</figcaption>
</figure>


Just like before, we check whether the origin is 'inside' or 'outside' the plane by comparing the direction of the plane's normal with the direction to the origin. If they're in the same direction, the origin is outside the plane and we need to evolve the simplex. Otherwise, the origin is inside the plane and we can test the other planes (or conclude the origin _must_ be inside the simplex).

<figure>
    <img src="/assets/images/collision-engine-3d-detection/triangle-normal.svg">
    <figcaption>Calculating the normal of a triangle using the cross product of two of it's edges using the right hand rule.</figcaption>
</figure>

Let's get started by declaring the edges we're interested in (`d` being the apex of the simplex and the most recent vertex that was added):

```haxe
// calculate the three edges of interest
var da = vertices[3] - vertices[0];
var db = vertices[3] - vertices[1];
var dc = vertices[3] - vertices[2];

// and the direction to the origin
var d0 = vertices[3] * -1;
```

Now we calculate the normals of the three triangle planes, ensuring that the normal is in the "outside" direction (using the [right hand rule](https://en.wikipedia.org/wiki/Right-hand_rule):

```haxe
// check triangles a-b-d, b-c-d, and c-a-d
var abdNorm:Vec3 = da.cross(db, new Vec3());
var bcdNorm:Vec3 = db.cross(dc, new Vec3());
var cadNorm:Vec3 = dc.cross(da, new Vec3());
```

And finally, we can check them against the direction to the origin:

```haxe
if(abdNorm.dot(d0) > 0) {
    // the origin is on the outside of triangle a-b-d
}
else if(bcdNorm.dot(d0) > 0) {
    // the origin is on the outside of triangle bcd
}
else if(cadNorm.dot(d0) > 0) {
    // the origin is on the outside of triangle cad
}
else {
    // the origin is inside all of the triangles!
    return EvolveResult.FoundIntersection;
}
```

If the origin is found to be on the 'outside' of any of the planes, we know that we can eliminate the vertex of the simplex that is not on that plane, and add a new vertex in the support direction of the plane's normal:

```haxe
// the origin is on the outside of triangle a-b-d
// eliminate c!
vertices.remove(vertices[2]);
direction = abdNorm;
```

So that altogether, this step looks like:

```haxe
case 4: {
    // calculate the three edges of interest
    var da = vertices[3] - vertices[0];
    var db = vertices[3] - vertices[1];
    var dc = vertices[3] - vertices[2];

    // and the direction to the origin
    var d0 = vertices[3] * -1;

    // check triangles a-b-d, b-c-d, and c-a-d
    var abdNorm:Vec3 = da.cross(db, new Vec3());
    var bcdNorm:Vec3 = db.cross(dc, new Vec3());
    var cadNorm:Vec3 = dc.cross(da, new Vec3());

    if(abdNorm.dot(d0) > 0) {
        // the origin is on the outside of triangle a-b-d
        // eliminate c!
        vertices.remove(vertices[2]);
        direction = abdNorm;
    }
    else if(bcdNorm.dot(d0) > 0) {
        // the origin is on the outside of triangle bcd
        // eliminate a!
        vertices.remove(vertices[0]);
        direction = bcdNorm;
    }
    else if(cadNorm.dot(d0) > 0) {
        // the origin is on the outside of triangle cad
        // eliminate b!
        vertices.remove(vertices[1]);
        direction = cadNorm;
    }
    else {
        // the origin is inside all of the triangles!
        return EvolveResult.FoundIntersection;
    }
}
```

## Putting it All Together

3D collision detection with GJk is definitely a lot easier than I originally thought it was, especially after you get all the concepts figured out in 2D. For reference, here is the complete 3D collision detection class at the time of writing from my [Headbutt](https://github.com/FuzzyWuzzie/headbutt) library:

### Headbutt3D.hx

```haxe
package headbutt;

using glm.Vec3;

class Headbutt3D {
    private var vertices:Array<Vec3>;
    private var direction:Vec3;
    private var shapeA:Shape3D;
    private var shapeB:Shape3D;

    public function new() {}

    private function calculateSupport(direction:Vec3):Vec3 {
        var oppositeDirection:Vec3 = direction.multiplyScalar(-1, new Vec3());
        var newVertex:Vec3 = shapeA.support(direction).copy(new Vec3());
        newVertex.subtractVec(shapeB.support(oppositeDirection), newVertex);
        return newVertex;
    }

    private function addSupport(direction:Vec3):Bool {
        var newVertex:Vec3 = calculateSupport(direction);
        vertices.push(newVertex);
        return Vec3.dot(direction, newVertex) >= 0;
    }

    private function evolveSimplex():EvolveResult {
        switch(vertices.length) {
            case 0: {
                direction = shapeB.centre - shapeA.centre;
            }
            case 1: {
                // flip the direction
                direction *= -1;
            }
            case 2: {
                // line ab is the line formed by the first two vertices
                var ab:Vec3 = vertices[1] - vertices[0];
                // line a0 is the line from the first vertex to the origin
                var a0:Vec3 = vertices[0] * -1;

                // use the triple-cross-product to calculate a direction perpendicular
                // to line ab in the direction of the origin
                var tmp:Vec3 = ab.cross(a0, new Vec3());
                direction = tmp.cross(ab, direction);
            }
            case 3: {
                var ac:Vec3 = vertices[2] - vertices[0];
                var ab:Vec3 = vertices[1] - vertices[0];
                direction = ac.cross(ab, new Vec3());

                // ensure it points toward the origin
                var a0:Vec3 = vertices[0] * -1;
                if(direction.dot(a0) < 0) direction *= -1;
            }
            case 4: {
                // ascii representation of our simplex at this point
                /*
                                           [D]
                                          ,|,
                                        ,7``\'VA,
                                      ,7`   |, `'VA,
                                    ,7`     `\    `'VA,
                                  ,7`        |,      `'VA,
                                ,7`          `\         `'VA,
                              ,7`             |,           `'VA,
                            ,7`               `\       ,..ooOOTK` [C]
                          ,7`                  |,.ooOOT''`    AV
                        ,7`            ,..ooOOT`\`           /7
                      ,7`      ,..ooOOT''`      |,          AV
                     ,T,..ooOOT''`              `\         /7
                [A] `'TTs.,                      |,       AV
                         `'TTs.,                 `\      /7
                              `'TTs.,             |,    AV
                                   `'TTs.,        `\   /7
                                        `'TTs.,    |, AV
                                             `'TTs.,\/7
                                                  `'T`
                                                    [B]
                */

                // calculate the three edges of interest
                var da = vertices[3] - vertices[0];
                var db = vertices[3] - vertices[1];
                var dc = vertices[3] - vertices[2];

                // and the direction to the origin
                var d0 = vertices[3] * -1;

                // check triangles a-b-d, b-c-d, and c-a-d
                var abdNorm:Vec3 = da.cross(db, new Vec3());
                var bcdNorm:Vec3 = db.cross(dc, new Vec3());
                var cadNorm:Vec3 = dc.cross(da, new Vec3());

                if(abdNorm.dot(d0) > 0) {
                    // the origin is on the outside of triangle a-b-d
                    // eliminate c!
                    vertices.remove(vertices[2]);
                    direction = abdNorm;
                }
                else if(bcdNorm.dot(d0) > 0) {
                    // the origin is on the outside of triangle bcd
                    // eliminate a!
                    vertices.remove(vertices[0]);
                    direction = bcdNorm;
                }
                else if(cadNorm.dot(d0) > 0) {
                    // the origin is on the outside of triangle cad
                    // eliminate b!
                    vertices.remove(vertices[1]);
                    direction = cadNorm;
                }
                else {
                    // the origin is inside all of the triangles!
                    return EvolveResult.FoundIntersection;
                }
            }
            case _: throw 'Can\'t have simplex with ${vertices.length} verts!';
        }

        return addSupport(direction)
            ? EvolveResult.StillEvolving
            : EvolveResult.NoIntersection;
    }

    public function test(shapeA:Shape3D, shapeB:Shape3D):Bool {
        // reset everything
        this.vertices = new Array<Vec3>();
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

## Demo

<figure>
    <iframe style="width: 100%; height: 200px; border: 0;" src="/assets/images/collision-engine-3d-detection/demo.html"></iframe>
</figure>

## Headbutt

I've started rolling this code into it's own library, tentatively called _Headbutt_, which you can follow along with if you're interested on Github: [https://github.com/FuzzyWuzzie/headbutt](https://github.com/FuzzyWuzzie/headbutt).