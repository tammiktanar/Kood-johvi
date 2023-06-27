# Scene File Documentation

Scenes are organized into files, which are loaded in from the working directory when starting the ray tracer.
You can change what scene file is loaded in by putting the filename in the 1st positional argument.

The scene file uses [RON](https://github.com/ron-rs/ron/), which stands for "Rusty Object Notation".
I chose it instead of JSON because it supports commenting and is a bit easier to read I think.
There are also [extensions](https://github.com/ron-rs/ron/#tooling) for various editors.

## RON syntax overview

* Numbers: `42`, `3.14`, `0xFF`, `0b0110`
* Strings: `"Hello"`, `"with\\escapes\n"`, `r#"raw string, great for regex\."#`
* Booleans: `true`, `false`
* Chars: `'e'`, `'\n'`
* Optionals: `Some("string")`, `Some(Some(1.34))`, `None`
* Tuples: `("abc", 1.23, true)`, `()`
* Lists: `["abc", "def"]`
* Structs: `( foo: 1.0, bar: ( baz: "I'm nested" ) )`
* Maps: `{ "arbitrary": "keys", "are": "allowed" }`

## Scene structure

#### Defaults

Most values have reasonable defaults.
If something doesn't have a default, the program will tell you about it in an error.

#### Variations

Structs can have many forms. For example if you see `(/* OBJECT */)`,
then that means this accepts any of the code blocks under the **Object** header.

Some struct keys also accept alternate forms.
These are indicated by the next line having the same key, but commented out.
For example:

```ron
(
    material: (/* MATERIAL */)
    // material: "my_material"
)
```

### Root

```ron
(
    name: "my_scene", // Prepended to image filenames.
    sky: (0.2, 0.3, 0.5), // Emission of the sky.
    cameras: {
        // Each camera renders an image in sequence.
        // The key used for each camera will be appended the final render's filename.
        "my_camera": (/* CAMERA */),
    },
    objects: [
        // Zero or more OBJECT structs.
        (/* OBJECT */),
    ],
    templates: {
        // This contains named objects that can be referred to anywhere an object would normally be.
        "my_object": (/* OBJECT */)
    },
    materials: {
        // Zero or more MATERIAL structs.
        // All materials in here can be referred to by name in any object.
        "my_material": (/* MATERIAL */),
    },
    lights: [
        // Zero or more LIGHT structs.
        // In here are point lights and directional lights.
        // Emissive objects don't need to be added here explicitly.
        (/* LIGHT */),
    ],
    post_process: [
        // These are post process effects that are applied after the render from first to last.
        // See the full list of effects under the (/* EFFECT */) header.
        // Below are the default effects that are applied when the post_process field is missing entirely.
        (type: "tone_map"),
        (type: "gamma_correction", factor: 2.2),
    ],
)
```

### Camera

```ron
(
    pos: (0, 3, 3), // World coordinates for the camera.
    look_at: (0, 0, 0), // What to aim the camera at.
    up: (0, 0, 0), // What direction is up.
    bounces: 50, // How many times rays shot from this camera should bounce.
    samples: 1000, // How many samples to take per pixel.
    fov: 30, // The vertical field of view.
    width: 400, // Width of the rendered image.
    height: 200, // Height of the rendered image.
    aperature: 0, // Size of the aparature (adds bokeh).
    focus_distance: 1, // What distance is in focus (for bokeh).

    // The following settings don't change the look of the camera, but can help reduce rendering artifacts

    // Enabling this setting will disable NEE for area lights. 
    // Try it if your scene has many fireflies, which usually come from reflective surfaces.
    // Because my algorithm is kinda bad, this may yield a better result sometimes.
    indirect_only: false,
    // This will clamp the brightness of each sample below a certain threshold
    // Useful for reducing fireflies at the cost of caustic quality
    // No clamping is done if this field is missing
    clamping: 1000.0
)
```

### Object

###### NOTE: All of these count as an object struct, not just the 1st.

```ron
(
    type: "object",
    transform: (/* TRANSFORM */),
    // transforms: [(/* TRANSFORM */), ], // Chained transforms applied in order.
    shape: (/* SHAPE */),
    material: (/* MATERIAL */),
    // material: "my_material", // Can reference a material by name as well.
)
```

```ron
(
    // Groups many objects together.
    // Groups are flattened before rendering. If you want to avoid this behaviour, use a union.
    type: "group",
    transform: (/* TRANSFORM */),
    // transforms: [(/* TRANSFORM */), ],
    objects: [
        (/* OBJECT */),
    ],
)
```

```ron
(
    // Takes one object, and duplicates it for every given transform.
    // Instancers are also flattened before rendering.
    type: "instancer",
    object: (/* OBJECT */),
    transform_all: (/* TRANSFORM */), // This transform is applied to all copied objects.
    transforms: [
        (/* TRANSFORM */), // Every transform defined here spawns a new copy of the object.
        (/* TRANSFORM */), // Each copy will be positioned according to its transform.
    ],
)
```

```ron
(
    // Performs the union operation on any number of objects.
    // Unions have their interior walls removed, which only really matters for glass.
    type: "union",
    transform: (/* TRANSFORM */),
    // transforms: [(/* TRANSFORM */), ],
    objects: [
        (/* OBJECT */),
    ],
)
```

```ron
(
    // Performs the intersection operation on any number of objects.
    // All of the objects must intersect at some point for this to produce a result.
    type: "intersection",
    transform: (/* TRANSFORM */),
    // transforms: [(/* TRANSFORM */), ],
    objects: [
        (/* OBJECT */),
    ],
)
```

```ron
(
    // Performs the difference operation on any number of objects.
    // The rest of the objects are subracted from the 1st one.
    type: "difference",
    transform: (/* TRANSFORM */),
    // transforms: [(/* TRANSFORM */), ],
    objects: [
        (/* OBJECT */),
    ],
)
```

### Shape

Shapes can be defined in two forms: as a string, or as a struct.
To use the string form, just replace the entire struct with the string in the `type` field.
If the struct form has any extra parameters, these are set to default with the string form.

For example: `(type: "sphere")` can also be defined as `"sphere"`.

```ron
(
    // Note that most shapes don't have extra extra parameters.
    // This is intentional, just use the object's transform instead.
    type: "sphere",
)
```

```ron
(type: "box")
```

```ron
(type: "cylinder")
```

```ron
(type: "plane")
```

### Transform

###### NOTE: If you have trouble getting the rotations right, then try applying multiple rotation transforms instead.

```ron
(
    scale: 1.0,
    // scale: (1.0, 1.0, 1.0) // Non-uniform scaling
    rotate: (0, 0, 0), // Rotation around each of the axes in degrees. Applied in XZY order.
    translate: (0.0, 0.0, 0.0), // Position translation
)
```

### Material

```ron
(
    // Basic lambertian diffuse shading
    type: "diffuse",
    emission: (0, 0, 0), // How much light this emits, values can be greater than 1
    color: (1, 1, 1), // The base color in RGB
)
```

```ron
(
    // Glossy shading with lambertian diffuse and cook-torrance specular
    // High roughness may cause the material to become too dark. 
    // This is a flaw in my material implementation for now.
    type: "glossy",
    emission: (0, 0, 0),
    color: (1, 1, 1),
    roughness: 1.0,
    reflectance: 1.0,
)
```

```ron
(
    // Metallic shading with only cook-torrance specular
    // High roughness may cause the material to become too dark. 
    // This is a flaw in my material implementation for now.
    type: "metal",
    emission: (0, 0, 0),
    color: (1, 1, 1),
    roughness: 1.0,
)
```

```ron
(
    // Metallic shading with only cook-torrance specular
    type: "metal",
    emission: (0, 0, 0),
    color: (1, 1, 1),
    roughness: 1.0,
)
```

```ron
(
    // A perfect glass material
    type: "glass",
    emission: (0, 0, 0),
    color: (1, 1, 1),
    ior: 1.5, // Index of refraction. For example: Air=1.0, Glass=1.5
)
```

```ron
(
    // A perfect mirror material
    type: "mirror",
    emission: (0, 0, 0),
    color: (1, 1, 1),
)
```

## Light

```ron
(
    // A light that occupies a singular point.
    type: "point",
    position: (0, 0, 0),
    emission: (1, 1, 1),
)
```

```ron
(
    // A light shining in a certain direction from infinitely far away (like our sun)
    type: "directional",
    direction: (-1, -1, 0), // This example direction shines down and left
    emission: (1, 1, 1),
)
```

## Effect

```ron
(
    // Applies the ACES tone mapper
    type: "tone_map"
)
```

```ron
(
    // Applies gamma correction. This should come last usually.
    type: "gamma_correction",
    factor: 2.2,
)
```

```ron
(
    // Multiplies all colors by a scalar.
    type: "exposure",
    factor: "1.0",
)
```
