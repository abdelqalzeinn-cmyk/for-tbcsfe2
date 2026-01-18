# Save File

See <https://codeberg.org/fieryhenry/bc_ree> for a starting tutorial on how to reverse engineer
the game.

## Adding support for new a new game version

You will want to find that `readEndingData` function and any new parsing code will be near the end
of that function.

Once you know the structure of the new data, you should create a new file in `src/blocks/` called
`gv_XXXXXX.rs` where `XXXXXX` is the game version (e.g `gv_140500.rs`).

In that file you should create a public struct called `GVXXXXXXBlock` which must impliment
`stream::Readable`, `stream::Writable`, `Debug`, `Clone` and `Default`. You can use derive macros
for this.

Note that the `Readable` and `Writable` derive proc macros are found in `bcsfe_derive`.

Most game version blocks in the save file end with the game version, so the `end_assert` attribute
should be added to the struct by adding a `#[rw(end_assert = XXXXXX)]` where `XXXXXX` is the game
version. This will read a 32 integer after reading everything else in the struct and compare it to
that value. It will also add code to write that value to the end of the struct during save file
writing.

Here is an example:

```rust
use bcsfe_derive::{Readable, Writable};

#[derive(Debug, Clone, Readable, Writable, Default)]
#[rw(end_assert = 140500)]
pub struct GV140500Block {
  // struct content here...
}

```

## Readable / Wriable Derive Macro

Can be used to automaically derive `stream::Readable` and `stream::Writable` for named structs.

All field need to impliment `Readable` / `Writable` (or be an Option<T> where `T` impliments
`Readable` / `Writable` see below).

Fields will be read / written in the order they are defined in the struct.

If you need to control whether certain fields should be read/written to depending on game version
and country code, the `Readable` and `Writable` derive macros have an attribute called `rw`
which has the following options:

- `gvcc: bool` - Whether to pass the `GVCC` struct into the `read` and `write` methods of the field. Defaults to `false`.
- `min_gv: Option<u32>` - The minimum (inclusive) game version required to read/write this field. Defaults to `0`.
- `max_gv: Option<u32>` - The maximum (inclusive) game version required to read/write this field. Defaults to `u32::MAX`.
- `en: Option<bool>` - Whether to read/write this field if the country code is `en`. Defaults to `true`.
- `jp: Option<bool>` - Whether to read/write this field if the country code is `jp`. Defaults to `true`.
- `kr: Option<bool>` - Whether to read/write this field if the country code is `kr`. Defaults to `true`.
- `tw: Option<bool>` - Whether to read/write this field if the country code is `tw`. Defaults to `true`.

Any fields using any of the above attributes (except `gvcc`) must be an `Option<T>` that impliments
`Default`.

See the `gv_100600_en` field in `save::Save`, which is for game versions greater than or equal to
10.6.0 and only for en:

```rust
..
#[rw(min_gv = 100600, jp = false, kr = false, tw = false)]
pub gv_100600_en: Option<gv_100600::GV100600BlockEn>,
..
```


## Manual Implimentation of Readable and Writable

If the structure you are reading/writing is quite complicated you may have to impliment `stream::Readable`
and `stream::Writable` yourself.

For example take `gv_90400::ExtraEnigmaData` which is an optional struct that only exists on
game versions greater than 14.5.0, and starts with a boolean flag whether this extra data even
exists.

```rust
#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct Engima {
    pub energy_since_1: i32,
    pub energy_since_2: i32,
    pub enigma_level: i8,
    pub unknown_1: i8,
    pub unknown_2: bool,
    // with = "<type>" will read as "<type>" then call .into to turn it into the field type
    // for writing it will call .into to turn it into "<type>" then write as "<type>"
    #[rw(with = "LengthVec<i8, EnigmaStage>")]
    pub stages: Vec<EnigmaStage>,
    #[rw(gvcc)] // notice this since [`ExtraEnigmaData`] has `GVCC` as arguments.
    pub extra_data: ExtraEnigmaData,
}

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
pub struct ExtraEnigmaDataInner {
    pub u1: i32,
    pub u2: i32,
    pub u3: i8,
    pub u4: f64,
}

/// Wrapper struct used to avoid having to manually impliment Readable and Writable for the
/// inner data.
#[derive(Debug, Clone, Copy, Default)]
pub struct ExtraEnigmaData(pub Option<ExtraEnigmaDataInner>);

impl Readable for ExtraEnigmaData {
    type Args<'a> = GVCC;
    fn read<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        args: Self::Args<'_>,
    ) -> StreamResult<Self> {
        match args.gv.0 {
            0..140500 => Ok(Self(None)),
            _ => {
                let has_extra = bool::read_no_opts(reader)?;

                match has_extra {
                    true => Ok(Self(Some(ExtraEnigmaDataInner::read_no_opts(reader)?))),
                    false => Ok(Self(None)),
                }
            }
        }
    }
}


impl Writable for ExtraEnigmaData {
    type Args<'a> = GVCC;
    fn write<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
        args: Self::Args<'_>,
    ) -> StreamResult<()> {
        match args.gv.0 {
            0..140500 => (),
            _ => match self.0 {
                Some(item) => {
                    true.write_no_opts(writer)?;
                    item.write_no_opts(writer)?;
                }
                None => {
                    false.write_no_opts(writer)?;
                }
            },
        };

        Ok(())
    }
}
```

`read_no_opts` and `write_no_opts` are methods from the `WritableNoOptions` and `ReadableNoOptions`
traits respetively and can be used if the `Args` type is empty, ie an empty tuple - `()`. If the
value you are reading/writing needs arguments you will have to use the `read` and `write` methods.

To make the error messages clearer you can add context to them by using the `add_context` method
like so:

```rust
_ = i32::read_no_opts(reader).add_context(|| "reading catfood")?;
// The closure must return something which impliments `ToString`
```

## Useful Structs

There are various generic structs to help with the parsing of the save file.

### LengthVec

This is used to read/write a variable length list from the save file where the length is stored
before the list contents.

This is a tuple struct which has 2 type parameters:

- `L`: this is the data type of the length. It must impliment `stream::ToUsize`, `stream::FromUsize`.
  It also must impliment `stream::Readable` and `stream::Writable` with `()` as arguments.
- `T`: the type of the item in the Vec.

### LengthString

Used to read/write length prefixed strings in the save file.

It has an `L` type parameter which has the same constraints as in <#LengthVec>.

Most strings in the save file are prefixed by an `i32`.

### HashMapLength

Used to read/write variable length hashmaps in the save file.

It has 3 type paramters:

- `L`: same as in <#LengthVec>
- `K`: data type of the hashmap key, must impliment `Readable` and `Writable` with no args.
  It must also impliment `Hash`,`Eq`, `Default` and `Display`. `Display` is just for printing an error message
  if the key already exists in the hashmap when reading.
- `V`: data type of the hashmap value, must impliment `Readable` and `Writable` with no args.
  It must also impliment `Debug` and `Default`.

### Using for fields

These structs are quite powerful, and should be used using the `with` attribute argument.

The type specified in `with` must impliment `Into<FieldType>` and `From<FieldType>` where `FieldType`
is the type of the field.

For example see `map_resets` in `gv_72::GV72Block`:

```rust
#[rw(with="HashMapLength<i32, i32, LengthVec<i32, MapResetData>>")]
pub map_resets: HashMap<i32, Vec<MapResetData>>,
```

Or zombie outbreaks in `gv_59::GV59Block`:

```rust
#[rw(with="HashMapLength<i32, i32, HashMapLength<i32, i32, bool>>")]
pub outbreaks: HashMap<i32, HashMap<i32, bool>>,
```

## GVCC

This is a struct with the following content:

```rust
pub struct GVCC {
    pub cc: CountryCode,
    pub gv: GameVersion,
}
```

It can be used to read/write different things depending on the game version.
