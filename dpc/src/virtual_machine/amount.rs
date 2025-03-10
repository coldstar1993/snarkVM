// Copyright (C) 2019-2021 Aleo Systems Inc.
// This file is part of the snarkVM library.

// The snarkVM library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkVM library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkVM library. If not, see <https://www.gnu.org/licenses/>.

use snarkvm_utilities::{FromBytes, ToBytes};

use serde::{Deserialize, Serialize};
use std::{
    fmt,
    io::{Read, Result as IoResult, Write},
};

/// Represents the amount of ALEOs.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct AleoAmount(pub i64);

pub enum Denomination {
    /// AB
    BYTE,
    /// AG
    GATE,
    /// ALEO
    CREDIT,
}

impl Denomination {
    /// The number of decimal places more than a Unit.
    fn precision(self) -> u32 {
        match self {
            Denomination::BYTE => 0,
            Denomination::GATE => 3,
            Denomination::CREDIT => 6,
        }
    }
}

impl fmt::Display for Denomination {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Denomination::BYTE => "AB",
            Denomination::GATE => "AG",
            Denomination::CREDIT => "ALEO",
        })
    }
}

impl AleoAmount {
    /// Exactly one Aleo byte (AB).
    pub const ONE_BYTE: AleoAmount = AleoAmount(1i64);
    /// Exactly one Aleo credit (ALEO).
    pub const ONE_CREDIT: AleoAmount = AleoAmount(1_000_000i64);
    /// Exactly one Aleo gate (AG).
    pub const ONE_GATE: AleoAmount = AleoAmount(1_000i64);
    /// The zero amount.
    pub const ZERO: AleoAmount = AleoAmount(0i64);

    /// Create an `AleoAmount` given a number of bytes
    pub fn from_bytes(bytes: i64) -> Self {
        Self(bytes)
    }

    /// Create an `AleoAmount` given a number of gates
    pub fn from_gates(gate_value: i64) -> Self {
        let bytes = gate_value * 10_i64.pow(Denomination::GATE.precision());

        Self::from_bytes(bytes)
    }

    /// Create an `AleoAmount` given a number of ALEOs
    pub fn from_aleo(aleo_value: i64) -> Self {
        let bytes = aleo_value * 10_i64.pow(Denomination::CREDIT.precision());

        Self::from_bytes(bytes)
    }

    /// Add the values of two `AleoAmount`s
    #[allow(clippy::should_implement_trait)]
    pub fn add(self, b: Self) -> Self {
        Self::from_bytes(self.0 + b.0)
    }

    /// Subtract the value of two `AleoAmounts`
    #[allow(clippy::should_implement_trait)]
    pub fn sub(self, b: AleoAmount) -> Self {
        Self::from_bytes(self.0 - b.0)
    }

    /// Returns `true` the amount is positive and `false` if the amount is zero or
    /// negative.
    pub const fn is_positive(self) -> bool {
        self.0.is_positive()
    }

    /// Returns `true` if the amount is negative and `false` if the amount is zero or
    /// positive.
    pub const fn is_negative(self) -> bool {
        self.0.is_negative()
    }

    /// Returns `true` if the amount is zero and `false` if the amount is not zero.
    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }
}

impl ToBytes for AleoAmount {
    fn write_le<W: Write>(&self, mut writer: W) -> IoResult<()> {
        self.0.write_le(&mut writer)
    }
}

impl FromBytes for AleoAmount {
    #[inline]
    fn read_le<R: Read>(mut reader: R) -> IoResult<Self> {
        let amount: i64 = FromBytes::read_le(&mut reader)?;

        Ok(Self(amount))
    }
}

impl fmt::Display for AleoAmount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_from_byte(byte_value: i64, expected_amount: AleoAmount) {
        let amount = AleoAmount::from_bytes(byte_value);
        assert_eq!(expected_amount, amount)
    }

    fn test_from_gate(gate_value: i64, expected_amount: AleoAmount) {
        let amount = AleoAmount::from_gates(gate_value);
        assert_eq!(expected_amount, amount)
    }

    fn test_from_aleo(aleo_value: i64, expected_amount: AleoAmount) {
        let amount = AleoAmount::from_aleo(aleo_value);
        assert_eq!(expected_amount, amount)
    }

    fn test_addition(a: &i64, b: &i64, result: &i64) {
        let a = AleoAmount::from_bytes(*a);
        let b = AleoAmount::from_bytes(*b);
        let result = AleoAmount::from_bytes(*result);

        assert_eq!(result, a.add(b));
    }

    fn test_subtraction(a: &i64, b: &i64, result: &i64) {
        let a = AleoAmount::from_bytes(*a);
        let b = AleoAmount::from_bytes(*b);
        let result = AleoAmount::from_bytes(*result);

        assert_eq!(result, a.sub(b));
    }

    pub(crate) struct AmountDenominationTestCase {
        byte: i64,
        gate: i64,
        aleo: i64,
    }

    mod valid_conversions {
        use super::*;

        const TEST_AMOUNTS: [AmountDenominationTestCase; 5] = [
            AmountDenominationTestCase {
                byte: 0,
                gate: 0,
                aleo: 0,
            },
            AmountDenominationTestCase {
                byte: 1_000_000,
                gate: 1_000,
                aleo: 1,
            },
            AmountDenominationTestCase {
                byte: 1_000_000_000,
                gate: 1_000_000,
                aleo: 1_000,
            },
            AmountDenominationTestCase {
                byte: 1_234_567_000_000_000,
                gate: 1_234_567_000_000,
                aleo: 1_234_567_000,
            },
            AmountDenominationTestCase {
                byte: 1_000_000_000_000_000_000,
                gate: 1_000_000_000_000_000,
                aleo: 1_000_000_000_000,
            },
        ];

        #[test]
        fn test_byte_conversion() {
            TEST_AMOUNTS
                .iter()
                .for_each(|amounts| test_from_byte(amounts.byte, AleoAmount(amounts.byte)));
        }

        #[test]
        fn test_gate_conversion() {
            TEST_AMOUNTS
                .iter()
                .for_each(|amounts| test_from_gate(amounts.gate, AleoAmount(amounts.byte)));
        }

        #[test]
        fn test_aleo_conversion() {
            TEST_AMOUNTS
                .iter()
                .for_each(|amounts| test_from_aleo(amounts.aleo, AleoAmount(amounts.byte)));
        }
    }

    mod valid_arithmetic {
        use super::*;

        const TEST_VALUES: [(i64, i64, i64); 7] = [
            (0, 0, 0),
            (1, 2, 3),
            (100000, 0, 100000),
            (123456789, 987654321, 1111111110),
            (100000000000000, 1000000000000000, 1100000000000000),
            (-100000000000000, -1000000000000000, -1100000000000000),
            (100000000000000, -100000000000000, 0),
        ];

        #[test]
        fn test_valid_addition() {
            TEST_VALUES.iter().for_each(|(a, b, c)| test_addition(a, b, c));
        }

        #[test]
        fn test_valid_subtraction() {
            TEST_VALUES.iter().for_each(|(a, b, c)| test_subtraction(c, b, a));
        }
    }

    mod test_invalid {
        use super::*;

        mod test_invalid_conversion {
            use super::*;

            const INVALID_TEST_AMOUNTS: [AmountDenominationTestCase; 4] = [
                AmountDenominationTestCase {
                    byte: 1,
                    gate: 1,
                    aleo: 1,
                },
                AmountDenominationTestCase {
                    byte: 10,
                    gate: 10000,
                    aleo: 100000000,
                },
                AmountDenominationTestCase {
                    byte: 1234567,
                    gate: 123,
                    aleo: 1,
                },
                AmountDenominationTestCase {
                    byte: 1_000_000_000_000_000_000,
                    gate: 1_000_000_000_000_000,
                    aleo: 999_999_999_999,
                },
            ];

            #[should_panic]
            #[test]
            fn test_invalid_gate_conversion() {
                INVALID_TEST_AMOUNTS
                    .iter()
                    .for_each(|amounts| test_from_gate(amounts.gate, AleoAmount(amounts.byte)));
            }

            #[should_panic]
            #[test]
            fn test_invalid_aleo_conversion() {
                INVALID_TEST_AMOUNTS
                    .iter()
                    .for_each(|amounts| test_from_aleo(amounts.aleo, AleoAmount(amounts.byte)));
            }
        }

        mod invalid_arithmetic {
            use super::*;

            const TEST_VALUES: [(i64, i64, i64); 5] = [
                (0, 0, 1),
                (1, 2, 5),
                (100000, 1, 100000),
                (123456789, 123456789, 123456789),
                (-1000, -1000, 2000),
            ];

            #[should_panic]
            #[test]
            fn test_invalid_addition() {
                TEST_VALUES.iter().for_each(|(a, b, c)| test_addition(a, b, c));
            }

            #[should_panic]
            #[test]
            fn test_invalid_subtraction() {
                TEST_VALUES.iter().for_each(|(a, b, c)| test_subtraction(a, b, c));
            }
        }
    }
}
