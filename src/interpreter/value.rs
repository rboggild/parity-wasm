use std::{i32, i64, u32, u64, f32};
use std::io;
use std::mem;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use interpreter::Error;
use interpreter::variable::VariableType;

/// Runtime value.
#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeValue {
	/// Null value.
	Null,
	/// Reference to the function in the same module.
	AnyFunc(u32),
	/// 32b-length signed/unsigned int.
	I32(i32),
	/// 64b-length signed/unsigned int.
	I64(i64),
	/// 32b-length float.
	F32(f32),
	/// 64b-length float.
	F64(f64),
}

/// Try to convert into trait.
pub trait TryInto<T, E> {
	/// Try to convert self into other value.
	fn try_into(self) -> Result<T, E>;
}

/// Convert one type to another by wrapping.
pub trait WrapInto<T> {
	/// Convert one type to another by wrapping.
	fn wrap_into(self) -> T;
}

/// Convert one type to another by rounding to the nearest integer towards zero.
pub trait TryTruncateInto<T, E> {
	/// Convert one type to another by rounding to the nearest integer towards zero.
	fn try_truncate_into(self) -> Result<T, E>;
}

/// Convert one type to another by extending with leading zeroes.
pub trait ExtendInto<T> {
	/// Convert one type to another by extending with leading zeroes.
	fn extend_into(self) -> T;
}

/// Reinterprets the bits of a value of one type as another type.
pub trait TransmuteInto<T> {
	/// Reinterprets the bits of a value of one type as another type.
	fn transmute_into(self) -> T;
}

/// Convert from and to little endian.
pub trait LittleEndianConvert where Self: Sized {
	/// Convert to little endian bufer.
	fn into_little_endian(self) -> Vec<u8>;
	/// Convert from little endian bufer.
	fn from_little_endian(buffer: Vec<u8>) -> Result<Self, Error>;
}

/// Arithmetic operations.
pub trait ArithmeticOps<T> {
	/// Add two values.
	fn add(self, other: T) -> T;
	/// Subtract two values.
	fn sub(self, other: T) -> T;
	/// Multiply two values.
	fn mul(self, other: T) -> T;
	/// Divide two values.
	fn div(self, other: T) -> T;
}

/// Integer value.
pub trait Integer<T>: ArithmeticOps<T> {
	/// Counts leading zeros in the bitwise representation of the value.
	fn leading_zeros(self) -> T;
	/// Counts trailing zeros in the bitwise representation of the value.
	fn trailing_zeros(self) -> T;
	/// Counts 1-bits in the bitwise representation of the value.
	fn count_ones(self) -> T;
	/// Get left bit rotation result.
	fn rotl(self, other: T) -> T;
	/// Get right bit rotation result.
	fn rotr(self, other: T) -> T;
	/// Get division remainder.
	fn rem(self, other: T) -> T;
}

/// Float-point value.
pub trait Float<T>: ArithmeticOps<T> {
	/// Get absolute value.
	fn abs(self) -> T;
	/// Returns the largest integer less than or equal to a number.
	fn floor(self) -> T;
	/// Returns the smallest integer greater than or equal to a number.
	fn ceil(self) -> T;
	/// Returns the integer part of a number.
	fn trunc(self) -> T;
	/// Returns the nearest integer to a number. Round half-way cases away from 0.0.
	fn round(self) -> T;
	/// Takes the square root of a number.
	fn sqrt(self) -> T;
	/// Returns the minimum of the two numbers.
	fn min(self, other: T) -> T;
	/// Returns the maximum of the two numbers.
	fn max(self, other: T) -> T;
	/// Sets sign of this value to the sign of other value.
	fn copysign(self, other: T) -> T;
}

impl RuntimeValue {
	/// Creates new default value of given type.
	pub fn default(variable_type: VariableType) -> Self {
		match variable_type {
			VariableType::AnyFunc => RuntimeValue::AnyFunc(0),
			VariableType::I32 => RuntimeValue::I32(0),
			VariableType::I64 => RuntimeValue::I64(0),
			VariableType::F32 => RuntimeValue::F32(0f32),
			VariableType::F64 => RuntimeValue::F64(0f64),
		}
	}

	/// Creates new value by interpreting passed u32 as f32.
	pub fn decode_f32(val: u32) -> Self {
		RuntimeValue::F32(val.transmute_into())
	}

	/// Creates new value by interpreting passed u64 as f64.
	pub fn decode_f64(val: u64) -> Self {
		RuntimeValue::F64(val.transmute_into())
	}

	/// Returns true if value is null.
	pub fn is_null(&self) -> bool {
		match *self {
			RuntimeValue::Null => true,
			_ => false,
		}
	}

	/// Gets function index, if type of value is AnyFunc.
	pub fn as_any_func_index(&self) -> Option<u32> {
		match *self {
			RuntimeValue::AnyFunc(idx) => Some(idx),
			_ => None,
		}
	}

	/// Get variable type for this value.
	pub fn variable_type(&self) -> Option<VariableType> {
		match *self {
			RuntimeValue::Null => None,
			RuntimeValue::AnyFunc(_) => Some(VariableType::AnyFunc),
			RuntimeValue::I32(_) => Some(VariableType::I32),
			RuntimeValue::I64(_) => Some(VariableType::I64),
			RuntimeValue::F32(_) => Some(VariableType::F32),
			RuntimeValue::F64(_) => Some(VariableType::F64),
		}
	}
}

impl From<i32> for RuntimeValue {
	fn from(val: i32) -> Self {
		RuntimeValue::I32(val)
	}
}

impl From<i64> for RuntimeValue {
	fn from(val: i64) -> Self {
		RuntimeValue::I64(val)
	}
}

impl From<f32> for RuntimeValue {
	fn from(val: f32) -> Self {
		RuntimeValue::F32(val)
	}
}

impl From<f64> for RuntimeValue {
	fn from(val: f64) -> Self {
		RuntimeValue::F64(val)
	}
}

impl TryInto<bool, Error> for RuntimeValue {
	fn try_into(self) -> Result<bool, Error> {
		match self {
			RuntimeValue::I32(val) => Ok(val != 0),
			_ => Err(Error::Value(format!("32-bit int value expected"))),
		}
	}
}

impl TryInto<i32, Error> for RuntimeValue {
	fn try_into(self) -> Result<i32, Error> {
		match self {
			RuntimeValue::I32(val) => Ok(val),
			_ => Err(Error::Value(format!("32-bit int value expected"))),
		}
	}
}

impl TryInto<i64, Error> for RuntimeValue {
	fn try_into(self) -> Result<i64, Error> {
		match self {
			RuntimeValue::I64(val) => Ok(val),
			_ => Err(Error::Value(format!("64-bit int value expected"))),
		}
	}
}

impl TryInto<f32, Error> for RuntimeValue {
	fn try_into(self) -> Result<f32, Error> {
		match self {
			RuntimeValue::F32(val) => Ok(val),
			_ => Err(Error::Value(format!("32-bit float value expected"))),
		}
	}
}

impl TryInto<f64, Error> for RuntimeValue {
	fn try_into(self) -> Result<f64, Error> {
		match self {
			//RuntimeValue::F32(val) => Some(val as f64),
			RuntimeValue::F64(val) => Ok(val),
			_ => Err(Error::Value(format!("64-bit float value expected"))),
		}
	}
}

impl TryInto<u32, Error> for RuntimeValue {
	fn try_into(self) -> Result<u32, Error> {
		match self {
			RuntimeValue::I32(val) => Ok(unsafe {
				mem::transmute(val)
			}),
			_ => Err(Error::Value(format!("32-bit int value expected"))),
		}
	}
}

impl TryInto<u64, Error> for RuntimeValue {
	fn try_into(self) -> Result<u64, Error> {
		match self {
			RuntimeValue::I64(val) => Ok(unsafe {
				mem::transmute(val)
			}),
			_ => Err(Error::Value(format!("64-bit int value expected"))),
		}
	}
}

macro_rules! impl_wrap_into {
	($from: ident, $into: ident) => {
		impl WrapInto<$into> for $from {
			fn wrap_into(self) -> $into {
				self as $into
			}
		}
	}
}

impl_wrap_into!(i32, i8);
impl_wrap_into!(i32, i16);
impl_wrap_into!(i64, i8);
impl_wrap_into!(i64, i16);
impl_wrap_into!(i64, i32);
impl_wrap_into!(i64, f32);
impl_wrap_into!(u64, f32);
// Casting from an f64 to an f32 will produce the closest possible value (rounding strategy unspecified)
// NOTE: currently this will cause Undefined Behavior if the value is finite but larger or smaller than the
// largest or smallest finite value representable by f32. This is a bug and will be fixed.
impl_wrap_into!(f64, f32);

macro_rules! impl_try_truncate_into {
	($from: ident, $into: ident) => {
		impl TryTruncateInto<$into, Error> for $from {
			fn try_truncate_into(self) -> Result<$into, Error> {
				if !self.is_normal() {
					return Err(Error::Value("invalid float value for this operation".into()));
				}

				let truncated = self.trunc();
				if truncated < $into::MIN as $from || truncated > $into::MAX as $from {
					return Err(Error::Value("invalid float value for this operation".into()));
				}

				Ok(truncated as $into)
			}
		}
	}
}

impl_try_truncate_into!(f32, i32);
impl_try_truncate_into!(f32, i64);
impl_try_truncate_into!(f64, i32);
impl_try_truncate_into!(f64, i64);
impl_try_truncate_into!(f32, u32);
impl_try_truncate_into!(f32, u64);
impl_try_truncate_into!(f64, u32);
impl_try_truncate_into!(f64, u64);

macro_rules! impl_extend_into {
	($from: ident, $into: ident) => {
		impl ExtendInto<$into> for $from {
			fn extend_into(self) -> $into {
				self as $into
			}
		}
	}
}

impl_extend_into!(i8, i32);
impl_extend_into!(u8, i32);
impl_extend_into!(i16, i32);
impl_extend_into!(u16, i32);
impl_extend_into!(i8, i64);
impl_extend_into!(u8, i64);
impl_extend_into!(i16, i64);
impl_extend_into!(u16, i64);
impl_extend_into!(i32, i64);
impl_extend_into!(u32, i64);
impl_extend_into!(u32, u64);
impl_extend_into!(i32, f32);
impl_extend_into!(i32, f64);
impl_extend_into!(u32, f32);
impl_extend_into!(u32, f64);
impl_extend_into!(i64, f64);
impl_extend_into!(u64, f64);
impl_extend_into!(f32, f64);

macro_rules! impl_transmute_into_self {
	($type: ident) => {
		impl TransmuteInto<$type> for $type {
			fn transmute_into(self) -> $type {
				self
			}
		}
	}
}

impl_transmute_into_self!(i32);
impl_transmute_into_self!(i64);
impl_transmute_into_self!(f32);
impl_transmute_into_self!(f64);

macro_rules! impl_transmute_into {
	($from: ident, $into: ident) => {
		impl TransmuteInto<$into> for $from {
			fn transmute_into(self) -> $into {
				unsafe {
					mem::transmute(self)
				}
			}
		}
	}
}

impl_transmute_into!(i8, u8);
impl_transmute_into!(u8, i8);
impl_transmute_into!(i32, u32);
impl_transmute_into!(u32, i32);
impl_transmute_into!(u32, f32);
impl_transmute_into!(i32, f32);
impl_transmute_into!(f32, i32);
impl_transmute_into!(i64, u64);
impl_transmute_into!(u64, i64);
impl_transmute_into!(u64, f64);
impl_transmute_into!(i64, f64);
impl_transmute_into!(f64, i64);

impl LittleEndianConvert for i8 {
	fn into_little_endian(self) -> Vec<u8> {
		vec![self.transmute_into()]
	}

	fn from_little_endian(buffer: Vec<u8>) -> Result<Self, Error> {
		buffer.get(0)
			.map(|v| v.transmute_into())
			.ok_or(Error::Value("invalid little endian buffer".into()))
	}
}

impl LittleEndianConvert for u8 {
	fn into_little_endian(self) -> Vec<u8> {
		vec![self]
	}

	fn from_little_endian(buffer: Vec<u8>) -> Result<Self, Error> {
		buffer.get(0)
			.cloned()
			.ok_or(Error::Value("invalid little endian buffer".into()))
	}
}

impl LittleEndianConvert for i16 {
	fn into_little_endian(self) -> Vec<u8> {
		let mut vec = Vec::with_capacity(2);
		vec.write_i16::<LittleEndian>(self)
			.expect("i16 is written without any errors");
		vec
	}

	fn from_little_endian(buffer: Vec<u8>) -> Result<Self, Error> {
		io::Cursor::new(buffer).read_i16::<LittleEndian>()
			.map_err(|e| Error::Value(e.to_string()))
	}
}

impl LittleEndianConvert for u16 {
	fn into_little_endian(self) -> Vec<u8> {
		let mut vec = Vec::with_capacity(2);
		vec.write_u16::<LittleEndian>(self)
			.expect("u16 is written without any errors");
		vec
	}

	fn from_little_endian(buffer: Vec<u8>) -> Result<Self, Error> {
		io::Cursor::new(buffer).read_u16::<LittleEndian>()
			.map_err(|e| Error::Value(e.to_string()))
	}
}

impl LittleEndianConvert for i32 {
	fn into_little_endian(self) -> Vec<u8> {
		let mut vec = Vec::with_capacity(4);
		vec.write_i32::<LittleEndian>(self)
			.expect("i32 is written without any errors");
		vec
	}

	fn from_little_endian(buffer: Vec<u8>) -> Result<Self, Error> {
		io::Cursor::new(buffer).read_i32::<LittleEndian>()
			.map_err(|e| Error::Value(e.to_string()))
	}
}

impl LittleEndianConvert for u32 {
	fn into_little_endian(self) -> Vec<u8> {
		let mut vec = Vec::with_capacity(4);
		vec.write_u32::<LittleEndian>(self)
			.expect("u32 is written without any errors");
		vec
	}

	fn from_little_endian(buffer: Vec<u8>) -> Result<Self, Error> {
		io::Cursor::new(buffer).read_u32::<LittleEndian>()
			.map_err(|e| Error::Value(e.to_string()))
	}
}

impl LittleEndianConvert for i64 {
	fn into_little_endian(self) -> Vec<u8> {
		let mut vec = Vec::with_capacity(8);
		vec.write_i64::<LittleEndian>(self)
			.expect("i64 is written without any errors");
		vec
	}

	fn from_little_endian(buffer: Vec<u8>) -> Result<Self, Error> {
		io::Cursor::new(buffer).read_i64::<LittleEndian>()
			.map_err(|e| Error::Value(e.to_string()))
	}
}

impl LittleEndianConvert for f32 {
	fn into_little_endian(self) -> Vec<u8> {
		let mut vec = Vec::with_capacity(4);
		vec.write_i32::<LittleEndian>(self.transmute_into())
			.expect("i32 is written without any errors");
		vec
	}

	fn from_little_endian(buffer: Vec<u8>) -> Result<Self, Error> {
		io::Cursor::new(buffer).read_i32::<LittleEndian>()
			.map(TransmuteInto::transmute_into)
			.map_err(|e| Error::Value(e.to_string()))
	}
}

impl LittleEndianConvert for f64 {
	fn into_little_endian(self) -> Vec<u8> {
		let mut vec = Vec::with_capacity(8);
		vec.write_i64::<LittleEndian>(self.transmute_into())
			.expect("i64 is written without any errors");
		vec
	}

	fn from_little_endian(buffer: Vec<u8>) -> Result<Self, Error> {
		io::Cursor::new(buffer).read_i64::<LittleEndian>()
			.map(TransmuteInto::transmute_into)
			.map_err(|e| Error::Value(e.to_string()))
	}
}

macro_rules! impl_integer_arithmetic_ops {
	($type: ident) => {
		impl ArithmeticOps<$type> for $type {
			fn add(self, other: $type) -> $type { self.wrapping_add(other) }
			fn sub(self, other: $type) -> $type { self.wrapping_sub(other) }
			fn mul(self, other: $type) -> $type { self.wrapping_mul(other) }
			fn div(self, other: $type) -> $type { self.wrapping_div(other) }
		}
	}
}

impl_integer_arithmetic_ops!(i32);
impl_integer_arithmetic_ops!(u32);
impl_integer_arithmetic_ops!(i64);
impl_integer_arithmetic_ops!(u64);

macro_rules! impl_float_arithmetic_ops {
	($type: ident) => {
		impl ArithmeticOps<$type> for $type {
			fn add(self, other: $type) -> $type { self + other }
			fn sub(self, other: $type) -> $type { self - other }
			fn mul(self, other: $type) -> $type { self * other }
			fn div(self, other: $type) -> $type { self / other }
		}
	}
}

impl_float_arithmetic_ops!(f32);
impl_float_arithmetic_ops!(f64);

macro_rules! impl_integer {
	($type: ident) => {
		impl Integer<$type> for $type {
			fn leading_zeros(self) -> $type { self.leading_zeros() as $type }
			fn trailing_zeros(self) -> $type { self.trailing_zeros() as $type }
			fn count_ones(self) -> $type { self.count_ones() as $type }
			fn rotl(self, other: $type) -> $type { self.rotate_left(other as u32) }
			fn rotr(self, other: $type) -> $type { self.rotate_right(other as u32) }
			fn rem(self, other: $type) -> $type { self.wrapping_rem(other) }
		}
	}
}

impl_integer!(i32);
impl_integer!(u32);
impl_integer!(i64);
impl_integer!(u64);

macro_rules! impl_float {
	($type: ident) => {
		impl Float<$type> for $type {
			fn abs(self) -> $type { self.abs() }
			fn floor(self) -> $type { self.floor() }
			fn ceil(self) -> $type { self.ceil() }
			fn trunc(self) -> $type { self.trunc() }
			fn round(self) -> $type { self.round() }
			fn sqrt(self) -> $type { self.sqrt() }
			// TODO
			// This instruction corresponds to what is sometimes called "minNaN" in other languages.
			// This differs from the IEEE 754-2008 minNum operation in that it returns a NaN if either operand is a NaN, and in that the behavior when the operands are zeros of differing signs is fully specified.
			// This differs from the common x<y?x:y expansion in its handling of negative zero and NaN values.
			fn min(self, other: $type) -> $type { self.min(other) }
			// TODO
			// This instruction corresponds to what is sometimes called "maxNaN" in other languages.
			// This differs from the IEEE 754-2008 maxNum operation in that it returns a NaN if either operand is a NaN, and in that the behavior when the operands are zeros of differing signs is fully specified.
			// This differs from the common x>y?x:y expansion in its handling of negative zero and NaN values.
			fn max(self, other: $type) -> $type { self.max(other) }
			fn copysign(self, other: $type) -> $type {
				// TODO: this may be buggy for edge cases
				if self.is_sign_positive() == other.is_sign_positive() {
					self
				} else {
					self * -1.0
				}
			}
		}
	}
}

impl_float!(f32);
impl_float!(f64);
