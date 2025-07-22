use std::{
    any::type_name,
    collections::HashMap,
    fmt::Display,
    hash::Hash,
    io::{Read, Seek, Write},
    marker::PhantomData,
};

#[derive(Debug)]
pub struct StreamError {
    pub error: std::io::Error,
    pub pos: u64,
    pub context: Vec<String>,
}

impl std::error::Error for StreamError {}

impl StreamError {
    pub fn new_context(error: std::io::Error, pos: u64, context: Vec<String>) -> Self {
        Self {
            error,
            pos,
            context,
        }
    }

    pub fn new(error: std::io::Error, pos: u64) -> Self {
        Self {
            error,
            pos,
            context: Vec::new(),
        }
    }

    pub fn new_str(error: &str, pos: u64) -> Self {
        Self::new(std::io::Error::other(error), pos)
    }

    pub fn add_context<C: ToString>(self, new_context: C) -> Self {
        let mut context = self.context;
        context.push(new_context.to_string());
        Self::new_context(self.error, self.pos, context)
    }

    pub fn new_string_context<E: ToString, C: ToString>(error: E, pos: u64, context: C) -> Self {
        Self {
            error: std::io::Error::other(error.to_string()),
            pos,
            context: vec![context.to_string()],
        }
    }
}

impl Display for StreamError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.pos == u64::MAX {
            return write!(f, "{}", self.error);
        }
        let string = format!("error: {} at {} with context:", self.error, self.pos);

        let mut context_str = String::new();

        for ctx in &self.context {
            context_str.push_str(&format!("\n{ctx}"));
        }

        write!(f, "{string}\n{context_str}")
    }
}

pub type StreamResult<T> = Result<T, StreamError>;

pub trait NewResultCtx {
    fn add_context<C: ToString, F: FnOnce() -> C>(self, context: F) -> Self;
}

impl<T> NewResultCtx for StreamResult<T> {
    fn add_context<C: ToString, F: FnOnce() -> C>(self, context: F) -> Self {
        match self {
            Ok(v) => Ok(v),
            Err(e) => Err(e.add_context(context())),
        }
    }
}

pub trait Readable: Sized {
    type Args<'a>;
    fn read<R: Read + Seek>(reader: &mut R, args: Self::Args<'_>) -> StreamResult<Self>;
}

pub trait Writable: Sized {
    type Args<'a>;
    fn write<W: Write + Seek>(&self, writer: &mut W, args: Self::Args<'_>) -> StreamResult<()>;
}

pub trait ResultCtx: Sized {
    type OkT;
    fn with_context<C: ToString, F: FnOnce() -> C>(
        self,
        pos: u64,
        context: F,
    ) -> StreamResult<Self::OkT>;
    fn stream_context<C: ToString, F: FnOnce() -> C>(self, context: F) -> StreamResult<Self::OkT>;
    fn with_pos(self, pos: u64) -> StreamResult<Self::OkT>;
}

impl From<std::io::Error> for StreamError {
    fn from(value: std::io::Error) -> Self {
        Self::new(value, u64::MAX)
    }
}

impl<T> ResultCtx for Result<T, std::io::Error> {
    type OkT = T;
    fn with_context<C: ToString, F: FnOnce() -> C>(
        self,
        pos: u64,
        context: F,
    ) -> StreamResult<<Self as ResultCtx>::OkT> {
        match self {
            Ok(v) => StreamResult::Ok(v),
            Err(e) => StreamResult::Err(StreamError::new_context(
                e,
                pos,
                vec![context().to_string()],
            )),
        }
    }
    fn with_pos(self, pos: u64) -> StreamResult<Self::OkT> {
        match self {
            Ok(v) => StreamResult::Ok(v),
            Err(e) => StreamResult::Err(StreamError::new(e, pos)),
        }
    }
    fn stream_context<C: ToString, F: FnOnce() -> C>(self, context: F) -> StreamResult<Self::OkT> {
        self.with_context(u64::MAX, context)
    }
}

fn read_data<R: Read + Seek, const N: usize>(reader: &mut R) -> StreamResult<[u8; N]> {
    let mut buf = [0; N];
    reader
        .read_exact(&mut buf)
        .with_context(reader.stream_position()?, || format!("read_data<{N}>"))?;
    Ok(buf)
}

fn write_data<W: Write + Seek>(data: &[u8], writer: &mut W) -> StreamResult<()> {
    writer
        .write_all(data)
        .with_context(writer.stream_position()?, || "write_data")
}

pub trait ReadableNoOptions: Sized {
    fn read_no_opts<R: Read + Seek>(reader: &mut R) -> StreamResult<Self>;
}

pub trait WritableNoOptions {
    fn write_no_opts<W: Write + Seek>(&self, writer: &mut W) -> StreamResult<()>;
}

impl<'a, T: Readable<Args<'a> = ()>> ReadableNoOptions for T {
    fn read_no_opts<R: Read + Seek>(reader: &mut R) -> StreamResult<Self> {
        T::read(reader, ())
    }
}

impl<'a, T: Writable<Args<'a> = ()>> WritableNoOptions for T {
    fn write_no_opts<W: Write + Seek>(&self, writer: &mut W) -> StreamResult<()> {
        self.write(writer, ())
    }
}

macro_rules! impl_readable {
    ($typ:ty) => {
        impl Readable for $typ {
            type Args<'a> = ();
            fn read<R: Read + Seek>(reader: &mut R, _args: Self::Args<'_>) -> StreamResult<Self> {
                Ok(Self::from_le_bytes(
                    read_data(reader).add_context(|| format!("read {}", type_name::<Self>()))?,
                ))
            }
        }
    };
}

impl_readable!(u8);
impl_readable!(u16);
impl_readable!(u32);
impl_readable!(u64);
impl_readable!(f32);
impl_readable!(f64);
impl_readable!(i8);
impl_readable!(i16);
impl_readable!(i32);
impl_readable!(i64);

macro_rules! impl_writeable {
    ($typ:ty) => {
        impl Writable for $typ {
            type Args<'a> = ();
            fn write<W: Write + Seek>(
                &self,
                writer: &mut W,
                _args: Self::Args<'_>,
            ) -> StreamResult<()> {
                write_data(&self.to_le_bytes(), writer)
                    .add_context(|| format!("write {}", type_name::<Self>()))
            }
        }
    };
}

impl_writeable!(u8);
impl_writeable!(u16);
impl_writeable!(u32);
impl_writeable!(u64);
impl_writeable!(i8);
impl_writeable!(i16);
impl_writeable!(i32);
impl_writeable!(i64);
impl_writeable!(f32);
impl_writeable!(f64);

impl Readable for bool {
    type Args<'a> = ();
    fn read<R: Read + Seek>(reader: &mut R, _args: Self::Args<'_>) -> StreamResult<Self> {
        let data: u8 = u8::read_no_opts(reader).add_context(|| "read u8 for bool")?;

        Ok(data != 0)
    }
}

impl Writable for bool {
    type Args<'a> = ();
    fn write<W: Write + Seek>(&self, writer: &mut W, _args: Self::Args<'_>) -> StreamResult<()> {
        let data: u8 = match self {
            true => 1,
            false => 0,
        };
        data.write_no_opts(writer)
            .add_context(|| "write u8 for bool")
    }
}

#[derive(Debug, Copy, Clone)]
pub enum VecArgsLength {
    Fixed(usize),
    I32,
    I16,
    I8,
    VariableLengthInt,
}

#[derive(Debug, Copy, Clone)]
pub struct VecArgs<T> {
    pub length: VecArgsLength,
    pub item: T,
}

impl VecArgs<()> {
    pub fn new_empty(length: VecArgsLength) -> VecArgs<()> {
        VecArgs { length, item: () }
    }
    pub fn new_empty_fixed(length: usize) -> VecArgs<()> {
        Self::new_empty(VecArgsLength::Fixed(length))
    }
    pub fn new_empty_i32() -> VecArgs<()> {
        Self::new_empty(VecArgsLength::I32)
    }
    pub fn new_empty_i16() -> VecArgs<()> {
        Self::new_empty(VecArgsLength::I16)
    }
    pub fn new_empty_i8() -> VecArgs<()> {
        Self::new_empty(VecArgsLength::I8)
    }
    pub fn new_empty_variable() -> VecArgs<()> {
        Self::new_empty(VecArgsLength::VariableLengthInt)
    }
}

impl Readable for String {
    type Args<'a> = VecArgsLength;

    fn read<R: Read + Seek>(reader: &mut R, args: Self::Args<'_>) -> StreamResult<Self> {
        let pos = reader.stream_position()?;
        let data =
            <Vec<u8>>::read(reader, VecArgs::new_empty(args)).add_context(|| "read string data")?;

        String::from_utf8(data)
            .map_err(|e| StreamError::new(std::io::Error::other(e), pos))
            .add_context(|| "decode string data")
    }
}

impl Writable for String {
    type Args<'a> = VecArgsLength;
    fn write<W: Write + Seek>(&self, writer: &mut W, args: Self::Args<'_>) -> StreamResult<()> {
        let data = self.as_bytes();

        data.write(writer, VecArgs::new_empty(args))
            .add_context(|| "write string data")
    }
}

impl VecArgsLength {
    pub fn write<W: Write + Seek>(&self, writer: &mut W, length: usize) -> StreamResult<usize> {
        Ok(match self {
            VecArgsLength::Fixed(fix) => *fix,
            VecArgsLength::I32 => {
                (length as i32).write_no_opts(writer)?;
                length
            }
            VecArgsLength::I16 => {
                (length as i16).write_no_opts(writer)?;
                length
            }
            VecArgsLength::I8 => {
                (length as i8).write_no_opts(writer)?;
                length
            }
            VecArgsLength::VariableLengthInt => {
                VariableLengthInt(length as u32).write_no_opts(writer)?;
                length
            }
        })
    }
    pub fn read<R: Read + Seek>(&self, reader: &mut R) -> StreamResult<usize> {
        Ok(match self {
            VecArgsLength::Fixed(s) => *s,
            VecArgsLength::I32 => {
                i32::read_no_opts(reader).add_context(|| "read i32 for vec length")? as usize
            }
            VecArgsLength::I16 => {
                i16::read_no_opts(reader).add_context(|| "read i16 for vec length")? as usize
            }
            VecArgsLength::I8 => {
                i8::read_no_opts(reader).add_context(|| "read i8 for vec length")? as usize
            }
            VecArgsLength::VariableLengthInt => VariableLengthInt::read_no_opts(reader)?.0 as usize,
        })
    }
}

impl<T: for<'a> Readable<Args<'a>: Clone> + std::fmt::Debug> Readable for Vec<T> {
    type Args<'a> = VecArgs<T::Args<'a>>;

    fn read<R: Read + Seek>(reader: &mut R, args: Self::Args<'_>) -> StreamResult<Self> {
        let length = args.length.read(reader)?;
        let mut result = Vec::with_capacity(length);

        for i in 0..length {
            result.push(T::read(reader, args.item.clone()).add_context(|| {
                format!(
                    "read {i}/{length} {} for {}",
                    type_name::<T>(),
                    type_name::<Self>()
                )
            })?);
        }

        Ok(result)
    }
}

impl<T: for<'a> Writable<Args<'a>: Clone> + std::fmt::Debug + Default> Writable for Vec<T> {
    type Args<'a> = VecArgs<T::Args<'a>>;

    fn write<W: Write + Seek>(&self, writer: &mut W, args: Self::Args<'_>) -> StreamResult<()> {
        self.as_slice().write(writer, args)
    }
}
impl<T: for<'a> Writable<Args<'a>: Clone> + Default + std::fmt::Debug> Writable for &[T] {
    type Args<'a> = VecArgs<T::Args<'a>>;

    fn write<W: Write + Seek>(&self, writer: &mut W, args: Self::Args<'_>) -> StreamResult<()> {
        let fixed_length = args.length.write(writer, self.len())?;

        let length = self.len();

        for (i, item) in self.iter().enumerate().take(fixed_length) {
            item.write(writer, args.item.clone()).add_context(|| {
                format!(
                    "write {i}/{fixed_length} {} for {}",
                    type_name::<T>(),
                    type_name::<Self>()
                )
            })?;
        }

        if fixed_length > length {
            for i in 0..(fixed_length - length) {
                T::default()
                    .write(writer, args.item.clone())
                    .add_context(|| {
                        format!(
                            "write {}/{fixed_length} {} for {}",
                            i + length,
                            type_name::<T>(),
                            type_name::<Self>()
                        )
                    })?;
            }
        }
        Ok(())
    }
}

impl<T: for<'a> Readable<Args<'a>: Clone> + std::fmt::Debug, const N: usize> Readable for [T; N] {
    type Args<'a> = T::Args<'a>;

    fn read<R: Read + Seek>(reader: &mut R, args: Self::Args<'_>) -> StreamResult<Self> {
        let mut result = Vec::with_capacity(N);

        for i in 0..N {
            result.push(T::read(reader, args.clone()).add_context(|| {
                format!(
                    "read {i}/{N} {} for {}",
                    type_name::<T>(),
                    type_name::<Self>()
                )
            })?);
        }

        Ok(result.try_into().expect("read N things"))
    }
}

impl<T: for<'a> Writable<Args<'a>: Clone> + std::fmt::Debug, const N: usize> Writable for [T; N] {
    type Args<'a> = T::Args<'a>;

    fn write<W: Write + Seek>(&self, writer: &mut W, args: Self::Args<'_>) -> StreamResult<()> {
        for (i, item) in self.iter().enumerate() {
            item.write(writer, args.clone()).add_context(|| {
                format!(
                    "write {i}/{N} {} for {}",
                    type_name::<T>(),
                    type_name::<Self>()
                )
            })?;
        }
        Ok(())
    }
}

pub trait FromUsize {
    fn from_usize(val: usize) -> Self;
}

pub trait ToUsize {
    fn to_usize(self) -> usize;
}

macro_rules! impl_to_usize {
    ($type:ty) => {
        impl ToUsize for $type {
            fn to_usize(self) -> usize {
                self as usize
            }
        }
    };
}
macro_rules! impl_from_usize {
    ($type:ty) => {
        impl FromUsize for $type {
            fn from_usize(val: usize) -> Self {
                val as Self
            }
        }
    };
}

impl_to_usize!(u8);
impl_to_usize!(u16);
impl_to_usize!(u32);
impl_to_usize!(u64);
impl_to_usize!(i8);
impl_to_usize!(i16);
impl_to_usize!(i32);
impl_to_usize!(i64);

impl_from_usize!(u8);
impl_from_usize!(u16);
impl_from_usize!(u32);
impl_from_usize!(u64);
impl_from_usize!(i8);
impl_from_usize!(i16);
impl_from_usize!(i32);
impl_from_usize!(i64);

#[derive(Debug, Clone, Default)]
pub struct LengthVec<L, T>(pub Vec<T>, PhantomData<L>);

impl<
    L: for<'a> Readable<Args<'a> = ()> + ToUsize,
    T: for<'a> Readable<Args<'a> = ()> + std::fmt::Debug,
> Readable for LengthVec<L, T>
{
    type Args<'a> = ();
    fn read<R: Read + Seek>(reader: &mut R, _args: Self::Args<'_>) -> StreamResult<Self> {
        let length = L::read_no_opts(reader)?;
        Ok(Self(
            <Vec<T>>::read(reader, VecArgs::new_empty_fixed(length.to_usize()))?,
            PhantomData,
        ))
    }
}

impl<
    L: for<'a> Writable<Args<'a> = ()> + FromUsize,
    T: for<'a> Writable<Args<'a> = ()> + std::fmt::Debug + Default,
> Writable for LengthVec<L, T>
{
    type Args<'a> = ();
    fn write<W: Write + Seek>(&self, writer: &mut W, _args: Self::Args<'_>) -> StreamResult<()> {
        let length = L::from_usize(self.0.len());

        length.write_no_opts(writer)?;

        self.0
            .write(writer, VecArgs::new_empty_fixed(self.0.len()))?;
        Ok(())
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Default)]
pub struct LengthString<L>(pub String, PhantomData<L>);

impl<L> Display for LengthString<L> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<L: for<'a> Readable<Args<'a> = ()> + ToUsize> Readable for LengthString<L> {
    type Args<'a> = ();
    fn read<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        _args: Self::Args<'_>,
    ) -> crate::stream::StreamResult<Self> {
        let length = L::read_no_opts(reader)?;
        Ok(Self(
            String::read(reader, VecArgsLength::Fixed(length.to_usize()))
                .add_context(|| "read string data")?,
            PhantomData,
        ))
    }
}

impl<L: for<'a> Writable<Args<'a> = ()> + FromUsize> Writable for LengthString<L> {
    type Args<'a> = ();
    fn write<W: Write + Seek>(&self, writer: &mut W, _args: Self::Args<'_>) -> StreamResult<()> {
        let length = L::from_usize(self.0.len());

        length.write_no_opts(writer)?;

        self.0.write(writer, VecArgsLength::Fixed(self.0.len()))
    }
}

macro_rules! impl_read_tuple {
    ($($type:ident),+) => {
        impl<$($type: ReadableNoOptions),+> Readable
            for ($($type),+)
        {
            type Args<'a> = ();
            fn read<R: Read + Seek>(reader: &mut R, _args: Self::Args<'_>) -> StreamResult<Self> {
                Ok((
                    $(
                        $type::read_no_opts(reader).add_context(|| "read")?,
                    )+
                ))
            }
        }
    };
}

macro_rules! impl_write_tuple {
    ($($type:ident => $num:tt),+) => {
        impl<$($type: WritableNoOptions),+> Writable
            for ($($type),+)
        {
            type Args<'a> = ();
            fn write<W: Write + Seek>(&self, writer: &mut W, _args: Self::Args<'_>) -> StreamResult<()> {
                $(
                    self.$num.write_no_opts(writer).add_context(|| format!("write {}", $num))?;
                )+
                Ok(())
            }
        }
    };
}

impl_read_tuple!(T1, T2);
impl_read_tuple!(T1, T2, T3);
impl_read_tuple!(T1, T2, T3, T4);
impl_read_tuple!(T1, T2, T3, T4, T5);
impl_read_tuple!(T1, T2, T3, T4, T5, T6);

impl_write_tuple!(T1 => 0, T2 => 1);
impl_write_tuple!(T1 => 0, T2 => 1, T3 => 2);
impl_write_tuple!(T1 => 0, T2 => 1, T3 => 2, T4 => 3);
impl_write_tuple!(T1 => 0, T2 => 1, T3 => 2, T4 => 3, T5 => 4);
impl_write_tuple!(T1 => 0, T2 => 1, T3 => 2, T4 => 3, T5 => 4, T6 => 5);

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq, Default)]
pub struct VariableLengthInt(pub u32);

impl ToUsize for VariableLengthInt {
    fn to_usize(self) -> usize {
        self.0 as usize
    }
}

impl FromUsize for VariableLengthInt {
    fn from_usize(val: usize) -> Self {
        Self(val as u32)
    }
}

impl Display for VariableLengthInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Readable for VariableLengthInt {
    type Args<'a> = ();
    fn read<R: Read + Seek>(reader: &mut R, _args: Self::Args<'_>) -> StreamResult<Self> {
        let mut val: u32 = 0;

        for i in 0..4 {
            let new_val = val << 7;
            let raw_value = u8::read_no_opts(reader)
                .add_context(|| format!("read variable length int value: {i}"))?;

            val = new_val | (raw_value & 0x7f) as u32;

            if raw_value & 0x80 == 0 {
                return Ok(Self(val));
            }
        }

        Ok(Self(val))
    }
}

impl Writable for VariableLengthInt {
    type Args<'a> = ();
    fn write<W: Write + Seek>(&self, writer: &mut W, _args: Self::Args<'_>) -> StreamResult<()> {
        let mut value = self.0;
        let mut i2 = 0;
        let mut i = 0;
        while value >= 128 {
            i2 |= ((value & 0x7f) | 0x8000) << (i * 8);
            value >>= 7;
            i += 1;
        }

        let i4 = i2 | (value << (i * 8));
        let i5 = i + 1;

        for i in 0..i5 {
            let byte = (i4 >> (((i5 - i) - 1) * 8)) as u8;

            byte.write_no_opts(writer)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct HashMapLength<L, K, V>(pub HashMap<K, V>, PhantomData<L>);

impl<L, K, V> HashMapLength<L, K, V> {
    pub fn new(map: HashMap<K, V>) -> Self {
        Self(map, PhantomData)
    }
}

impl<
    L: for<'a> Readable<Args<'a> = ()> + ToUsize,
    K: for<'a> Readable<Args<'a> = ()> + Hash + Eq + Display,
    V: for<'a> Readable<Args<'a> = ()>,
> Readable for HashMapLength<L, K, V>
{
    type Args<'a> = ();
    fn read<R: Read + Seek>(reader: &mut R, _args: Self::Args<'_>) -> StreamResult<Self> {
        let length = L::read_no_opts(reader)?;
        Ok(Self(
            HashMap::read(reader, VecArgsLength::Fixed(length.to_usize()))?,
            PhantomData,
        ))
    }
}

impl<
    L: for<'a> Writable<Args<'a> = ()> + FromUsize,
    K: for<'a> Writable<Args<'a> = ()> + std::fmt::Display + Default,
    V: for<'a> Writable<Args<'a> = ()> + std::fmt::Debug + Default,
> Writable for HashMapLength<L, K, V>
{
    type Args<'a> = ();
    fn write<W: Write + Seek>(&self, writer: &mut W, _args: Self::Args<'_>) -> StreamResult<()> {
        let length = L::from_usize(self.0.len());

        length.write_no_opts(writer)?;

        self.0.write(writer, VecArgsLength::Fixed(self.0.len()))
    }
}

impl<K: for<'a> Readable<Args<'a> = ()> + Hash + Eq + Display, V: for<'a> Readable<Args<'a> = ()>>
    Readable for HashMap<K, V>
{
    type Args<'a> = VecArgsLength;
    fn read<R: Read + Seek>(reader: &mut R, args: Self::Args<'_>) -> StreamResult<Self> {
        let length = args.read(reader)?;

        let mut map = HashMap::with_capacity(length);

        for i in 0..length {
            let pos = reader.stream_position()?;
            let key = K::read(reader, ())
                .add_context(|| format!("read key for hashmap: {i}/{length}"))?;

            if map.contains_key(&key) {
                return Err(StreamError::new(
                    std::io::Error::other(format!("key: {key} already exists in hashmap!")),
                    pos,
                ));
            }
            let value = V::read(reader, ())
                .add_context(|| format!("read value for hashmap: {i}/{length}"))?;

            map.insert(key, value);
        }

        Ok(map)
    }
}

impl<
    K: for<'a> Writable<Args<'a> = ()> + std::fmt::Display + Default,
    V: for<'a> Writable<Args<'a> = ()> + std::fmt::Debug + Default,
> Writable for HashMap<K, V>
{
    type Args<'a> = VecArgsLength;

    fn write<W: Write + Seek>(&self, writer: &mut W, args: Self::Args<'_>) -> StreamResult<()> {
        let fixed_length = args.write(writer, self.len())?;

        let length = self.len();

        for (k, v) in self.iter().take(fixed_length) {
            k.write_no_opts(writer)
                .add_context(|| format!("writing key: {k} for hashmap"))?;
            v.write_no_opts(writer)
                .add_context(|| format!("writing value: {v:?} for hashmap"))?;
        }

        if fixed_length > length {
            for _ in 0..(fixed_length - length) {
                K::default()
                    .write_no_opts(writer)
                    .add_context(|| "write extra key for hashmap")?;
                V::default()
                    .write_no_opts(writer)
                    .add_context(|| "write extra key for hashmap")?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Assertable<const EX: i32>;

impl<const EX: i32> Readable for Assertable<EX> {
    type Args<'a> = ();
    fn read<R: Read + Seek>(reader: &mut R, _args: Self::Args<'_>) -> StreamResult<Self> {
        let pos = reader.stream_position()?;
        let value = i32::read_no_opts(reader).add_context(|| "read i32 for assertable")?;

        if value != EX {
            Err(StreamError::new(
                std::io::Error::other(format!("assertion error. expected: {EX}, got {value}")),
                pos,
            ))
        } else {
            Ok(Self)
        }
    }
}

impl<const EX: i32> Writable for Assertable<EX> {
    type Args<'a> = ();
    fn write<W: Write + Seek>(&self, writer: &mut W, _args: Self::Args<'_>) -> StreamResult<()> {
        EX.write_no_opts(writer)
    }
}
