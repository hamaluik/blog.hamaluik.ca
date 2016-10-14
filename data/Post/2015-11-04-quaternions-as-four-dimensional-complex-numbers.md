---
title: Quaternions as Four-Dimensional Complex Numbers
slug: quaternions-as-four-dimensional-complex-numbers
summary: Although I have a pretty solid background in math (especially vectors, matrices, and even tensors), I've always somewhat struggled with quaternions. Most sources focus on quaternions as some tool for performing rotations in three-dimensions while avoiding gimbal lock. Which is true, they are/ that, but they're also more. After reading several articles about quaternions over the past several days, quaternions finally clicked and made sense! I'll try to share that insight with here here, though be warned that my description may be just as confusing (if not more so) than anywhere else.
author: kenton
published: 2015-11-04
category: math
tags: Quaternions
---

Although I have a pretty solid background in math (especially vectors, matrices, and even tensors), I've always somewhat struggled with *quaternions*. Most sources focus on quaternions as some tool for performing rotations in three-dimensions while avoiding gimbal lock. Which is true, they *are* that, but they're also more. After reading several articles about quaternions over the past several days, quaternions finally clicked and made sense! I'll try to share that insight with here here, though be warned that my description may be just as confusing (if not more so) than anywhere else.

In short, once I really understood that quaternions are simply four-dimensional [complex numbers](https://en.wikipedia.org/wiki/Complex_number), understanding their creation and use became a lot simpler. Quaternions are basically just four-dimensional vectors, who's orthonormal basis lies in some weird four-dimensional existence. That sounds like a mouthful, and to be honest, it kind of is. Let's take a step back and look at complex numbers. Actually, before that, let's look at orthonormal bases.

## Orthonormal Bases

If you don't know what an orthonormal basis is, that's probably just because you don't know their name. To quote [wikipedia](https://en.wikipedia.org/wiki/Orthonormal_basis):

&gt;In mathematics, particularly linear algebra, an orthonormal basis for an inner product space V with finite dimension is a basis for V whose vectors are orthonormal, that is, they are all unit vectors and orthogonal to each other.

That is to say, an orthonormal basis is a set of vectors which are **all** perpendicular to each other. You almost assuredly know one such basis: the `&lt;x, y, z&gt;` coordinate system (also called the "[Cartesian coordinate system](https://en.wikipedia.org/wiki/Cartesian_coordinate_system)"). Essentially each component of the basis represents a different dimension. There are many other orthonormal bases, for example: 2D Cartesian coordinates, polar coordinates (2D), cylindrical coordinates (3D), and spherical coordinates (3D) to name a few.

As it turns out, complex numbers *also* form an orthonormal basis. However, instead of representing physical dimensions, complex numbers represent a complex plane composed of *real* and *imaginary* components representing real and imaginary dimensions.

## Complex Numbers

Complex numbers are just two-dimensional vectors which are composed of both *real* and *imaginary* dimensions. In the 2D Cartesian coordinate system, vectors are composed of the `x` and `y` dimensions. In the complex plane, the imaginary dimension is given the label `i`, where:

$$
\hat{i}^2 = -1
$$

Which is an important identity to know, however we don't really need to use it often. Where in the Cartesian plane you might write a vector as such:

$$
\vec{v} = a \hat{x} + b \hat{y}
$$

In the complex plane, you might write a vector as:

$$
\vec{x} = a + b \hat{i}
$$

Where `a` represents the *real* part of the vector and `b` represents the *imaginary* part.

### Rotating with Complex Numbers

When rotating a vector in Cartesian coordinates, you can represent the rotation as a combination of `cos` and `sin` transforms in the two dimensions:

$$
R(\theta) = \cos(\theta)\hat{x} + \sin(\theta)\hat{y}
$$

Similarly, rotations in the complex plane can be represented as the combinations of `cos` and `sin` transforms in the two complex dimensions:

$$
R(\theta) = \cos\left(\theta\right) + \hat{i}\sin\left(\theta\right)
$$

Because of some neat math with complex numbers (including \\(i^2 = -1\\) formula above) which I won't repeat here, this can be reduced to:

$$
R(\theta) = e^{\hat{i} \theta}
$$

## Quaternions as Four-Dimensional Complex Numbers

Now that we have an understanding of complex numbers in two dimensions, it's pretty straightforward to extend the concept into the four dimensions necessary for quaternions---essentially all we do is define `j²` and `k²` dimensions to cast the vector into, defining the directions according to Hamilton's formula:

$$
\hat{i}^2 = \hat{j}^2 = \hat{k}^2 = \hat{i}\hat{j}\hat{k} = -1
$$

A quaternion can then be written as:

$$
\vec{q} = w + x\hat{i}^2 + y\hat{j}^2 + z\hat{k}^2
$$

Or, more commonly:

$$
\vec{q} = \left&lt;w, x, y, z\right&gt;
$$

Where `w` corresponds to the *real* dimension and `x`, `y`, and `z` correspond to the three *imaginary* dimensions.

And *that's it*. That's all quaternions really are.

Of course, quaternions are useful for all sorts of things, owing to some more neat math.

### Real and Pure Quaternions

If a quaternion's imaginary components are all equal to zero, then the quaternion is said to be "real":

$$
\vec{q}\_{real} = w
$$

Alternatively, if a quaternion's real component is equal to zero, then the quaternion is said to be "pure":

$$
\vec{q}\_{pure} = x\hat{i} + y\hat{j} + z\hat{k}
$$

Note that any quaternion can be expressed as the sum of its "real" and "pure" parts:

$$
\\begin{aligned}
\vec{q} &= \vec{q}\_{real} + \vec{q}\_{pure} \\\\
        &= \left(w\right) + \left(x\hat{i} + y\hat{j} + z\hat{k}\right)
\\end{aligned}
$$

### Rotations Using Quaternions

Since quaternions are composed of a single *real* component and three orthogonal *imaginary* components, they can be written similarly to vectors in our 2D complex plane:

$$
\begin{aligned}
\vec{q} &= w + \left&lt;x, y, z\right&gt; \cdot \left&lt;\hat{i}, \hat{j}, \hat{k}\right&gt; \\\\
        &= w + \vec{u} \cdot \vec{i}
\end{aligned}
$$

Look familiar?

Using the same multiplication formula as before, we get:

$$
R(\theta) = e^{(\vec{u} \cdot \vec{i})\theta}
$$

or:

$$
R(\theta) = \cos(\theta) + (\vec{u} \cdot \vec{i})\sin(\theta)
$$

By multiplying a vector by a quaternion (noting that to satisfy the math, we must use a four-dimensional vector, which we can set to be our three-dimensional vector with the fourth element set to 0), we get another quaternion:

$$
\begin{aligned}
\vec{p}' &= q\vec{p} \\\\
         &= \left&lt;w, \vec{u}\right&gt;\left&lt;0, \vec{p}\right&gt; \\\\
         &= \left&lt;-\vec{u}\cdot\vec{p}, w\vec{p} + \vec{u}\times\vec{p}\right&gt;
\end{aligned}
$$

Now, if the quaternion represents a rotation as defined above, the result should represent a rotated version of the vector `p`. Note that we essentially converted `p` to a "pure" quaternion, so we would expect `p'` to be a pure quaternion as well, from which we could extract the rotated vector. Somewhat unfortunately, this isn't the case for all but a few very specific circumstances. Most of the time, the result will be a mixed quaternion (meaning it will have *both* real and pure components), and the pure portion of it will not represent the origin vector (it will be longer). Fortunately, this can easily by solved by following the multiplication up by another multiplication---this time, by the inverse of `q`:

$$
\vec{p}' = q\vec{p}q^{-1}
$$

By adding this multiplication in, the resulting `p'` quaternion will be **pure** quaternion, with the complex parts representing the vector `p` rotated by the quaternion `q`. There's a catch however: since you effectively multiplied the vector twice (once by `q` and once by the inverse of `q`), the resulting vector gets rotated by `2θ`, meaning to rotate the vector only by `θ`, you need to construct `q` as if it was rotated by `0.5 θ`.

#### Constructing a Quaternion as a Rotation

Remembering the formula for `R(θ)` from before, we can construct a rotation quaternion as such:

$$
q(\theta) = \cos(\theta) + \sin(\theta)\hat{i} + \sin(\theta)\hat{j} + \sin(\theta)\hat{k}
$$

However, this suffers from the `2θ` issue mentioned above, so we actually want to construct it as so:

$$
q\left(\theta\right) = \cos\left(\frac{\theta}{2}\right) + \sin\left(\frac{\theta}{2}\right)\hat{i} + \sin\left(\frac{\theta}{2}\right)\hat{j} + \sin\left(\frac{\theta}{2}\right)\hat{k}
$$

The resulting quaterion `q` can now be used to rotate a vector in three dimensions! Not so shabby, eh?

To actually implement the rotation however, you'll need a couple more formulas---namely how to multiply quaternions, and how to calculate the inverse of a quaternion.

##### Multiplying Quaternions

The derivation of multiplying quaternions is fairly straightforward, if somewhat tedious. To save on tedium, I'll just give you the result here:

$$
\vec{q}\_1 \vec{q}\_2 = \left&lt;q\_{1,w}q\_{2,w} - \vec{q\_1}\cdot\vec{q\_2}, q\_{1,w}\vec{q}\_2 + q\_{2,w}\vec{q}\_1 + \vec{q}\_1 \times \vec{q\_2}\right&gt;
$$

##### Calculating the Inverse of a Quaternion

The inverse of a quaternion is given via the following formula:

$$
q^{-1} = \frac{q^*}{\left|q\right|^2}
$$

Where `q*` represents the conjugate of the quaternion, and is calculated as such:

$$
\begin{aligned}
q^* &= q\_w - q\_x\hat{i} - q\_y\hat{j} - q\_z\hat{k} \\\\
    &= \left&lt;q\_w, -\vec{u}\hat{i}\right&gt;
\end{aligned}
$$

## Conclusions

* Quaternions are just vectors (in four dimensions)
* The four dimensions are:
    - *real*
    - `i`
    - `j`
    - `k`
* 3D vectors can be written as a quaternion, where the `x-y-z` components of the vector map to the `i-j-k` components of the quaternion
* Quaternions can be used to rotate other quaternions using a couple of simple formula

For full in-depth discussion about quaternions, as well as to check out the sources I used for this, please check these places out:

* [http://www.3dgep.com/understanding-quaternions/](http://www.3dgep.com/understanding-quaternions/)
* [http://math.ucr.edu/~huerta/introquaternions.pdf](http://math.ucr.edu/~huerta/introquaternions.pdf)