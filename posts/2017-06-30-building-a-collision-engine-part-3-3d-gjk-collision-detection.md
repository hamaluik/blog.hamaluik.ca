---
title: "Building a Collision Engine Part 3: 3D GJK Collision Detection"
slug: building-a-collision-engine-part-3-3d-gjk-collision-detection
author: kenton
tags: [Math, Haxe]
published: 2017-06-30T00:00:00-07:00
summary: "Now that I've conquered 2D collision detection and intersection calculation, it's time to move onto 3D. Thankfully, GJK is relatively easy to extend into 3D once we have the base framework that we developed before."
section: Programming Tutorials
---

Now that I've conquered 2D collision detection and intersection calculation, it's time to move onto 3D. Thankfully, GJK is relatively easy to extend into 3D once we have the base framework that we developed before. There are three main changes / additions we have to make to the original collision detection code to get it to work for 3D:

1. Change all the 2D data-types into 3D ones (`Vec2` ➜ `Vec3`)
2. Change the case when the simplex has 3 vertices to add a new vertex instead of checking for collision
3. Add a case when the simplex has 4 vertices to check for collision & evolve the simplex

The first bit is fairly obvious, so I won't go into it here. For the rest, it's important to note that the overall collision detection process is the same as before, we just calculate the normal of a triangle instead of the normal of a line.

For the rest of this post, I'll refer to the following tetrahedral simplex and corresponding points `A`, `B`, `C`, `D` which comprise the simplex:

<figure>
    <svg xmlns="http://www.w3.org/2000/svg" id="Layer_1" x="0" y="0" version="1.1" viewBox="0 0 1024 512" xml:space="preserve"><defs/><style>.simplex3d-st0{fill:currentColor}.simplex3d-st1,.simplex3d-st2{fill:none;stroke:var(--theme-red);stroke-width:4;stroke-linecap:round;stroke-linejoin:round;stroke-miterlimit:10}.simplex3d-st2{stroke:var(--theme-orange)}</style><path d="M841.1 200.4v-23.9h11.8c2.4 0 4.1.6 5.4 1.7 1.2 1.2 1.8 2.8 1.8 4.8 0 1-.3 2-.8 2.8-.6.8-1.2 1.4-2.1 1.7.5.2 1 .4 1.4.7.4.3.8.7 1.2 1.2.3.5.6 1 .8 1.7.2.6.3 1.4.3 2.3 0 2.2-.7 3.9-2.2 5.2-1.5 1.3-3.7 1.9-6.7 1.9h-10.9zm4.8-19.8v5.2h6.3c2 0 3-.9 3-2.6 0-.9-.3-1.6-.8-2-.5-.4-1.4-.6-2.5-.6h-6zm6.4 15.7c1.2 0 2.1-.3 2.7-1s1-1.5 1-2.4c0-1.1-.3-1.8-1-2.3-.6-.5-1.5-.7-2.6-.7H846v6.4h6.3zM882.7 192.2c-.3 1.4-.7 2.6-1.3 3.7-.6 1.1-1.3 2.1-2.2 2.8-.9.8-1.9 1.4-3 1.8-1.2.4-2.4.6-3.8.6-1.6 0-3-.3-4.4-.8-1.4-.5-2.5-1.3-3.5-2.4-1-1.1-1.8-2.4-2.3-3.9-.6-1.5-.8-3.4-.8-5.4 0-2 .3-3.8.8-5.5.5-1.6 1.3-3 2.2-4.1 1-1.1 2.1-2 3.5-2.6 1.4-.6 2.9-.9 4.6-.9 1.5 0 2.8.2 4.1.7 1.2.4 2.3 1.1 3.1 1.8.9.8 1.6 1.7 2.1 2.8.5 1.1.8 2.2 1 3.4h-5c-.3-1.4-1-2.4-1.9-3.2-1-.8-2-1.1-3.2-1.1-2.1 0-3.6.8-4.6 2.3-1 1.5-1.5 3.6-1.5 6.2 0 2.7.5 4.7 1.5 6.2s2.4 2.2 4.4 2.2c1.5 0 2.7-.4 3.6-1.2.9-.8 1.5-2 1.9-3.5h4.7zM883.5 200.4v-23.9h9.3c1.6 0 3.1.3 4.5.8s2.5 1.3 3.5 2.3 1.7 2.2 2.3 3.7c.6 1.4.8 3.1.8 4.9 0 1.8-.2 3.5-.7 5-.5 1.5-1.2 2.8-2.1 3.9-.9 1.1-2 1.9-3.3 2.5-1.3.6-2.7.9-4.2.9h-10.1zm9-4.1c2.2 0 3.8-.7 4.8-2 1-1.4 1.5-3.4 1.5-6.1 0-1.3-.1-2.5-.4-3.5-.2-1-.6-1.7-1.2-2.4-.5-.6-1.2-1.1-2.1-1.3-.8-.3-1.9-.4-3.1-.4h-3.7v15.7h4.2zM841.7 407.4l-1.5-4.9h-8.8l-1.7 4.9h-5.3l8.5-23.9h5.8l8.4 23.9h-5.4zm-5.9-18.5l-3.1 9.5h6.1l-3-9.5zM848.1 407.4v-23.9h11.8c2.4 0 4.1.6 5.4 1.7 1.2 1.2 1.8 2.8 1.8 4.8 0 1-.3 2-.8 2.8s-1.2 1.4-2.1 1.7c.5.2 1 .4 1.4.7.4.3.8.7 1.2 1.2s.6 1 .8 1.7c.2.6.3 1.4.3 2.3 0 2.2-.7 3.9-2.2 5.2-1.5 1.3-3.7 1.9-6.7 1.9h-10.9zm4.8-19.8v5.2h6.3c2 0 3-.9 3-2.6 0-.9-.3-1.6-.8-2-.5-.4-1.4-.6-2.5-.6h-6zm6.3 15.7c1.2 0 2.1-.3 2.7-1 .6-.6 1-1.5 1-2.4 0-1.1-.3-1.8-1-2.3-.6-.5-1.5-.7-2.6-.7h-6.4v6.4h6.3zM889.7 399.2c-.3 1.4-.7 2.6-1.3 3.7s-1.3 2.1-2.2 2.8c-.9.8-1.9 1.4-3 1.8-1.2.4-2.4.6-3.8.6-1.6 0-3-.3-4.4-.8-1.4-.5-2.5-1.3-3.5-2.4s-1.8-2.4-2.3-3.9c-.6-1.5-.8-3.4-.8-5.4 0-2 .3-3.8.8-5.5.5-1.6 1.3-3 2.2-4.1 1-1.1 2.1-2 3.5-2.6 1.4-.6 2.9-.9 4.6-.9 1.5 0 2.8.2 4 .7 1.2.4 2.3 1.1 3.1 1.8.9.8 1.6 1.7 2.1 2.8.5 1.1.8 2.2 1 3.4h-5c-.3-1.4-1-2.4-1.9-3.2-1-.8-2-1.1-3.2-1.1-2.1 0-3.6.8-4.6 2.3-1 1.5-1.5 3.6-1.5 6.2 0 2.7.5 4.7 1.5 6.2s2.4 2.2 4.4 2.2c1.5 0 2.7-.4 3.6-1.2.9-.8 1.5-2 1.9-3.5h4.8zM638.2 457.2c-.3 1.4-.7 2.6-1.3 3.7s-1.3 2.1-2.2 2.8c-.9.8-1.9 1.4-3 1.8s-2.4.6-3.8.6c-1.6 0-3-.3-4.4-.8-1.4-.5-2.5-1.3-3.5-2.4-1-1.1-1.8-2.4-2.3-3.9-.6-1.5-.8-3.4-.8-5.4 0-2 .3-3.8.8-5.5s1.3-3 2.2-4.1c1-1.1 2.1-2 3.5-2.6 1.4-.6 2.9-.9 4.6-.9 1.5 0 2.8.2 4.1.7 1.2.4 2.3 1.1 3.1 1.8.9.8 1.6 1.7 2.1 2.8.5 1.1.8 2.2 1 3.4h-5c-.3-1.4-1-2.4-1.9-3.2s-2-1.1-3.2-1.1c-2.1 0-3.6.8-4.6 2.3-1 1.5-1.5 3.6-1.5 6.2 0 2.7.5 4.7 1.5 6.2s2.4 2.2 4.4 2.2c1.5 0 2.7-.4 3.6-1.2.9-.8 1.5-2 1.9-3.5h4.7zM654 465.4l-1.5-4.9h-8.8l-1.7 4.9h-5.3l8.5-23.9h5.8l8.4 23.9H654zm-5.9-18.5l-3.1 9.5h6.1l-3-9.5zM660.4 465.4v-23.9h9.3c1.6 0 3.1.3 4.5.8s2.5 1.3 3.5 2.3c1 1 1.7 2.2 2.3 3.7.6 1.4.8 3.1.8 4.9 0 1.8-.2 3.5-.7 5-.5 1.5-1.2 2.8-2.1 3.9-.9 1.1-2 1.9-3.3 2.5-1.3.6-2.7.9-4.2.9h-10.1zm9-4.1c2.2 0 3.8-.7 4.8-2s1.5-3.4 1.5-6.1c0-1.3-.1-2.5-.4-3.5-.2-1-.6-1.7-1.2-2.4-.5-.6-1.2-1.1-2.1-1.3-.8-.3-1.9-.4-3.1-.4h-3.7v15.7h4.2zM700.7 139.4l-1.5-4.9h-8.8l-1.7 4.9h-5.3l8.5-23.9h5.8l8.4 23.9h-5.4zm-5.9-18.5l-3.1 9.5h6.1l-3-9.5zM707.1 139.4v-23.9h11.8c2.4 0 4.1.6 5.4 1.7 1.2 1.2 1.8 2.8 1.8 4.8 0 1-.3 2-.8 2.8s-1.2 1.4-2.1 1.7c.5.2 1 .4 1.4.7.4.3.8.7 1.2 1.2.3.5.6 1 .8 1.7.2.6.3 1.4.3 2.3 0 2.2-.7 3.9-2.2 5.2-1.5 1.3-3.7 1.9-6.7 1.9h-10.9zm4.8-19.8v5.2h6.3c2 0 3-.9 3-2.6 0-.9-.3-1.6-.8-2-.5-.4-1.4-.6-2.5-.6h-6zm6.3 15.7c1.2 0 2.1-.3 2.7-1 .6-.6 1-1.5 1-2.4 0-1.1-.3-1.8-1-2.3-.6-.5-1.5-.7-2.6-.7h-6.4v6.4h6.3zM727.6 139.4v-23.9h9.3c1.6 0 3.1.3 4.5.8s2.5 1.3 3.5 2.3 1.7 2.2 2.3 3.7c.6 1.4.8 3.1.8 4.9 0 1.8-.2 3.5-.7 5-.5 1.5-1.2 2.8-2.1 3.9-.9 1.1-2 1.9-3.3 2.5-1.3.6-2.7.9-4.2.9h-10.1zm9-4.1c2.2 0 3.8-.7 4.8-2 1-1.4 1.5-3.4 1.5-6.1 0-1.3-.1-2.5-.4-3.5-.2-1-.6-1.7-1.2-2.4-.5-.6-1.2-1.1-2.1-1.3-.8-.3-1.9-.4-3.1-.4h-3.7v15.7h4.2z" class="simplex3d-st0"/><path d="M937.1 275.3l9.9-108M937.1 275.3L894.3 84l52.7 83.3zM773 453l164.1 51.3 9.9-108zM947 396.3l-9.9 108M533 224l164.1 51.3M654.3 84L533 224 654.3 84zM697.1 275.3L654.3 84M707 396.3L533 453M654.3 313L533 453l121.3-140zM654.3 313l52.7 83.3" class="simplex3d-st1"/><path d="M64 363l333 104 20-219zM310 79L64 363 310 79z" class="simplex3d-st2"/><path d="M397 467L310 79l107 169z" class="simplex3d-st2"/><g><path d="M46.3 384.4l-2.5-8.2H29.1l-2.8 8.2h-8.8l14.2-39.8h9.7l14 39.8h-9.1zm-9.9-30.9l-5.2 15.8h10.1l-4.9-15.8z" class="simplex3d-st0"/></g><g><path d="M407.7 502.4v-39.8h19.6c3.9 0 6.9 1 8.9 2.9s3.1 4.6 3.1 7.9c0 1.7-.5 3.3-1.4 4.6-.9 1.3-2.1 2.3-3.5 2.9.8.3 1.6.7 2.4 1.2.7.5 1.4 1.1 1.9 1.9.5.8 1 1.7 1.3 2.8.3 1.1.5 2.3.5 3.8 0 3.6-1.2 6.5-3.7 8.7-2.4 2.1-6.2 3.2-11.2 3.2h-17.9zm8-32.9v8.6h10.5c3.4 0 5-1.4 5-4.3 0-1.5-.4-2.6-1.3-3.3-.9-.7-2.3-1-4.2-1h-10zm10.6 26.1c1.9 0 3.4-.5 4.5-1.6s1.6-2.4 1.6-4.1c0-1.8-.5-3-1.6-3.8-1.1-.8-2.5-1.2-4.4-1.2h-10.7v10.7h10.6z" class="simplex3d-st0"/></g><g><path d="M465.8 249.7c-.4 2.3-1.1 4.3-2.1 6.2-1 1.8-2.2 3.4-3.6 4.7-1.5 1.3-3.1 2.3-5.1 3.1-1.9.7-4 1.1-6.4 1.1-2.6 0-5-.4-7.3-1.3s-4.2-2.2-5.8-4c-1.6-1.8-2.9-3.9-3.9-6.5-.9-2.6-1.4-5.6-1.4-9 0-3.4.4-6.4 1.3-9.1.9-2.7 2.1-5 3.7-6.8 1.6-1.9 3.5-3.3 5.9-4.3 2.3-1 4.9-1.5 7.7-1.5 2.5 0 4.7.4 6.8 1.1 2 .7 3.8 1.8 5.2 3.1 1.5 1.3 2.6 2.8 3.5 4.6.9 1.8 1.4 3.6 1.6 5.6h-8.3c-.6-2.3-1.6-4.1-3.2-5.3-1.6-1.3-3.4-1.9-5.4-1.9-3.5 0-6 1.3-7.7 3.8-1.6 2.5-2.5 5.9-2.5 10.3s.8 7.9 2.4 10.4 4 3.7 7.3 3.7c2.5 0 4.5-.7 6-2s2.5-3.3 3.1-5.8h8.2z" class="simplex3d-st0"/></g><g><path d="M45 52.4L31.5 12.6h8.9l8.7 28 8.6-28h8.6L52.8 52.4H45zM70.7 39.7c.1 2.2.8 4 1.9 5.3 1.2 1.3 2.7 2 4.6 2 1.3 0 2.4-.3 3.4-.9 1-.6 1.6-1.4 1.9-2.4h8c-.9 3-2.5 5.4-4.8 7-2.3 1.6-5 2.5-8.2 2.5-9.8 0-14.8-5.4-14.8-16.1 0-2.3.3-4.3 1-6.2.6-1.8 1.6-3.4 2.8-4.7 1.2-1.3 2.7-2.3 4.5-3 1.8-.7 3.8-1 6.1-1 4.6 0 8 1.5 10.3 4.4 2.3 2.9 3.5 7.3 3.5 13.2H70.7zM83 34.8c0-1.1-.2-2-.6-2.9-.3-.8-.8-1.5-1.4-2-.6-.5-1.2-.9-1.9-1.2-.7-.3-1.5-.4-2.2-.4-1.6 0-2.9.6-4 1.7s-1.8 2.7-2 4.8H83zM93.4 52.4V22.9h7.3v3.5c.6-1 1.3-1.8 2-2.3.7-.6 1.4-1 2.2-1.3.7-.3 1.5-.5 2.3-.6.8-.1 1.5-.1 2.3-.1h1v8c-.7-.1-1.4-.2-2.2-.2-4.8 0-7.1 2.4-7.1 7.1v15.4h-7.8zM129 33h7.3v8.4c0 1.6-.2 3.1-.7 4.5-.5 1.4-1.2 2.7-2.2 3.8-1 1.1-2.2 2-3.7 2.6-1.5.6-3.3 1-5.4 1-2.2 0-4-.3-5.5-1-1.5-.6-2.8-1.5-3.8-2.6s-1.7-2.3-2.1-3.8c-.4-1.4-.7-3-.7-4.6V14.9h7.7v8h16.3v5.5H120v12.1c0 1.9.3 3.3 1 4.4.7 1 1.8 1.5 3.5 1.5 1.6 0 2.7-.5 3.4-1.5.7-1 1.1-2.4 1.1-4.3V33zM139.3 19.3V12h7.8v7.3h-7.8zm0 33.1V22.9h7.8v29.5h-7.8zM177 41.7c-.2 1.7-.7 3.3-1.5 4.7-.8 1.4-1.8 2.6-3 3.6s-2.6 1.8-4.1 2.3c-1.5.6-3.2.8-5 .8-2 0-3.8-.3-5.5-1-1.7-.7-3.1-1.7-4.3-3-1.2-1.3-2.1-3-2.8-5-.7-2-1-4.3-1-6.9 0-2.6.3-4.9 1-6.8.7-1.9 1.6-3.5 2.8-4.7 1.2-1.2 2.6-2.1 4.4-2.7 1.7-.6 3.6-.9 5.7-.9 1.9 0 3.7.3 5.3.8 1.6.5 3 1.3 4.1 2.3 1.2 1 2.1 2.2 2.8 3.6.7 1.4 1.1 3 1.2 4.7h-7.9c-.2-1.5-.8-2.7-1.8-3.6-1-.9-2.3-1.3-3.8-1.3-.8 0-1.6.1-2.3.4-.7.3-1.4.8-1.9 1.4-.5.7-1 1.5-1.3 2.6-.3 1.1-.5 2.5-.5 4.1 0 3.2.6 5.6 1.7 7.2 1.2 1.6 2.5 2.3 4 2.3s2.8-.4 3.9-1.3 1.7-2.1 1.9-3.7h7.9zM186.1 39.7c.1 2.2.8 4 1.9 5.3 1.2 1.3 2.7 2 4.6 2 1.3 0 2.4-.3 3.4-.9 1-.6 1.6-1.4 1.9-2.4h8c-.9 3-2.5 5.4-4.8 7-2.3 1.6-5 2.5-8.2 2.5-9.8 0-14.8-5.4-14.8-16.1 0-2.3.3-4.3 1-6.2.6-1.8 1.6-3.4 2.8-4.7 1.2-1.3 2.7-2.3 4.5-3 1.8-.7 3.8-1 6.1-1 4.6 0 8 1.5 10.3 4.4 2.3 2.9 3.5 7.3 3.5 13.2h-20.2zm12.3-4.9c0-1.1-.2-2-.6-2.9-.3-.8-.8-1.5-1.4-2-.6-.5-1.2-.9-1.9-1.2-.7-.3-1.5-.4-2.2-.4-1.6 0-2.9.6-4 1.7s-1.8 2.7-2 4.8h12.1zM226.6 31.8c-.2-1.4-.7-2.3-1.5-3-.8-.6-2.1-.9-3.8-.9-1.6 0-2.8.2-3.6.6-.8.4-1.2 1-1.2 2 0 .8.4 1.4 1.2 1.9.8.5 2 .9 3.6 1.4 2.6.7 4.7 1.3 6.5 1.8 1.8.5 3.2 1.1 4.2 1.8 1.1.7 1.8 1.5 2.3 2.6.5 1 .7 2.4.7 4.1 0 2.6-1.1 4.8-3.3 6.6-2.2 1.7-5.5 2.6-9.8 2.6-2.1 0-4-.2-5.8-.7-1.7-.5-3.2-1.2-4.4-2.1-1.2-.9-2.2-2-2.8-3.2-.7-1.3-1-2.7-1-4.2h8c0 1.4.5 2.5 1.6 3.2 1.1.8 2.5 1.1 4.3 1.1 1.5 0 2.7-.2 3.8-.7 1.1-.5 1.6-1.2 1.6-2.1 0-1.1-.4-1.9-1.2-2.3-.8-.5-2-.9-3.6-1.3-2.9-.6-5.2-1.3-7-2-1.8-.7-3.1-1.5-4.1-2.3-1-.8-1.6-1.8-2-2.8-.3-1-.5-2.1-.5-3.4 0-1.1.2-2.2.7-3.2.5-1 1.2-1.9 2.2-2.6 1-.8 2.3-1.4 3.9-1.8 1.6-.5 3.6-.7 5.9-.7 4.2 0 7.3.9 9.3 2.6 2 1.7 3 4.1 3.2 7.1h-7.4z" class="simplex3d-st0"/></g><g><path d="M701.7 57.4V17.6h17.6c2 0 3.7.3 5.3 1 1.6.7 2.9 1.6 4 2.8 1.1 1.2 2 2.5 2.6 4.1.6 1.6.9 3.2.9 5 0 1.8-.3 3.5-.9 5-.6 1.5-1.5 2.9-2.6 4s-2.5 2-4 2.6c-1.6.6-3.3.9-5.3.9H710v14.5h-8.3zm16.2-21.2c2.4 0 4.1-.5 5.2-1.6 1.1-1.1 1.6-2.5 1.6-4.3 0-1.7-.5-3.1-1.6-4.1-1.1-1.1-2.8-1.6-5.2-1.6H710v11.7h7.9zM733.6 57.4V17.1h7.8v40.3h-7.8zM762.9 57.4c-.2-.8-.4-1.6-.5-2.6-.9 1-2.1 1.9-3.7 2.5-1.5.6-3.3.9-5.3.9-3.4 0-5.9-.8-7.5-2.3s-2.4-3.4-2.4-5.8c0-2.1.3-3.8 1-5.1.7-1.3 1.6-2.3 2.8-3 1.2-.7 2.6-1.3 4.3-1.6 1.7-.3 3.5-.6 5.4-.9 2-.3 3.3-.6 4-1.1.7-.5 1-1.2 1-2.3 0-1-.4-1.7-1.3-2.3-.9-.5-2.1-.8-3.6-.8-1.8 0-3.1.4-3.9 1.2-.8.8-1.3 1.9-1.5 3.2h-7.3c0-1.5.3-2.9.8-4.2.5-1.3 1.2-2.4 2.2-3.3 1-.9 2.3-1.6 4-2.1 1.6-.5 3.6-.8 6-.8 2.3 0 4.3.3 5.9.8 1.6.5 2.9 1.3 3.9 2.3 1 1 1.7 2.2 2.1 3.7.4 1.5.7 3.1.7 4.9v18.7h-7.1zm-.5-14.5c-.4.4-1 .7-1.7 1s-1.9.5-3.3.8c-2.2.4-3.8 1-4.6 1.7-.8.7-1.3 1.7-1.3 2.9 0 2.2 1.3 3.3 3.8 3.3 1 0 1.9-.2 2.8-.5.9-.3 1.6-.8 2.2-1.4.6-.6 1.1-1.3 1.5-2 .4-.8.6-1.6.6-2.5v-3.3zM792.2 57.4V39.5c0-2.3-.4-3.8-1.1-4.6-.8-.8-2.1-1.2-3.9-1.2-4 0-6.1 2.3-6.1 6.8v16.9h-7.8V27.9h7.4v4.3c.9-1.7 2.1-2.9 3.7-3.8 1.5-.9 3.5-1.3 5.9-1.3 1.4 0 2.7.2 3.9.6 1.2.4 2.2 1 3.1 1.9.9.8 1.5 1.9 2 3.1.5 1.2.8 2.6.8 4.2v20.5h-7.9zM810.3 44.7c.1 2.2.8 4 1.9 5.3 1.2 1.3 2.7 2 4.7 2 1.3 0 2.4-.3 3.4-.9 1-.6 1.6-1.4 1.9-2.4h8c-.9 3-2.5 5.4-4.8 7-2.3 1.6-5 2.5-8.2 2.5-9.8 0-14.8-5.4-14.8-16.1 0-2.3.3-4.3 1-6.2.6-1.8 1.6-3.4 2.8-4.7 1.2-1.3 2.7-2.3 4.5-3 1.8-.7 3.8-1 6.1-1 4.6 0 8 1.5 10.3 4.4 2.3 2.9 3.5 7.3 3.5 13.2h-20.3zm12.3-4.9c0-1.1-.2-2-.6-2.9-.3-.8-.8-1.5-1.4-2-.6-.5-1.2-.9-1.9-1.2-.7-.3-1.5-.4-2.2-.4-1.6 0-2.9.6-4 1.7s-1.8 2.7-2 4.8h12.1zM850.8 36.8c-.2-1.4-.7-2.3-1.5-3-.8-.6-2.1-.9-3.8-.9-1.6 0-2.8.2-3.6.6-.8.4-1.2 1-1.2 2 0 .8.4 1.4 1.2 1.9.8.5 2 .9 3.6 1.4 2.6.7 4.7 1.3 6.5 1.8 1.8.5 3.2 1.1 4.2 1.8 1.1.7 1.8 1.5 2.3 2.6.5 1 .7 2.4.7 4.1 0 2.6-1.1 4.8-3.3 6.6-2.2 1.7-5.5 2.6-9.8 2.6-2.1 0-4-.2-5.8-.7-1.7-.5-3.2-1.2-4.4-2.1-1.2-.9-2.2-2-2.8-3.2-.7-1.3-1-2.7-1-4.2h8c0 1.4.5 2.5 1.6 3.2 1.1.8 2.5 1.1 4.3 1.1 1.5 0 2.7-.2 3.8-.7 1.1-.5 1.6-1.2 1.6-2.1 0-1.1-.4-1.9-1.2-2.3-.8-.5-2-.9-3.6-1.3-2.9-.6-5.2-1.3-7-2-1.8-.7-3.1-1.5-4.1-2.3-1-.8-1.6-1.8-2-2.8-.3-1-.5-2.1-.5-3.4 0-1.1.2-2.2.7-3.2.5-1 1.2-1.9 2.2-2.6 1-.8 2.3-1.4 3.9-1.8 1.6-.5 3.6-.7 5.9-.7 4.2 0 7.3.9 9.3 2.6 2 1.7 3 4.1 3.2 7.1h-7.4z" class="simplex3d-st0"/></g><g><path d="M305.7 71.4V31.6h15.4c2.7 0 5.2.4 7.4 1.3 2.3.9 4.2 2.2 5.8 3.9 1.6 1.7 2.9 3.7 3.8 6.1.9 2.4 1.4 5.1 1.4 8.2 0 3-.4 5.8-1.2 8.3-.8 2.5-2 4.6-3.5 6.4-1.5 1.8-3.3 3.2-5.5 4.1-2.1 1-4.5 1.5-7 1.5h-16.6zm15-6.8c3.6 0 6.3-1.1 7.9-3.4 1.6-2.3 2.5-5.6 2.5-10.2 0-2.2-.2-4.2-.6-5.8-.4-1.6-1.1-2.9-2-3.9s-2.1-1.8-3.5-2.2c-1.4-.5-3.1-.7-5.1-.7h-6.2v26.2h7z" class="simplex3d-st0"/></g></svg>
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
    <svg xmlns="http://www.w3.org/2000/svg" id="Layer_1" x="0" y="0" version="1.1" viewBox="0 0 1024 512" xml:space="preserve"><defs/><style>.insideoutside-st0{fill:currentColor}.insideoutside-st2{fill:none;stroke:var(--theme-red);stroke-width:4;stroke-linecap:round;stroke-linejoin:round;stroke-miterlimit:10}.insideoutside-st4{fill:var(--theme-purple)}</style><path d="M256.3 384.4l-2.5-8.2h-14.7l-2.8 8.2h-8.8l14.2-39.8h9.7l14 39.8h-9.1zm-9.9-30.9l-5.2 15.8h10.1l-4.9-15.8z" class="insideoutside-st0"/><path fill="var(--theme-orange)" d="M520 79l87 388-333-104" opacity=".75"/><path d="M274 363l333 104 20-219z" class="insideoutside-st2"/><path fill="none" stroke="currentColor" stroke-linecap="round" stroke-miterlimit="10" stroke-width="4" d="M463 279l142.1 19.7"/><path d="M629 302c-11.8 2.6-26.8 7.8-36.5 14.2l9.4-17.9-4.2-19.8c7.7 8.7 20.6 17.8 31.3 23.5z" class="insideoutside-st0"/><path d="M520 79L274 363 520 79zM607 467L520 79l107 169z" class="insideoutside-st2"/><path d="M635.1 318c-.9-.2-1.7-.5-2.4-.8-.6-.4-1.1-.8-1.5-1.3s-.7-1.2-.8-2c-.2-.8-.3-1.7-.3-2.7v-3.7h5v4.9h-2.8c0 1.2.3 2 .7 2.6s1.1 1 2.1 1.2v1.8zm6.6 0c-.9-.2-1.7-.5-2.4-.8-.6-.4-1.1-.8-1.5-1.3-.4-.5-.7-1.2-.8-2-.2-.8-.3-1.7-.3-2.7v-3.7h5v4.9h-2.8c0 1.2.3 2 .7 2.6.5.6 1.1 1 2.1 1.2v1.8zM643.4 332.4v-23.9h5v23.9h-5zM661.4 332.4v-10.7c0-1.4-.2-2.3-.7-2.8s-1.2-.7-2.3-.7c-2.4 0-3.6 1.4-3.6 4.1v10.2h-4.7v-17.7h4.5v2.6c.6-1 1.3-1.8 2.2-2.3.9-.5 2.1-.8 3.5-.8.8 0 1.6.1 2.3.4.7.3 1.3.6 1.9 1.1.5.5.9 1.1 1.2 1.9.3.7.5 1.6.5 2.5v12.3h-4.8zM678.7 320c-.1-.8-.4-1.4-.9-1.8s-1.2-.6-2.3-.6c-1 0-1.7.1-2.2.3s-.7.6-.7 1.2c0 .5.2.9.7 1.2.5.3 1.2.6 2.1.8 1.5.4 2.8.8 3.9 1.1 1.1.3 1.9.7 2.5 1.1.6.4 1.1.9 1.4 1.5.3.6.4 1.4.4 2.5 0 1.6-.7 2.9-2 3.9s-3.3 1.6-5.9 1.6c-1.3 0-2.4-.1-3.5-.4-1-.3-1.9-.7-2.6-1.2s-1.3-1.2-1.7-1.9c-.4-.8-.6-1.6-.6-2.5h4.8c0 .8.3 1.5 1 1.9.7.5 1.5.7 2.6.7.9 0 1.6-.1 2.3-.4.6-.3 1-.7 1-1.3 0-.6-.2-1.1-.7-1.4-.5-.3-1.2-.5-2.1-.8-1.7-.4-3.1-.8-4.2-1.2-1.1-.4-1.9-.9-2.5-1.4-.6-.5-1-1.1-1.2-1.7-.2-.6-.3-1.3-.3-2s.1-1.3.4-1.9c.3-.6.7-1.1 1.3-1.6.6-.5 1.4-.8 2.3-1.1 1-.3 2.1-.4 3.5-.4 2.5 0 4.4.5 5.6 1.5s1.8 2.5 1.9 4.3h-4.3zM684.9 312.6v-4.4h4.7v4.4h-4.7zm0 19.8v-17.7h4.7v17.7h-4.7zM703.9 332.4v-2.2c-1.1 1.8-2.7 2.6-4.9 2.6-1.1 0-2.2-.2-3.1-.6-.9-.4-1.7-1.1-2.4-1.9s-1.2-1.8-1.6-3c-.4-1.2-.6-2.5-.6-4 0-1.3.2-2.6.5-3.7s.8-2.1 1.4-2.9c.6-.8 1.4-1.4 2.3-1.9.9-.4 1.9-.7 3.1-.7 2.2 0 3.9.9 5.3 2.8v-8.8h4.6v24.2h-4.6zM700 329c1.1 0 2.1-.5 2.8-1.4s1.1-2.2 1.1-3.7c0-3.8-1.3-5.8-3.9-5.8-2.7 0-4 1.8-4 5.5 0 1.6.4 2.9 1.1 3.9s1.7 1.5 2.9 1.5zM714.5 324.8c.1 1.3.5 2.4 1.2 3.2.7.8 1.6 1.2 2.8 1.2.8 0 1.4-.2 2-.5s1-.8 1.1-1.5h4.8c-.6 1.8-1.5 3.2-2.9 4.2s-3 1.5-4.9 1.5c-5.9 0-8.9-3.2-8.9-9.6 0-1.4.2-2.6.6-3.7.4-1.1.9-2.1 1.7-2.8.7-.8 1.6-1.4 2.7-1.8s2.3-.6 3.6-.6c2.7 0 4.8.9 6.2 2.6 1.4 1.8 2.1 4.4 2.1 7.9h-12.1zm7.4-3c0-.6-.1-1.2-.3-1.7-.2-.5-.5-.9-.8-1.2s-.7-.6-1.2-.7-.9-.2-1.3-.2c-.9 0-1.7.3-2.4 1s-1.1 1.6-1.2 2.9h7.2zM728.1 316.2c.9-.2 1.6-.6 2.1-1.2s.7-1.4.7-2.6h-2.8v-4.9h5v3.7c0 1-.1 1.9-.3 2.7-.2.8-.4 1.4-.8 2s-.9 1-1.5 1.3c-.6.4-1.4.6-2.4.8v-1.8zm6.6 0c.9-.2 1.6-.6 2.1-1.2s.7-1.4.7-2.6h-2.8v-4.9h5v3.7c0 1-.1 1.9-.3 2.7-.2.8-.4 1.4-.8 2-.4.5-.9 1-1.5 1.3-.6.4-1.4.6-2.4.8v-1.8z" class="insideoutside-st0"/><g><path d="M213.1 223c-.9-.2-1.7-.5-2.4-.8-.6-.4-1.1-.8-1.5-1.4-.4-.5-.7-1.2-.8-2-.2-.8-.3-1.7-.3-2.7v-3.7h5v4.9h-2.8c0 1.2.3 2 .7 2.6.5.6 1.1 1 2.1 1.2v1.9zm6.6 0c-.9-.2-1.7-.5-2.4-.8-.6-.4-1.1-.8-1.5-1.4-.4-.5-.7-1.2-.8-2-.2-.8-.3-1.7-.3-2.7v-3.7h5v4.9h-2.8c0 1.2.3 2 .7 2.6.5.6 1.1 1 2.1 1.2v1.9zM232.6 238.2c-3.7 0-6.5-1.1-8.5-3.2-2-2.2-3-5.3-3-9.3 0-2 .3-3.8.8-5.4s1.2-3 2.2-4.1c1-1.1 2.2-2 3.6-2.6 1.4-.6 3-.9 4.9-.9 1.8 0 3.4.3 4.9.9 1.4.6 2.6 1.4 3.6 2.6 1 1.1 1.7 2.5 2.2 4.1.5 1.6.8 3.5.8 5.5 0 4.1-1 7.2-2.9 9.3-2.1 2.1-4.9 3.1-8.6 3.1zm0-4.3c.9 0 1.7-.2 2.5-.5s1.5-.8 2.1-1.5c.6-.7 1-1.5 1.4-2.5.3-1 .5-2.3.5-3.7 0-2.8-.6-5-1.7-6.4-1.1-1.5-2.7-2.2-4.8-2.2-2.1 0-3.7.7-4.9 2.2-1.1 1.5-1.7 3.6-1.7 6.4 0 2.8.6 4.9 1.7 6.2 1.2 1.4 2.8 2 4.9 2zM257 237.4v-2.5c-1.2 2-3 3-5.5 3-.9 0-1.7-.2-2.4-.5s-1.4-.8-1.9-1.3c-.5-.6-.9-1.3-1.2-2.1-.3-.8-.5-1.7-.5-2.7v-11.7h4.7v10.9c0 2.2 1 3.3 3 3.3 1.2 0 2.1-.4 2.7-1.1.6-.8.9-1.7.9-2.9v-10.2h4.6v17.7H257zM273.3 225.7h4.4v5c0 .9-.1 1.8-.4 2.7-.3.9-.7 1.6-1.3 2.3-.6.7-1.3 1.2-2.2 1.6-.9.4-2 .6-3.2.6-1.3 0-2.4-.2-3.3-.6-.9-.4-1.7-.9-2.3-1.5-.6-.6-1-1.4-1.3-2.3-.3-.9-.4-1.8-.4-2.7V215h4.6v4.8h9.8v3.3h-9.8v7.3c0 1.1.2 2 .6 2.6.4.6 1.1.9 2.1.9.9 0 1.6-.3 2.1-.9.4-.6.7-1.5.7-2.6v-4.7zM290 225c-.1-.8-.4-1.4-.9-1.8-.5-.4-1.2-.6-2.3-.6-1 0-1.7.1-2.2.3-.5.2-.7.6-.7 1.2 0 .5.2.9.7 1.2.5.3 1.2.6 2.1.8 1.5.4 2.8.8 3.9 1.1 1.1.3 1.9.7 2.5 1.1.6.4 1.1.9 1.4 1.5.3.6.4 1.4.4 2.5 0 1.6-.7 2.9-2 3.9s-3.3 1.6-5.9 1.6c-1.3 0-2.4-.1-3.5-.4-1-.3-1.9-.7-2.6-1.2-.7-.5-1.3-1.2-1.7-1.9s-.6-1.6-.6-2.5h4.8c0 .8.3 1.5 1 1.9.7.5 1.5.7 2.6.7.9 0 1.6-.1 2.3-.4.6-.3 1-.7 1-1.3 0-.6-.2-1.1-.7-1.4-.5-.3-1.2-.5-2.1-.8-1.7-.4-3.1-.8-4.2-1.2-1.1-.4-1.9-.9-2.5-1.4-.6-.5-1-1.1-1.2-1.7-.2-.6-.3-1.3-.3-2s.1-1.3.4-1.9c.3-.6.7-1.1 1.3-1.6.6-.5 1.4-.8 2.3-1.1 1-.3 2.1-.4 3.5-.4 2.5 0 4.4.5 5.6 1.5 1.2 1 1.8 2.5 1.9 4.3H290zM296.3 217.6v-4.4h4.7v4.4h-4.7zm0 19.8v-17.7h4.7v17.7h-4.7zM315.2 237.4v-2.2c-1.1 1.8-2.7 2.6-4.9 2.6-1.1 0-2.2-.2-3.1-.6-.9-.4-1.7-1.1-2.4-1.9s-1.2-1.8-1.6-3c-.4-1.2-.6-2.5-.6-4 0-1.3.2-2.6.5-3.7s.8-2.1 1.4-2.9c.6-.8 1.4-1.4 2.3-1.9.9-.4 1.9-.7 3.1-.7 2.2 0 3.9.9 5.3 2.8v-8.8h4.6v24.2h-4.6zm-3.9-3.4c1.1 0 2.1-.5 2.8-1.4s1.1-2.2 1.1-3.7c0-3.8-1.3-5.8-3.9-5.8-2.7 0-4 1.8-4 5.5 0 1.6.4 2.9 1.1 3.9.8 1 1.8 1.5 2.9 1.5zM325.9 229.8c.1 1.3.5 2.4 1.2 3.2.7.8 1.6 1.2 2.8 1.2.8 0 1.4-.2 2-.5.6-.3 1-.8 1.1-1.5h4.8c-.6 1.8-1.5 3.2-2.9 4.2s-3 1.5-4.9 1.5c-5.9 0-8.9-3.2-8.9-9.6 0-1.4.2-2.6.6-3.7.4-1.1.9-2.1 1.7-2.8.7-.8 1.6-1.4 2.7-1.8 1.1-.4 2.3-.6 3.6-.6 2.7 0 4.8.9 6.2 2.6 1.4 1.8 2.1 4.4 2.1 7.9h-12.1zm7.4-3c0-.6-.1-1.2-.3-1.7-.2-.5-.5-.9-.8-1.2-.3-.3-.7-.6-1.2-.7s-.9-.2-1.3-.2c-.9 0-1.7.3-2.4 1s-1.1 1.6-1.2 2.9h7.2zM339.4 221.2c.9-.2 1.6-.6 2.1-1.2s.7-1.4.7-2.6h-2.8v-4.9h5v3.7c0 1-.1 1.9-.3 2.7-.2.8-.4 1.4-.8 2-.4.5-.9 1-1.5 1.4-.6.4-1.4.6-2.4.8v-1.9zm6.7 0c.9-.2 1.6-.6 2.1-1.2s.7-1.4.7-2.6h-2.8v-4.9h5v3.7c0 1-.1 1.9-.3 2.7-.2.8-.4 1.4-.8 2-.4.5-.9 1-1.5 1.4-.6.4-1.4.6-2.4.8v-1.9z" class="insideoutside-st4"/></g><g><path d="M617.7 502.4v-39.8h19.6c3.9 0 6.9 1 8.9 2.9s3.1 4.6 3.1 7.9c0 1.7-.5 3.3-1.4 4.6-.9 1.3-2.1 2.3-3.5 2.9.8.3 1.6.7 2.4 1.2.7.5 1.4 1.1 1.9 1.9.5.8 1 1.7 1.3 2.8.3 1.1.5 2.3.5 3.8 0 3.6-1.2 6.5-3.7 8.7-2.4 2.1-6.2 3.2-11.2 3.2h-17.9zm8-32.9v8.6h10.5c3.4 0 5-1.4 5-4.3 0-1.5-.4-2.6-1.3-3.3-.9-.7-2.3-1-4.2-1h-10zm10.6 26.1c1.9 0 3.4-.5 4.5-1.6s1.6-2.4 1.6-4.1c0-1.8-.5-3-1.6-3.8-1.1-.8-2.5-1.2-4.4-1.2h-10.7v10.7h10.6z" class="insideoutside-st0"/></g><g><path d="M515.7 71.4V31.6h15.4c2.7 0 5.2.4 7.4 1.3 2.3.9 4.2 2.2 5.8 3.9 1.6 1.7 2.9 3.7 3.8 6.1s1.4 5.1 1.4 8.2c0 3-.4 5.8-1.2 8.3-.8 2.5-2 4.6-3.5 6.4-1.5 1.8-3.3 3.2-5.5 4.1-2.1 1-4.5 1.5-7 1.5h-16.6zm15-6.8c3.6 0 6.3-1.1 7.9-3.4 1.6-2.3 2.5-5.6 2.5-10.2 0-2.2-.2-4.2-.6-5.8-.4-1.6-1-2.9-1.9-3.9-.9-1-2.1-1.8-3.5-2.2-1.4-.5-3.1-.7-5.1-.7h-6.2v26.2h6.9z" class="insideoutside-st0"/></g><g><path fill="none" stroke="var(--theme-purple)" stroke-linecap="round" stroke-miterlimit="10" stroke-width="4" d="M463 279l-142.1-19.7"/><path d="M297 256c11.8-2.6 26.8-7.8 36.5-14.2l-9.4 17.9 4.2 19.8c-7.7-8.7-20.6-17.8-31.3-23.5z" class="insideoutside-st4"/></g></svg>
    <figcaption>We need to check whether the origin is 'inside' or 'outside' plane <code>ABD</code> (then again for <code>BCD</code> and <code>CAD</code>).</figcaption>
</figure>

Just like before, we check whether the origin is 'inside' or 'outside' the plane by comparing the direction of the plane's normal with the direction to the origin. If they're in the same direction, the origin is outside the plane and we need to evolve the simplex. Otherwise, the origin is inside the plane and we can test the other planes (or conclude the origin _must_ be inside the simplex).

<figure>
    <svg xmlns="http://www.w3.org/2000/svg" id="Layer_1" x="0" y="0" version="1.1" viewBox="0 0 1024 512" xml:space="preserve"><defs/><style>.tnorm-st0{fill:none;stroke:currentColor;stroke-width:4;stroke-linecap:round;stroke-linejoin:round}.tnorm-st1{fill:currentColor}.tnorm-st2{fill:none;stroke:var(--theme-red);stroke-width:4;stroke-linecap:round;stroke-linejoin:round;stroke-miterlimit:10}.tnorm-st4{fill:var(--theme-purple)}</style><path d="M274 363l-109.1-49.9" class="tnorm-st0"/><path d="M143 303c12.1.9 27.9.2 39-3.1l-14.2 14.4-1.7 20.2c-4.7-10.5-14.5-22.9-23.1-31.5z" class="tnorm-st1"/><path d="M627 248L274 363" class="tnorm-st2"/><path d="M256.3 414.4l-2.5-8.2h-14.7l-2.8 8.2h-8.8l14.2-39.8h9.7l14 39.8h-9.1zm-9.9-30.9l-5.2 15.8h10.1l-4.9-15.8z" class="tnorm-st1"/><path d="M607 467l20-219M607 467L520 79l107 169z" class="tnorm-st2"/><path d="M198.9 288.4v-1.1c.8-.2 1.4-.3 1.7-.5s.7-.4.9-.8.3-.8.3-1.3-.1-1.1-.2-1.7-.3-1.3-.5-2.1l-.9-3h-10.9l-.8 2.3c-.4 1.2-.7 2-.8 2.6s-.2 1.1-.2 1.6c0 .9.2 1.6.7 2.1s1.2.7 2.2.9v1.1h-10v-1.1c.9-.1 1.6-.6 2.3-1.5s1.3-2.1 1.9-3.8l9.8-25.9h3.7l8 26.1c.4 1.2.7 2.1 1 2.8s.6 1.1 1 1.5 1 .6 1.7.8v1.1h-10.9zm-9-12.7h9.8l-4.5-15.1-5.3 15.1zM213 288.4v-1.1c1-.3 1.6-.5 1.9-.8s.5-.7.6-1.3.2-1.4.2-2.6v-20.3c0-1.1 0-1.9-.1-2.4s-.2-.9-.4-1.1-.4-.5-.7-.7-.8-.3-1.5-.6v-1.1h12.8c2.5 0 4.5.3 6 .8s2.6 1.3 3.4 2.4 1.1 2.4 1.1 4c0 1.4-.3 2.5-.8 3.5s-1.2 1.8-2.1 2.5-1.9 1.3-3.3 1.8v.2c2.1.6 3.7 1.6 4.8 3s1.6 2.9 1.6 4.8c0 2.1-.5 3.9-1.4 5.2s-2.3 2.3-4 2.9-3.8.9-6.2.9H213zm11.3-17.7c2.4 0 4.2-.6 5.4-1.7s1.9-2.8 1.9-5c0-1.1-.2-2-.5-2.7s-.8-1.3-1.5-1.7-1.3-.7-2.2-.9-1.8-.3-2.7-.3h-2.3c-.9 0-1.7.1-2.2.1v12.1h4.1zm-4.1 15.5c1.1.1 2.4.2 3.8.2 2 0 3.6-.3 4.8-.8s2-1.3 2.5-2.3.8-2.3.8-3.8-.3-2.8-.8-3.8-1.4-1.7-2.6-2.2-2.8-.7-4.8-.7h-3.6v13.4zM259.7 274.7l7.5 7.4-3.1 3.1-7.4-7.5-7.4 7.5-3.1-3.1 7.5-7.4-7.5-7.4 3.1-3.1 7.4 7.5 7.4-7.5 3.1 3.1-7.5 7.4zM292.5 288.4v-1.1c.8-.2 1.4-.3 1.7-.5s.7-.4.9-.8.3-.8.3-1.3-.1-1.1-.2-1.7-.3-1.3-.5-2.1l-.9-3h-10.9l-.8 2.3c-.4 1.2-.7 2-.8 2.6s-.2 1.1-.2 1.6c0 .9.2 1.6.7 2.1s1.2.7 2.2.9v1.1h-10v-1.1c.9-.1 1.6-.6 2.3-1.5s1.3-2.1 1.9-3.8l9.8-25.9h3.7l8 26.1c.4 1.2.7 2.1 1 2.8s.6 1.1 1 1.5 1 .6 1.7.8v1.1h-10.9zm-9-12.7h9.8l-4.5-15.1-5.3 15.1zM306.5 288.4v-1.1c.7-.2 1.2-.4 1.5-.5s.6-.4.7-.7.3-.6.4-1.1.1-1.3.1-2.4v-20.3c0-1.1 0-1.9-.1-2.4s-.2-.9-.4-1.1-.4-.5-.7-.6-.8-.3-1.5-.6v-1.1h10.3c3 0 5.4.3 7.3 1s3.5 1.6 4.8 2.9 2.3 2.8 2.9 4.7 1 4.1 1 6.7c0 2.6-.3 5-.9 7s-1.5 3.7-2.6 5.1c-1 1.2-2.2 2.2-3.7 2.9-1.2.6-2.6 1.1-4.2 1.3s-3.7.4-6.3.4h-8.6zm7.2-2.2c.6.1 1.5.1 2.8.1 1.3 0 2.5-.1 3.5-.3s2-.6 2.8-1 1.6-1.1 2.3-2 1.2-1.8 1.6-2.9.7-2.3.9-3.6.3-2.8.3-4.4c0-3.2-.5-5.8-1.4-7.8s-2.2-3.5-3.9-4.5-3.7-1.4-6-1.4c-1.1 0-2.1 0-3 .1v27.7zM617.7 502.4v-39.8h19.6c3.9 0 6.9 1 8.9 2.9s3.1 4.6 3.1 7.9c0 1.7-.5 3.3-1.4 4.6-.9 1.3-2.1 2.3-3.5 2.9.8.3 1.6.7 2.4 1.2.7.5 1.4 1.1 1.9 1.9.5.8 1 1.7 1.3 2.8.3 1.1.5 2.3.5 3.8 0 3.6-1.2 6.5-3.7 8.7-2.4 2.1-6.2 3.2-11.2 3.2h-17.9zm8-32.9v8.6h10.5c3.4 0 5-1.4 5-4.3 0-1.5-.4-2.6-1.3-3.3-.9-.7-2.3-1-4.2-1h-10zm10.6 26.1c1.9 0 3.4-.5 4.5-1.6s1.6-2.4 1.6-4.1c0-1.8-.5-3-1.6-3.8-1.1-.8-2.5-1.2-4.4-1.2h-10.7v10.7h10.6z" class="tnorm-st1"/><path fill="none" stroke="var(--theme-purple)" stroke-linecap="round" stroke-linejoin="round" stroke-width="4" d="M504.2 97.2L274 363l307.8 96.1"/><path d="M520 79c-10.6 5.8-25.3 11.8-36.8 13.4l18.9 7.3 9.9 17.7c0-11.7 3.7-27.1 8-38.4zM604.8 466.3c-12.1.6-27.7 3.3-38.3 8l12.2-16.1-.9-20.2c6.1 9.8 17.4 20.9 27 28.3z" class="tnorm-st4"/><path d="M515.7 71.4V31.6h15.4c2.7 0 5.2.4 7.4 1.3 2.3.9 4.2 2.2 5.8 3.9 1.6 1.7 2.9 3.7 3.8 6.1s1.4 5.1 1.4 8.2c0 3-.4 5.8-1.2 8.3-.8 2.5-2 4.6-3.5 6.4-1.5 1.8-3.3 3.2-5.5 4.1-2.1 1-4.5 1.5-7 1.5h-16.6zm15-6.8c3.6 0 6.3-1.1 7.9-3.4 1.6-2.3 2.5-5.6 2.5-10.2 0-2.2-.2-4.2-.6-5.8-.4-1.6-1-2.9-1.9-3.9-.9-1-2.1-1.8-3.5-2.2-1.4-.5-3.1-.7-5.1-.7h-6.2v26.2h6.9z" class="tnorm-st1"/><path d="M429.9 411.7c31.7-42.2 35.1-121.5-17.7-171.3" class="tnorm-st0"/><path d="M393.2 225.4c11.5 3.8 27 7.1 38.6 6.6l-17.3 10.5-6.6 19.1c-2-11.4-8.5-25.8-14.7-36.2z" class="tnorm-st1"/></svg>
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

3D collision detection with GJk is definitely a lot easier than I originally thought it was, especially after you get all the concepts figured out in 2D. For reference, here is the complete 3D collision detection class at the time of writing from my [Headbutt](https://github.com/hamaluik/headbutt) library:

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
    <iframe style="width: 100%; height: 200px; border: 0;" src="/images/collision-engine-3d-detection/demo.html"></iframe>
</figure>

## Headbutt

I've started rolling this code into it's own library, tentatively called _Headbutt_, which you can follow along with if you're interested on Github: [https://github.com/hamaluik/headbutt](https://github.com/hamaluik/headbutt).
