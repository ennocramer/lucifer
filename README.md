# Lucifer

Lucifer is a physically based renderer, written as a tutorial on how
to implement raytracing and pathtracing in Rust.

![example](example.png)

Lucifer tries to showcase a very clean and easy-to-understand
implementation of the basics of 3D rendering using raytracing.

The first few [commits](https://github.com/ennocramer/lucifer) are
structured as a tutorial, showing the implementation of the core
structures and algorithms of a [path
tracing](https://en.wikipedia.org/wiki/Path_tracing) renderer.

## Building

Lucifer is written in [Rust](https://www.rust-lang.org/) and may,
depending on when you're trying to build it, require on the nightly
release of the Rust compiler.

``` sh
# Install Rust via Rustup
$ curl https://sh.rustup.rs -sSf | sh

# Install nightly release, if necessary
$ rustup install nightly
$ rustup default nightly

# Build lucifer
$ cargo build --release

# Run lucifer to generate the example output
$ cargo run --release example.png
```

## Core Concepts

### Space

Space is represented using the `Vector` and `Point` types, taken from
the [cgmath crate](https://crates.io/crates/cgmath).  `Point`
represents positions in space, while `Vector` represents directions
and distances between points.

A `Ray` is a straight line through space.  It has an origin, a
direction, and a possibly infinite length.  Rays represent the path
photons travel unhindered after being emitted, until interacting
(possibly being absorbed or reflected) with the surface of an object.

An `Intersection` describes a location where a photon hits, and
interacts with, an object. It consists of the point in space where the
intersection occurred, the object's surface's normal vector, the
distance from the photon's origin, and a boolean indicating whether
the surface was hit from the object's inside or outside.

A `Geometry` is the abstraction of a physical shape. It provides
methods for intersections tests between itself and `Ray`s.

### Energy

Light is represented using the `Radiance` structure, which models the
radiant intensity of light coming from a given direction. Radiant
intensity defines the color and brightness of light.

The color of a surface is represented by `Albedo`.  An `Albedo` is a
multiplicative factor for `Radiance`, with the assumption that any
value is between zero and one.  I.e. an `Albedo` cannot increase
`Radiance` and cannot turn `Radiance` negative.

The appearance of a surface is modeled as a set of `Effect`s, each
representing a specific type of interaction between light and the
surface.  The set of `Effect`s is called `Bsdf`.

Each effect is composed of an effect type, a `Distribution`, and some
combination of `Radiance`, `Albedo`, and `Ior`.  There are five
different effects:

  1. **Emission** is light emitted by a surface, independently of
     incoming light. This is the primary `Effect` for light sources.

     Emission is defined by a `Radiance` value and a distribution
     function.

  2. **Diffuse Reflection** is light scattered by the surface.  This
     is the primary `Effect` for rough surfaces.

     Diffuse reflection is defined by an `Albedo` value and a
     distribution function.  The `Distribution` is centered on the
     surface normal

  3. **Specular Reflection** is light reflected by the surface.  This
     is the primary `Effect` for shiny surfaces and mirrors.

     Specular reflection is defined by an `Albedo` value and a
     distribution function.  The `Distribution` is centered on the
     reflected incidence direction.

  4. **Diffuse Refraction** is light transmitted, but scattered by the
     surface. This is the primary `Effect` for light-transmitting, but
     non-transparent objects.

     Diffuse refraction is defined by an `Albedo` value and a
     distribution function. The `Distribution` is centered on the
     mirrored surface normal.

  5. **Specular Refraction** is light transmitted and refracted by the
     surface. This is the primary `Effect` for clear, transparent
     objects.

     Specular refraction can turn into reflection, if the angle of
     incidence is low.

     Specular refraction is defined by an `Albedo` and `Ior` value and
     a distribution function.  The `Distribution` is centered on the
     refracted incidence direction.

The `Material` trait is responsible to compute the surface's
appearance at the point of intersection.  The generated set of
`Effect`s and their attributes can vary depending on the attributes of
the `Intersection`.
