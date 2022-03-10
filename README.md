A Rust library for generating static-compatible code from serializable data.

# Motivation

This library was created for easily and generically integrating data like assets directly into a binary, providing both
efficient and ergonomic access to the hierarchical structure of the data. The main driver is the
[VES](https://github.com/knonderful/ves) project, in which game assets are created in an editor and then integrated into
the game code.

# Design

`staticgen` takes arbitrary data for which the `Serialize` trait has been implemented and generates Rust source code
that represents the data structure as a static reference (`&'static`). Since the source type(s) may contain types that
can not be represented by a static reference, such as a `String` or `Vec`, `staticgen` also creates type definitions for
the output it generates.

`staticgen`'s serializer writes the static source code to the provided output as the source data is being serialized by
Serde. With this approach the caller can avoid having to collect all the data in memory before writing it out to a file.
Note that the caller may still buffer all the data, if he chooses to, by letting the serializer write into a
`Vec<u8>`, for example.

After serialization is complete the type definitions that were created by the serializer can also be written to an
output. A common use case is to use `staticgen` in a `build.rs` file and thus generate source code for some persisted
data as part of the build process. This generated code can then be included in the source code tree and referenced from
regular code.

## Example

This is an example where some "movie" data is stored in a file in [`bincode`](https://github.com/bincode-org/bincode)
format (which is the output of some other tool). This data is loaded in `build.rs` and static code and the accompanying
types are written to Rust source code files, which are then integrated into the main code for the target application.

**build.rs:**

```rust
const INPUT_PATH: &'static str = "assets/movie.bincode";

fn main() -> Result<()> {
    let movie = load_movie_data()?;
    generate_static_code(&movie)?;

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={INPUT_PATH}");

    Ok(())
}

fn load_movie_data() -> Result<Movie> {
    let movie_file_path = PathBuf::from(INPUT_PATH);
    let movie_file = File::open(&movie_file_path)?;
    Ok(bincode::deserialize_from(movie_file)?)
}

fn generate_static_code(movie: &Movie) -> Result<()> {
    const OUTPUT_METHODS_PATH: &'static str = "src/generated/methods.rs";
    let generated_methods_file = File::create(OUTPUT_METHODS_PATH)?;
    let mut serializer = staticgen::Serializer::new(generated_methods_file);
    writeln!(serializer.out_mut(), "use crate::generated::types::*;")?;
    writeln!(serializer.out_mut(), "")?;
    writeln!(
        serializer.out_mut(),
        "pub const fn palettes() -> &'static [Palette] {{"
    )?;

    use serde::Serialize as _;
    movie.palettes().serialize(&mut serializer)?;

    writeln!(serializer.out_mut(), "}}")?;
    writeln!(serializer.out_mut(), "")?;
    writeln!(
        serializer.out_mut(),
        "pub const fn frames() -> &'static [MovieFrame] {{"
    )?;

    let frames = movie
        .frames()
        .chunks(10)
        .next()
        .ok_or_else(|| anyhow!("Got no frames."))?;
    frames.serialize(&mut serializer)?;

    writeln!(serializer.out_mut(), "}}")?;

    let structs = std::mem::take(serializer.structs_mut());
    let enums = std::mem::take(serializer.enums_mut());

    const OUTPUT_TYPES_PATH: &'static str = "src/generated/types.rs";
    let mut generated_types_file = File::create(OUTPUT_TYPES_PATH)?;
    structs.write(&mut generated_types_file)?;
    enums.write(&mut generated_types_file)?;

    rust_format::format_file(OUTPUT_TYPES_PATH)?;
    rust_format::format_file(OUTPUT_METHODS_PATH)?;

    Ok(())
}
```

This example shows how two different functions (`palettes()` and `frames()`) are generated and written to `methods.rs`
in the source tree. It also shows how the generated types (contained in `structs` and `enums`) are written to
`types.rs`. From that point, using the generated code in the normal source code is rather trivial:

**src/generated.rs:**

```rust
pub mod methods;
pub mod types;
```

**src/main.rs:**

```rust
static PALETTES: &'static [crate::generated::types::Palette] = crate::generated::methods::palettes();
static FRAMES: &'static [crate::generated::types::MovieFrame] = crate::generated::methods::frames();

fn main() {
    // We can print the values easily, since the Debug trait is implemented for the generated types.

    println!("The following palettes are available:");
    for palette in PALETTES {
        println!("  {palette:?}");
    }

    println!("The following frames are available:");
    for frame in FRAMES {
        println!("  {frame:?}");
    }
}
```

# Type mapping

`staticgen` uses Serde for serialization. This means that the data types available to `staticgen` are determined by
the [Serde data model](https://serde.rs/data-model.html). `staticgen` never "sees" the original types (structs and
enums) on which Serde's `serialize()` method was called, which means it has no choice but to generate types from what it
receives from Serde. With that in mind, `staticgen` applies the following type mappings.

| Serde type      | Rust type                                                                                            |
|-----------------|------------------------------------------------------------------------------------------------------|
| bool            | `bool`                                                                                               |
| i8              | `i8`                                                                                                 |
| i16             | `i16`                                                                                                |
| i32             | `i32`                                                                                                |
| i64             | `i64`                                                                                                |
| i128            | `i128`                                                                                               |
| u8              | `u8`                                                                                                 |
| u16             | `u16`                                                                                                |
| u32             | `u32`                                                                                                |
| u64             | `u64`                                                                                                |
| u128            | `u128`                                                                                               |
| f32             | `f32`                                                                                                |
| f64             | `f64`                                                                                                |
| char            | `char`                                                                                               |
| string          | `&'static str`                                                                                       |
| byte_array      | `&'static [u8]`                                                                                      |
| option          | `Option<T>` or `Option<()>` <sup>(see [Indeterminite generics](#indeterminite-generics))</sup>       |
| unit            | `()`                                                                                                 |
| unit_struct     | `struct Sample`                                                                                      |
| unit_variant    | `enum Sample { First }` <sup>(see [Enums](#enums))</sup>                                             |
| newtype_struct  | `struct Sample(Inner)`                                                                               |
| newtype_variant | `enum Sample { First(Inner) }` <sup>(see [Enums](#enums))</sup>                                      |
| seq             | `&'static [T]` or `&'static [()]` <sup>(see [Indeterminite generics](#indeterminite-generics))</sup> |
| tuple           | `(First, Inner, Other)`                                                                              |
| tuple_struct    | `struct(First, Inner, Other)`                                                                        |
| tuple_variant   | `enum Sample { First(Inner, Other) }` <sup>(see [Enums](#enums))</sup>                               |
| map             | `struct Generated1 { name: First, language: First }` <sup>(see [Maps](#maps))</sup>                  |
| struct          | `struct Sample { first: First, inner: Inner }`                                                       |
| struct_variant  | `enum Sample { First{ inner: Inner, other: Other } }` <sup>(see [Enums](#enums))</sup>               |

## Enums

`staticgen` can only generate enums from the variants it sees in the input data, due to its dependence on what Serde
provides during serialization. Consider the following definition for the original enum of a serializable object:

```rust
#[derive(serde::Serialize)]
enum ActionButton {
    Start,
    Select,
    A,
    B,
}
```

If the input data fed into `staticgen` only contains the variants `ActionButton::A` and `ActionButton::Start`, then it
will generate the following type for this data:

```rust
enum ActionButton {
    Start,
    A,
}
```

## Indeterminite generics

For the Serde types "option" and "seq" it can happen that `staticgen` never sees a "contained" value, because the input
data does not contain such a value (e.g. contains only `None` or empty `Vec`s for a type). In this case the generic `T`
is internally marked as "undefined" and, when written to some output, will result in the Rust unit type: `()`.

## Maps

`staticgen` only supports maps which have a "string" as a key. From such a map it generates a new struct where the field
names correspond to the key values in the map. The values are treated as any other Serde value (see
[Type mapping](#type-mapping)).

Consider the following example:

```rust
use serde::Serialize;
use staticgen::Serializer;
use std::collections::HashMap;
use std::io::Sink;
use std::default::Default;

#[derive(Serialize)]
struct TvShow {
    title: String,
    actors: HashMap<String, u32>,
}

pub fn main() {
    let mut actors = HashMap::new();
    actors.insert(String::from("john_james"), 12);
    actors.insert(String::from("freddy"), 42);
    let tv_show = TvShow {
        title: String::from("Hello Show"),
        actors
    };

    let mut serializer = Serializer::new(Sink::default());
    tv_show.serialize(&mut serializer).unwrap();
    serializer.structs().write(&mut std::io::stdout()).unwrap();
}
```

With this input `staticgen` will generate the following type definitions:

```rust
struct TvShow {
    name: &'static str,
    actors: Generated1,
}

struct Generated1 {
    john_james: u32,
    freddy: u32,
}
```

# Crate state

This crate is mainly developed for use in the VES project. As such, it only contains features that are needed for that
project. Any more advanced features, like non-`String` maps or finer control over the type generation, will not be added
until needed. Furthermore, the API should be considered to be very unstable: breaking changes may happen with any new
release.