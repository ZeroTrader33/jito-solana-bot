///! 128 and 256 bit numbers
///! U128 is more efficient that u128
///! https://github.com/solana-labs/solana/issues/19549
use uint::construct_uint;
construct_uint! {
    pub struct U128(2);
}

construct_uint! {
    pub struct U256(4);
}

construct_uint! {
    pub struct U512(8);
}

#[macro_export]
macro_rules! construct_bignum {
    ( $(#[$attr:meta])* $visibility:vis struct $name:ident ( $n_words:tt ); ) => {
        construct_bignum! { @construct $(#[$attr])* $visibility struct $name ($n_words); }
        impl uint::core_::convert::From<u128> for $name {
            fn from(value: u128) -> $name {
                let mut ret = [0; $n_words];
                ret[0] = value as u64;
                ret[1] = (value >> 64) as u64;
                $name(ret)
            }
        }

        impl uint::core_::convert::From<i128> for $name {
            fn from(value: i128) -> $name {
                match value >= 0 {
                    true => From::from(value as u128),
                    false => { panic!("Unsigned integer can't be created from negative value"); }
                }
            }
        }

        impl $name {
            /// Low 2 words (u128)
            #[inline]
            pub const fn low_u128(&self) -> u128 {
                let &$name(ref arr) = self;
                ((arr[1] as u128) << 64) + arr[0] as u128
            }

            /// Conversion to u128 with overflow checking
            ///
            /// # Panics
            ///
            /// Panics if the number is larger than 2^128.
            #[inline]
            pub fn as_u128(&self) -> u128 {
                let &$name(ref arr) = self;
                for i in 2..$n_words {
                    if arr[i] != 0 {
                        panic!("Integer overflow when casting to u128")
                    }

                }
                self.low_u128()
            }
        }

        impl uint::core_::convert::TryFrom<$name> for u128 {
            type Error = &'static str;

            #[inline]
            fn try_from(u: $name) -> uint::core_::result::Result<u128, &'static str> {
                let $name(arr) = u;
                for i in 2..$n_words {
                    if arr[i] != 0 {
                        return Err("integer overflow when casting to u128");
                    }
                }
                Ok(((arr[1] as u128) << 64) + arr[0] as u128)
            }
        }

        impl uint::core_::convert::TryFrom<$name> for i128 {
            type Error = &'static str;

            #[inline]
            fn try_from(u: $name) -> uint::core_::result::Result<i128, &'static str> {
                let err_str = "integer overflow when casting to i128";
                let i = u128::try_from(u).map_err(|_| err_str)?;
                if i > i128::max_value() as u128 {
                    Err(err_str)
                } else {
                    Ok(i as i128)
                }
            }
        }
    };

    ( @construct $(#[$attr:meta])* $visibility:vis struct $name:ident ( $n_words:tt ); ) => {
		/// Little-endian large integer type
		#[repr(C)]
		$(#[$attr])*
		#[derive(Copy, Clone, Eq, PartialEq, Hash)]
		$visibility struct $name (pub [u64; $n_words]);

		/// Get a reference to the underlying little-endian words.
		impl AsRef<[u64]> for $name {
			#[inline]
			fn as_ref(&self) -> &[u64] {
				&self.0
			}
		}

		impl<'a> From<&'a $name> for $name {
			fn from(x: &'a $name) -> $name {
				*x
			}
		}

        impl $name {
			/// Maximum value.
			pub const MAX: $name = $name([u64::max_value(); $n_words]);

            /// Conversion to usize with overflow checking
			///
			/// # Panics
			///
			/// Panics if the number is larger than usize::max_value().
			#[inline]
			pub fn as_usize(&self) -> usize {
				let &$name(ref arr) = self;
				if !self.fits_word() || arr[0] > usize::max_value() as u64 {
					panic!("Integer overflow when casting to usize")
				}
				arr[0] as usize
			}

			/// Whether this is zero.
			#[inline]
			pub const fn is_zero(&self) -> bool {
				let &$name(ref arr) = self;
				let mut i = 0;
				while i < $n_words { if arr[i] != 0 { return false; } else { i += 1; } }
				return true;
			}

            // Whether this fits u64.
			#[inline]
			fn fits_word(&self) -> bool {
				let &$name(ref arr) = self;
				for i in 1..$n_words { if arr[i] != 0 { return false; } }
				return true;
			}

            /// Return if specific bit is set.
			///
			/// # Panics
			///
			/// Panics if `index` exceeds the bit width of the number.
			#[inline]
			pub const fn bit(&self, index: usize) -> bool {
				let &$name(ref arr) = self;
				arr[index / 64] & (1 << (index % 64)) != 0
			}

            /// Returns the number of leading zeros in the binary representation of self.
			pub fn leading_zeros(&self) -> u32 {
				let mut r = 0;
				for i in 0..$n_words {
					let w = self.0[$n_words - i - 1];
					if w == 0 {
						r += 64;
					} else {
						r += w.leading_zeros();
						break;
					}
				}
				r
			}

			/// Returns the number of trailing zeros in the binary representation of self.
			pub fn trailing_zeros(&self) -> u32 {
				let mut r = 0;
				for i in 0..$n_words {
					let w = self.0[i];
					if w == 0 {
						r += 64;
					} else {
						r += w.trailing_zeros();
						break;
					}
				}
				r
			}

            /// Zero (additive identity) of this type.
			#[inline]
			pub const fn zero() -> Self {
				Self([0; $n_words])
			}

			/// One (multiplicative identity) of this type.
			#[inline]
			pub const fn one() -> Self {
				let mut words = [0; $n_words];
				words[0] = 1u64;
				Self(words)
			}

			/// The maximum value which can be inhabited by this type.
			#[inline]
			pub const fn max_value() -> Self {
				Self::MAX
			}
        }

        impl uint::core_::default::Default for $name {
            fn default() -> Self {
                $name::zero()
            }
        }

        impl uint::core_::ops::BitAnd<$name> for $name {
            type Output = $name;

            #[inline]
            fn bitand(self, other: $name) -> $name {
                let $name(ref arr1) = self;
                let $name(ref arr2) = other;
                let mut ret = [0u64; $n_words];
                for i in 0..$n_words {
                    ret[i] = arr1[i] & arr2[i];
                }
                $name(ret)
            }
        }

        impl uint::core_::ops::BitOr<$name> for $name {
            type Output = $name;

            #[inline]
            fn bitor(self, other: $name) -> $name {
                let $name(ref arr1) = self;
                let $name(ref arr2) = other;
                let mut ret = [0u64; $n_words];
                for i in 0..$n_words {
                    ret[i] = arr1[i] | arr2[i];
                }
                $name(ret)
            }
        }

        impl uint::core_::ops::BitXor<$name> for $name {
            type Output = $name;

            #[inline]
            fn bitxor(self, other: $name) -> $name {
                let $name(ref arr1) = self;
                let $name(ref arr2) = other;
                let mut ret = [0u64; $n_words];
                for i in 0..$n_words {
                    ret[i] = arr1[i] ^ arr2[i];
                }
                $name(ret)
            }
        }

        impl uint::core_::ops::Not for $name {
            type Output = $name;

            #[inline]
            fn not(self) -> $name {
                let $name(ref arr) = self;
                let mut ret = [0u64; $n_words];
                for i in 0..$n_words {
                    ret[i] = !arr[i];
                }
                $name(ret)
            }
        }

        impl uint::core_::ops::Shl<usize> for $name {
            type Output = $name;

            fn shl(self, shift: usize) -> $name {
                let $name(ref original) = self;
                let mut ret = [0u64; $n_words];
                let word_shift = shift / 64;
                let bit_shift = shift % 64;

                // shift
                for i in word_shift..$n_words {
                    ret[i] = original[i - word_shift] << bit_shift;
                }
                // carry
                if bit_shift > 0 {
                    for i in word_shift+1..$n_words {
                        ret[i] += original[i - 1 - word_shift] >> (64 - bit_shift);
                    }
                }
                $name(ret)
            }
        }

        impl<'a> uint::core_::ops::Shl<usize> for &'a $name {
            type Output = $name;
            fn shl(self, shift: usize) -> $name {
                *self << shift
            }
        }

        impl uint::core_::ops::Shr<usize> for $name {
            type Output = $name;

            fn shr(self, shift: usize) -> $name {
                let $name(ref original) = self;
                let mut ret = [0u64; $n_words];
                let word_shift = shift / 64;
                let bit_shift = shift % 64;

                // shift
                for i in word_shift..$n_words {
                    ret[i - word_shift] = original[i] >> bit_shift;
                }

                // Carry
                if bit_shift > 0 {
                    for i in word_shift+1..$n_words {
                        ret[i - word_shift - 1] += original[i] << (64 - bit_shift);
                    }
                }

                $name(ret)
            }
        }

        impl<'a> uint::core_::ops::Shr<usize> for &'a $name {
            type Output = $name;
            fn shr(self, shift: usize) -> $name {
                *self >> shift
            }
        }
    };
}
construct_bignum! {
    pub struct U1024(16);
}

pub trait MulDiv<RHS = Self> {
    /// Output type for the methods of this trait.
    type Output;

    /// Calculates `floor(val * num / denom)`, i.e. the largest integer less than or equal to the
    /// result of the division.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use libraries::raydium_whirlpool::full_math::MulDiv;
    ///
    /// # fn main() {
    /// let x = 3i8.mul_div_floor(4, 2);
    /// assert_eq!(x, Some(6));
    ///
    /// let x = 5i8.mul_div_floor(2, 3);
    /// assert_eq!(x, Some(3));
    ///
    /// let x = (-5i8).mul_div_floor(2, 3);
    /// assert_eq!(x, Some(-4));
    ///
    /// let x = 3i8.mul_div_floor(3, 2);
    /// assert_eq!(x, Some(4));
    ///
    /// let x = (-3i8).mul_div_floor(3, 2);
    /// assert_eq!(x, Some(-5));
    ///
    /// let x = 127i8.mul_div_floor(4, 3);
    /// assert_eq!(x, None);
    /// # }
    /// ```
    fn mul_div_floor(self, num: RHS, denom: RHS) -> Option<Self::Output>;

    /// Calculates `ceil(val * num / denom)`, i.e. the the smallest integer greater than or equal to
    /// the result of the division.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use libraries::raydium_whirlpool::full_math::MulDiv;
    ///
    /// # fn main() {
    /// let x = 3i8.mul_div_ceil(4, 2);
    /// assert_eq!(x, Some(6));
    ///
    /// let x = 5i8.mul_div_ceil(2, 3);
    /// assert_eq!(x, Some(4));
    ///
    /// let x = (-5i8).mul_div_ceil(2, 3);
    /// assert_eq!(x, Some(-3));
    ///
    /// let x = 3i8.mul_div_ceil(3, 2);
    /// assert_eq!(x, Some(5));
    ///
    /// let x = (-3i8).mul_div_ceil(3, 2);
    /// assert_eq!(x, Some(-4));
    ///
    /// let x = (127i8).mul_div_ceil(4, 3);
    /// assert_eq!(x, None);
    /// # }
    /// ```
    fn mul_div_ceil(self, num: RHS, denom: RHS) -> Option<Self::Output>;

    /// Return u64 not out of bounds
    fn to_underflow_u64(self) -> u64;
}

pub trait Upcast256 {
    fn as_u256(self) -> U256;
}
impl Upcast256 for U128 {
    fn as_u256(self) -> U256 {
        U256([self.0[0], self.0[1], 0, 0])
    }
}

pub trait Downcast256 {
    /// Unsafe cast to U128
    /// Bits beyond the 128th position are lost
    fn as_u128(self) -> U128;
}
impl Downcast256 for U256 {
    fn as_u128(self) -> U128 {
        U128([self.0[0], self.0[1]])
    }
}

pub trait Upcast512 {
    fn as_u512(self) -> U512;
}
impl Upcast512 for U256 {
    fn as_u512(self) -> U512 {
        U512([self.0[0], self.0[1], self.0[2], self.0[3], 0, 0, 0, 0])
    }
}

pub trait Downcast512 {
    /// Unsafe cast to U256
    /// Bits beyond the 256th position are lost
    fn as_u256(self) -> U256;
}
impl Downcast512 for U512 {
    fn as_u256(self) -> U256 {
        U256([self.0[0], self.0[1], self.0[2], self.0[3]])
    }
}

impl MulDiv for U128 {
    type Output = U128;

    fn mul_div_floor(self, num: Self, denom: Self) -> Option<Self::Output> {
        assert_ne!(denom, U128::default());
        let r = ((self.as_u256()) * (num.as_u256())) / (denom.as_u256());
        if r > U128::MAX.as_u256() {
            None
        } else {
            Some(r.as_u128())
        }
    }

    fn mul_div_ceil(self, num: Self, denom: Self) -> Option<Self::Output> {
        assert_ne!(denom, U128::default());
        let r = (self.as_u256() * num.as_u256() + (denom - 1).as_u256()) / denom.as_u256();
        if r > U128::MAX.as_u256() {
            None
        } else {
            Some(r.as_u128())
        }
    }

    fn to_underflow_u64(self) -> u64 {
        if self < U128::from(u64::MAX) {
            self.as_u64()
        } else {
            0
        }
    }
}


pub trait UnsafeMathTrait {
    /// Returns ceil (x / y)
    /// Division by 0 throws a panic, and must be checked externally
    ///
    /// In Solidity dividing by 0 results in 0, not an exception.
    ///
    fn div_rounding_up(x: Self, y: Self) -> Self;
}

impl UnsafeMathTrait for u64 {
    fn div_rounding_up(x: Self, y: Self) -> Self {
        x / y + ((x % y > 0) as u64)
    }
}

impl UnsafeMathTrait for U128 {
    fn div_rounding_up(x: Self, y: Self) -> Self {
        x / y + U128::from((x % y > U128::default()) as u8)
    }
}

impl UnsafeMathTrait for U256 {
    fn div_rounding_up(x: Self, y: Self) -> Self {
        x / y + U256::from((x % y > U256::default()) as u8)
    }
}