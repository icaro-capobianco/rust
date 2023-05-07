use core::slice;
use std::collections::btree_set::Union;

// Functions
fn function() {
    "can return the last statement";
}

// Types

/** Integer and ranges
 * Most systems 1 byte = 8 bits */

// 1 byte
const SINT8: i8 = 8; // Signed:   -128 a 127
const UINT8: u8 = 8; // Unsigned: 0 a 255

// 2 bytes
const SINT16: i16 = 16; // Signed:   -32768 a 32767
const UINT16: u16 = 16; // Unsigned: 0 a 65535

// 4 bytes
const SINT32: i32 = 32; // Signed:   -2147483648 a 2147483647
const UINT32: u32 = 32; // Unsigned: 0 a 4294967295

// 8 bytes
const SINT64: i64 = 64; // Signed:   -9223372036854775808 a 9223372036854775807
const UINT64: u64 = 64; // Unsigned: 0 a 18446744073709551615

// 16 bytes
const SINT128: i128 = 128; // Signed:   -170141183460469231731687303715884105728 a 170141183460469231731687303715884105727
const UINT128: u128 = 128; // Unsigned: 0 a 340282366920938463463374607431768211455

/** THE _UNDERSCORE_
 * throw away the value */
fn underscore() {
    let _ = function(); // Throws away the result
}

/** Tuples
 * - can be destructured
 * - can be split */
fn tuples() {
    let (pair) = ('a', 'b');
    let (a, b) = pair;
}

/** Blocks
 * return the last statement */
fn blocks() {
    let block = { "the final expression" };
    assert!(block == "the final expression");
}

// Enum instead of Union
enum StrOrI32 {
    str,
    i32,
}
/** Match is like switch */
fn _matchStr(a: StrOrI32) {
    match a {
        StrOrI32::str => true,
        StrOrI32::i32 => false,
    };
}

fn main() {}
