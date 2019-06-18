// Copyright 2018 Eduardo Sánchez Muñoz
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#![allow(clippy::cognitive_complexity)]
#![allow(clippy::excessive_precision)]
#![allow(clippy::unreadable_literal)]

// Encode
#[test]
fn test_encode_bool() {
    assert_eq!(crate::encode_bool(true), "true");
    assert_eq!(crate::encode_bool(false), "false");
}

#[test]
fn test_encode_i8() {
    assert_eq!(crate::encode_i8(0), "0");
    assert_eq!(crate::encode_i8(1), "1");
    assert_eq!(crate::encode_i8(-1), "-1");
    assert_eq!(crate::encode_i8(127), "127");
    assert_eq!(crate::encode_i8(-128), "-128");
}

#[test]
fn test_encode_i16() {
    assert_eq!(crate::encode_i16(0), "0");
    assert_eq!(crate::encode_i16(1), "1");
    assert_eq!(crate::encode_i16(-1), "-1");
    assert_eq!(crate::encode_i16(32767), "32767");
    assert_eq!(crate::encode_i16(-32768), "-32768");
}

#[test]
fn test_encode_i32() {
    assert_eq!(crate::encode_i32(0), "0");
    assert_eq!(crate::encode_i32(1), "1");
    assert_eq!(crate::encode_i32(-1), "-1");
    assert_eq!(crate::encode_i32(2147483647), "2147483647");
    assert_eq!(crate::encode_i32(-2147483648), "-2147483648");
}

#[test]
fn test_encode_i64() {
    assert_eq!(crate::encode_i64(0), "0");
    assert_eq!(crate::encode_i64(1), "1");
    assert_eq!(crate::encode_i64(-1), "-1");
    assert_eq!(crate::encode_i64(9223372036854775807), "9223372036854775807");
    assert_eq!(crate::encode_i64(-9223372036854775808), "-9223372036854775808");
}

#[test]
fn test_encode_i128() {
    assert_eq!(crate::encode_i128(0), "0");
    assert_eq!(crate::encode_i128(1), "1");
    assert_eq!(crate::encode_i128(-1), "-1");
    assert_eq!(crate::encode_i128(170141183460469231731687303715884105727),
               "170141183460469231731687303715884105727");
    assert_eq!(crate::encode_i128(-170141183460469231731687303715884105728),
               "-170141183460469231731687303715884105728");
}

#[test]
fn test_encode_u8() {
    assert_eq!(crate::encode_u8(0), "0");
    assert_eq!(crate::encode_u8(1), "1");
    assert_eq!(crate::encode_u8(255), "255");
}

#[test]
fn test_encode_u16() {
    assert_eq!(crate::encode_u16(0), "0");
    assert_eq!(crate::encode_u16(1), "1");
    assert_eq!(crate::encode_u16(65535), "65535");
}

#[test]
fn test_encode_u32() {
    assert_eq!(crate::encode_u32(0), "0");
    assert_eq!(crate::encode_u32(1), "1");
    assert_eq!(crate::encode_u32(4294967295), "4294967295");
}

#[test]
fn test_encode_u64() {
    assert_eq!(crate::encode_u64(0), "0");
    assert_eq!(crate::encode_u64(1), "1");
    assert_eq!(crate::encode_u64(18446744073709551615), "18446744073709551615");
}

#[test]
fn test_encode_u128() {
    assert_eq!(crate::encode_u128(0), "0");
    assert_eq!(crate::encode_u128(1), "1");
    assert_eq!(crate::encode_u128(340282366920938463463374607431768211455),
               "340282366920938463463374607431768211455");
}

#[test]
fn test_encode_f32() {
    assert_eq!(crate::encode_f32(::std::f32::NAN), "NaN");
    assert_eq!(crate::encode_f32(::std::f32::INFINITY), "inf");
    assert_eq!(crate::encode_f32(::std::f32::NEG_INFINITY), "-inf");

    assert_eq!(crate::encode_f32(0.0), "0.0");
    assert_eq!(crate::encode_f32(1.0), "1.0");
    assert_eq!(crate::encode_f32(-1.0), "-1.0");
    assert_eq!(crate::encode_f32(10.0), "10.0");
    assert_eq!(crate::encode_f32(100.0), "100.0");
    assert_eq!(crate::encode_f32(1.0e10), "1.0e10");
    assert_eq!(crate::encode_f32(0.1), "0.1");
    assert_eq!(crate::encode_f32(0.01), "0.01");
    assert_eq!(crate::encode_f32(0.001), "0.001");
    assert_eq!(crate::encode_f32(0.0001), "0.0001");
    assert_eq!(crate::encode_f32(0.00001), "0.00001");
    assert_eq!(crate::encode_f32(10.1), "10.1");
    assert_eq!(crate::encode_f32(10.101), "10.101");
    assert_eq!(crate::encode_f32(10.101), "10.101");
    assert_eq!(crate::encode_f32(3.402823e38), "3.402823e38");
    assert_eq!(crate::encode_f32(1.175494e-38), "1.175494e-38");
}

#[test]
fn test_encode_f64() {
    assert_eq!(crate::encode_f64(::std::f64::NAN), "NaN");
    assert_eq!(crate::encode_f64(::std::f64::INFINITY), "inf");
    assert_eq!(crate::encode_f64(::std::f64::NEG_INFINITY), "-inf");

    assert_eq!(crate::encode_f64(0.0), "0.0");
    assert_eq!(crate::encode_f64(1.0), "1.0");
    assert_eq!(crate::encode_f64(-1.0), "-1.0");
    assert_eq!(crate::encode_f64(10.0), "10.0");
    assert_eq!(crate::encode_f64(100.0), "100.0");
    assert_eq!(crate::encode_f64(1.0e10), "1.0e10");
    assert_eq!(crate::encode_f64(0.1), "0.1");
    assert_eq!(crate::encode_f64(0.01), "0.01");
    assert_eq!(crate::encode_f64(0.001), "0.001");
    assert_eq!(crate::encode_f64(0.0001), "0.0001");
    assert_eq!(crate::encode_f64(0.00001), "0.00001");
    assert_eq!(crate::encode_f64(10.1), "10.1");
    assert_eq!(crate::encode_f64(10.101), "10.101");
    assert_eq!(crate::encode_f64(10.0000001), "10.0000001");
    assert_eq!(crate::encode_f64(1234.5678), "1234.5678");
    assert_eq!(crate::encode_f64(4.591856e50), "4.591856e50");
    assert_eq!(crate::encode_f64(4.591856e-50), "4.591856e-50");
    assert_eq!(crate::encode_f64(4.80144468355317e-300), "4.80144468355317e-300");
    assert_eq!(crate::encode_f64(4.80144468355317e+300), "4.80144468355317e300");
    assert_eq!(crate::encode_f64(1.2e-315), "1.2e-315");
    assert_eq!(crate::encode_f64(1.7976931348623157e+308), "1.7976931348623157e308");
    assert_eq!(crate::encode_f64(2.2250738585072014e-308), "2.2250738585072014e-308");
    assert_eq!(crate::encode_f64(5.0e-324), "5.0e-324");
}

#[test]
fn test_encode_byte_string() {
    assert_eq!(crate::encode_byte_string(b""), r#""""#);
    assert_eq!(crate::encode_byte_string(b"\x20\x7E"), r#"" ~""#);
    assert_eq!(crate::encode_byte_string(b" \t "), r#"" \t ""#);
    assert_eq!(crate::encode_byte_string(b" \n "), r#"" \n ""#);
    assert_eq!(crate::encode_byte_string(b" \" "), r#"" \" ""#);
    assert_eq!(crate::encode_byte_string(b" \\ "), r#"" \\ ""#);
    assert_eq!(crate::encode_byte_string(b" \x00 "), r#"" \x00 ""#);
    assert_eq!(crate::encode_byte_string(b" \xFF "), r#"" \xff ""#);
}


#[test]
fn test_encode_ascii_string() {
    assert_eq!(crate::encode_ascii_string(""), r#""""#);
    assert_eq!(crate::encode_ascii_string("\x20\x7E"), r#"" ~""#);
    assert_eq!(crate::encode_ascii_string(" \t "), r#"" \t ""#);
    assert_eq!(crate::encode_ascii_string(" \n "), r#"" \n ""#);
    assert_eq!(crate::encode_ascii_string(" \" "), r#"" \" ""#);
    assert_eq!(crate::encode_ascii_string(" \\ "), r#"" \\ ""#);
    assert_eq!(crate::encode_ascii_string(" \x00 "), r#"" \x00 ""#);
    assert_eq!(crate::encode_ascii_string(" \x7F "), r#"" \x7f ""#);
}

#[test]
fn test_encode_utf8_string() {
    assert_eq!(crate::encode_utf8_string(""), r#""""#);
    assert_eq!(crate::encode_utf8_string("\x20\x7E"), r#"" ~""#);
    assert_eq!(crate::encode_utf8_string(" \t "), r#"" \t ""#);
    assert_eq!(crate::encode_utf8_string(" \n "), r#"" \n ""#);
    assert_eq!(crate::encode_utf8_string(" \" "), r#"" \" ""#);
    assert_eq!(crate::encode_utf8_string(" \\ "), r#"" \\ ""#);
    assert_eq!(crate::encode_utf8_string(" \x00 "), r#"" \u{0} ""#);
    assert_eq!(crate::encode_utf8_string(" \x7F "), r#"" \u{7f} ""#);
    assert_eq!(crate::encode_utf8_string(" \u{FFFD} "), r#"" \u{fffd} ""#);
}

// Decode
#[test]
fn test_decode_bool() {
    assert_eq!(crate::decode_bool("true"), Some(true));
    assert_eq!(crate::decode_bool("false"), Some(false));

    assert_eq!(crate::decode_bool(""), None);
    assert_eq!(crate::decode_bool("TRUE"), None);
    assert_eq!(crate::decode_bool("FALSE"), None);
    assert_eq!(crate::decode_bool("abc"), None);
}

#[test]
fn test_decode_i8() {
    assert_eq!(crate::decode_i8("0"), Some(0));
    assert_eq!(crate::decode_i8("-0"), Some(0));
    assert_eq!(crate::decode_i8("+0"), Some(0));
    assert_eq!(crate::decode_i8("1"), Some(1));
    assert_eq!(crate::decode_i8("-1"), Some(-1));
    assert_eq!(crate::decode_i8("+1"), Some(1));

    assert_eq!(crate::decode_i8("-128"), Some(-128));
    assert_eq!(crate::decode_i8("127"), Some(127));
    assert_eq!(crate::decode_i8("-127"), Some(-127));

    assert_eq!(crate::decode_i8(""), None);
    assert_eq!(crate::decode_i8("-129"), None);
    assert_eq!(crate::decode_i8("128"), None);
}

#[test]
fn test_decode_i16() {
    assert_eq!(crate::decode_i16("0"), Some(0));
    assert_eq!(crate::decode_i16("-0"), Some(0));
    assert_eq!(crate::decode_i16("+0"), Some(0));
    assert_eq!(crate::decode_i16("1"), Some(1));
    assert_eq!(crate::decode_i16("-1"), Some(-1));
    assert_eq!(crate::decode_i16("+1"), Some(1));

    assert_eq!(crate::decode_i16("-32768"), Some(-32768));
    assert_eq!(crate::decode_i16("32767"), Some(32767));
    assert_eq!(crate::decode_i16("-32767"), Some(-32767));

    assert_eq!(crate::decode_i16(""), None);
    assert_eq!(crate::decode_i16("-32769"), None);
    assert_eq!(crate::decode_i16("32768"), None);
}

#[test]
fn test_decode_i32() {
    assert_eq!(crate::decode_i32("0"), Some(0));
    assert_eq!(crate::decode_i32("-0"), Some(0));
    assert_eq!(crate::decode_i32("+0"), Some(0));
    assert_eq!(crate::decode_i32("1"), Some(1));
    assert_eq!(crate::decode_i32("-1"), Some(-1));
    assert_eq!(crate::decode_i32("+1"), Some(1));

    assert_eq!(crate::decode_i32("-2147483648"), Some(-2147483648));
    assert_eq!(crate::decode_i32("2147483647"), Some(2147483647));
    assert_eq!(crate::decode_i32("-2147483647"), Some(-2147483647));

    assert_eq!(crate::decode_i32(""), None);
    assert_eq!(crate::decode_i32("-2147483649"), None);
    assert_eq!(crate::decode_i32("2147483648"), None);
}

#[test]
fn test_decode_i64() {
    assert_eq!(crate::decode_i64("0"), Some(0));
    assert_eq!(crate::decode_i64("-0"), Some(0));
    assert_eq!(crate::decode_i64("+0"), Some(0));
    assert_eq!(crate::decode_i64("1"), Some(1));
    assert_eq!(crate::decode_i64("-1"), Some(-1));
    assert_eq!(crate::decode_i64("+1"), Some(1));

    assert_eq!(crate::decode_i64("-9223372036854775808"), Some(-9223372036854775808));
    assert_eq!(crate::decode_i64("9223372036854775807"), Some(9223372036854775807));
    assert_eq!(crate::decode_i64("-9223372036854775807"), Some(-9223372036854775807));

    assert_eq!(crate::decode_i64(""), None);
    assert_eq!(crate::decode_i64("-9223372036854775809"), None);
    assert_eq!(crate::decode_i64("9223372036854775808"), None);
}

#[test]
fn test_decode_i128() {
    assert_eq!(crate::decode_i128("0"), Some(0));
    assert_eq!(crate::decode_i128("-0"), Some(0));
    assert_eq!(crate::decode_i128("+0"), Some(0));
    assert_eq!(crate::decode_i128("1"), Some(1));
    assert_eq!(crate::decode_i128("-1"), Some(-1));
    assert_eq!(crate::decode_i128("+1"), Some(1));

    assert_eq!(crate::decode_i128("-170141183460469231731687303715884105728"),
               Some(-170141183460469231731687303715884105728));
    assert_eq!(crate::decode_i128("170141183460469231731687303715884105727"),
               Some(170141183460469231731687303715884105727));
    assert_eq!(crate::decode_i128("-170141183460469231731687303715884105727"),
               Some(-170141183460469231731687303715884105727));

    assert_eq!(crate::decode_i128(""), None);
    assert_eq!(crate::decode_i128("-170141183460469231731687303715884105729"), None);
    assert_eq!(crate::decode_i128("170141183460469231731687303715884105728"), None);
}

#[test]
fn test_decode_u8() {
    assert_eq!(crate::decode_u8("0"), Some(0));
    assert_eq!(crate::decode_u8("1"), Some(1));

    assert_eq!(crate::decode_u8("255"), Some(255));

    assert_eq!(crate::decode_u8(""), None);
    assert_eq!(crate::decode_u8("256"), None);
}

#[test]
fn test_decode_u16() {
    assert_eq!(crate::decode_u16("0"), Some(0));
    assert_eq!(crate::decode_u16("1"), Some(1));

    assert_eq!(crate::decode_u16("65535"), Some(65535));

    assert_eq!(crate::decode_u16(""), None);
    assert_eq!(crate::decode_u16("65536"), None);
}

#[test]
fn test_decode_u32() {
    assert_eq!(crate::decode_u32("0"), Some(0));
    assert_eq!(crate::decode_u32("1"), Some(1));

    assert_eq!(crate::decode_u32("4294967295"), Some(4294967295));

    assert_eq!(crate::decode_u32(""), None);
    assert_eq!(crate::decode_u32("4294967296"), None);
}

#[test]
fn test_decode_u64() {
    assert_eq!(crate::decode_u64("0"), Some(0));
    assert_eq!(crate::decode_u64("1"), Some(1));

    assert_eq!(crate::decode_u64("18446744073709551615"), Some(18446744073709551615));

    assert_eq!(crate::decode_u64(""), None);
    assert_eq!(crate::decode_u64("18446744073709551616"), None);
}

#[test]
fn test_decode_u128() {
    assert_eq!(crate::decode_u128("0"), Some(0));
    assert_eq!(crate::decode_u128("1"), Some(1));

    assert_eq!(crate::decode_u128("340282366920938463463374607431768211455"),
               Some(340282366920938463463374607431768211455));

    assert_eq!(crate::decode_u128(""), None);
    assert_eq!(crate::decode_u128("340282366920938463463374607431768211456"), None);
}

#[test]
fn test_decode_f32() {
    fn approx(x: f32, y: f32) -> bool {
        (x - y).abs() < (1.0 * ::std::f32::EPSILON * f32::max(x.abs(), y.abs()))
    }

    assert!(crate::decode_f32("NaN").unwrap().is_nan());
    assert_eq!(crate::decode_f32("inf"), Some(::std::f32::INFINITY));
    assert_eq!(crate::decode_f32("+inf"), Some(::std::f32::INFINITY));
    assert_eq!(crate::decode_f32("-inf"), Some(::std::f32::NEG_INFINITY));

    assert_eq!(crate::decode_f32("0"), Some(0.0));
    assert_eq!(crate::decode_f32("0.0"), Some(0.0));
    assert_eq!(crate::decode_f32("1"), Some(1.0));
    assert_eq!(crate::decode_f32("1.0"), Some(1.0));
    assert_eq!(crate::decode_f32("-1"), Some(-1.0));
    assert_eq!(crate::decode_f32("-1.0"), Some(-1.0));
    assert_eq!(crate::decode_f32("+1"), Some(1.0));
    assert_eq!(crate::decode_f32("+1.0"), Some(1.0));
    assert_eq!(crate::decode_f32(".0"), Some(0.0));
    assert_eq!(crate::decode_f32("+.0"), Some(0.0));
    assert_eq!(crate::decode_f32("-.0"), Some(0.0));
    assert_eq!(crate::decode_f32("0."), Some(0.0));
    assert_eq!(crate::decode_f32("+0."), Some(0.0));
    assert_eq!(crate::decode_f32("-0."), Some(0.0));

    assert!(approx(crate::decode_f32("1.1").unwrap(), 1.1));
    assert!(approx(crate::decode_f32("1e2").unwrap(), 1.0e2));
    assert!(approx(crate::decode_f32("1E2").unwrap(), 1.0e2));
    assert!(approx(crate::decode_f32("1.e1").unwrap(), 1.0e1));
    assert!(approx(crate::decode_f32("1.E1").unwrap(), 1.0e1));
    assert!(approx(crate::decode_f32(".1e1").unwrap(), 0.1e1));
    assert!(approx(crate::decode_f32(".1E1").unwrap(), 0.1e1));
    assert!(approx(crate::decode_f32("1.1e2").unwrap(), 1.1e2));
    assert!(approx(crate::decode_f32("1.1e+2").unwrap(), 1.1e2));
    assert!(approx(crate::decode_f32("1.1e-2").unwrap(), 1.1e-2));
    assert!(approx(crate::decode_f32("123e-20").unwrap(), 123.0e-20));
    assert!(approx(crate::decode_f32("123e+20").unwrap(), 123.0e20));
    assert!(approx(crate::decode_f32("1000000000000").unwrap(), 1000000000000.0));
    assert!(approx(crate::decode_f32("0.000000000001").unwrap(), 0.000000000001));
    assert!(approx(crate::decode_f32("1.23456789").unwrap(), 1.23456789));
    assert!(approx(crate::decode_f32("480.1444").unwrap(), 480.1444));
    assert!(approx(crate::decode_f32("480.1444e-30").unwrap(), 480.1444e-30));
    assert!(approx(crate::decode_f32("480.1444e+30").unwrap(), 480.1444e+30));

    assert_eq!(crate::decode_f32("0e99999999999999999"), Some(0.0));
    assert_eq!(crate::decode_f32("1e-99999999999999999"), Some(0.0));
    assert_eq!(crate::decode_f32("0e9999999999999999999999999999999999"), Some(0.0));
    assert_eq!(crate::decode_f32("1e-9999999999999999999999999999999999"), Some(0.0));

    assert_eq!(crate::decode_f32(""), None);
    assert_eq!(crate::decode_f32("."), None);
    assert_eq!(crate::decode_f32("+."), None);
    assert_eq!(crate::decode_f32("-."), None);
    assert_eq!(crate::decode_f32("e1"), None);
    assert_eq!(crate::decode_f32("E1"), None);
    assert_eq!(crate::decode_f32("+e1"), None);
    assert_eq!(crate::decode_f32("+E1"), None);
    assert_eq!(crate::decode_f32("-e1"), None);
    assert_eq!(crate::decode_f32("-E1"), None);
    assert_eq!(crate::decode_f32("1e1000"), None);
    assert_eq!(crate::decode_f32("1e9999999999999999999999999999999999"), None);
    assert_eq!(crate::decode_f32("1e"), None);
    assert_eq!(crate::decode_f32("1E"), None);
}

#[test]
fn test_decode_f64() {
    fn approx(x: f64, y: f64) -> bool {
        (x - y).abs() < (1.0 * ::std::f64::EPSILON * f64::max(x.abs(), y.abs()))
    }

    assert!(crate::decode_f64("NaN").unwrap().is_nan());
    assert_eq!(crate::decode_f64("inf"), Some(::std::f64::INFINITY));
    assert_eq!(crate::decode_f64("+inf"), Some(::std::f64::INFINITY));
    assert_eq!(crate::decode_f64("-inf"), Some(::std::f64::NEG_INFINITY));

    assert_eq!(crate::decode_f64("0"), Some(0.0));
    assert_eq!(crate::decode_f64("0.0"), Some(0.0));
    assert_eq!(crate::decode_f64("1"), Some(1.0));
    assert_eq!(crate::decode_f64("1.0"), Some(1.0));
    assert_eq!(crate::decode_f64("-1"), Some(-1.0));
    assert_eq!(crate::decode_f64("-1.0"), Some(-1.0));
    assert_eq!(crate::decode_f64("+1"), Some(1.0));
    assert_eq!(crate::decode_f64("+1.0"), Some(1.0));
    assert_eq!(crate::decode_f64(".0"), Some(0.0));
    assert_eq!(crate::decode_f64("+.0"), Some(0.0));
    assert_eq!(crate::decode_f64("-.0"), Some(0.0));
    assert_eq!(crate::decode_f64("0."), Some(0.0));
    assert_eq!(crate::decode_f64("+0."), Some(0.0));
    assert_eq!(crate::decode_f64("-0."), Some(0.0));

    assert!(approx(crate::decode_f64("1.1").unwrap(), 1.1));
    assert!(approx(crate::decode_f64("1e2").unwrap(), 1.0e2));
    assert!(approx(crate::decode_f64("1E2").unwrap(), 1.0e2));
    assert!(approx(crate::decode_f64("1.e1").unwrap(), 1.0e1));
    assert!(approx(crate::decode_f64("1.E1").unwrap(), 1.0e1));
    assert!(approx(crate::decode_f64(".1e1").unwrap(), 0.1e1));
    assert!(approx(crate::decode_f64(".1E1").unwrap(), 0.1e1));
    assert!(approx(crate::decode_f64("1.1e2").unwrap(), 1.1e2));
    assert!(approx(crate::decode_f64("1.1e+2").unwrap(), 1.1e2));
    assert!(approx(crate::decode_f64("1.1e-2").unwrap(), 1.1e-2));
    assert!(approx(crate::decode_f64("123e-90").unwrap(), 123.0e-90));
    assert!(approx(crate::decode_f64("123e+90").unwrap(), 123.0e90));
    assert!(approx(crate::decode_f64("1000000000000").unwrap(), 1000000000000.0));
    assert!(approx(crate::decode_f64("0.000000000001").unwrap(), 0.000000000001));
    assert!(approx(crate::decode_f64("1.23456789").unwrap(), 1.23456789));
    assert!(approx(crate::decode_f64("1.2345678987654321").unwrap(), 1.2345678987654321));
    assert!(approx(crate::decode_f64("480.144468355317204515627862").unwrap(), 480.144468355317204515627862));
    assert!(approx(crate::decode_f64("480.144468355317204515627862e-250").unwrap(), 480.144468355317204515627862e-250));
    assert!(approx(crate::decode_f64("480.144468355317204515627862e+250").unwrap(), 480.144468355317204515627862e+250));

    assert_eq!(crate::decode_f64("0e99999999999999999"), Some(0.0));
    assert_eq!(crate::decode_f64("1e-99999999999999999"), Some(0.0));
    assert_eq!(crate::decode_f64("0e9999999999999999999999999999999999"), Some(0.0));
    assert_eq!(crate::decode_f64("1e-9999999999999999999999999999999999"), Some(0.0));

    assert_eq!(crate::decode_f64(""), None);
    assert_eq!(crate::decode_f64("."), None);
    assert_eq!(crate::decode_f64("+."), None);
    assert_eq!(crate::decode_f64("-."), None);
    assert_eq!(crate::decode_f64("e1"), None);
    assert_eq!(crate::decode_f64("E1"), None);
    assert_eq!(crate::decode_f64("+e1"), None);
    assert_eq!(crate::decode_f64("+E1"), None);
    assert_eq!(crate::decode_f64("-e1"), None);
    assert_eq!(crate::decode_f64("-E1"), None);
    assert_eq!(crate::decode_f64("1e1000"), None);
    assert_eq!(crate::decode_f64("1e9999999999999999999999999999999999"), None);
    assert_eq!(crate::decode_f64("1e"), None);
    assert_eq!(crate::decode_f64("1E"), None);
}

#[test]
fn test_decode_byte_string() {
    assert_eq!(crate::decode_byte_string(r#""""#).unwrap(), b"");
    assert_eq!(crate::decode_byte_string(r#""123""#).unwrap(), b"123");
    assert_eq!(crate::decode_byte_string(r#"" \\ ""#).unwrap(), b" \\ ");
    assert_eq!(crate::decode_byte_string(r#"" \" ""#).unwrap(), b" \" ");
    assert_eq!(crate::decode_byte_string(r#"" \r\n\t ""#).unwrap(), b" \r\n\t ");
    assert_eq!(crate::decode_byte_string(r#"" \x00\xFF ""#).unwrap(), b" \x00\xFF ");
    assert_eq!(crate::decode_byte_string(r#"" \xaB\xCd ""#).unwrap(), b" \xAB\xCD ");

    assert_eq!(crate::decode_byte_string(""), None);
    assert_eq!(crate::decode_byte_string(r#"""#), None);
    assert_eq!(crate::decode_byte_string(r#""" "#), None);
    assert_eq!(crate::decode_byte_string(r#""\""#), None);
    assert_eq!(crate::decode_byte_string(r#"" \M ""#), None);
    assert_eq!(crate::decode_byte_string(r#"" \xT0 ""#), None);
    assert_eq!(crate::decode_byte_string(r#"" \x0T ""#), None);
}

#[test]
fn test_decode_ascii_string() {
    assert_eq!(crate::decode_ascii_string(r#""""#).unwrap(), "");
    assert_eq!(crate::decode_ascii_string(r#""123""#).unwrap(), "123");
    assert_eq!(crate::decode_ascii_string(r#"" \\ ""#).unwrap(), " \\ ");
    assert_eq!(crate::decode_ascii_string(r#"" \" ""#).unwrap(), " \" ");
    assert_eq!(crate::decode_ascii_string(r#"" \r\n\t ""#).unwrap(), " \r\n\t ");
    assert_eq!(crate::decode_ascii_string(r#"" \x00\x7F ""#).unwrap(), " \x00\x7F ");
    assert_eq!(crate::decode_ascii_string(r#"" \x1B\x1d ""#).unwrap(), " \x1B\x1D ");

    assert_eq!(crate::decode_ascii_string(""), None);
    assert_eq!(crate::decode_ascii_string("\" \u{FF} \""), None);
    assert_eq!(crate::decode_ascii_string(r#"""#), None);
    assert_eq!(crate::decode_ascii_string(r#""" "#), None);
    assert_eq!(crate::decode_ascii_string(r#""\""#), None);
    assert_eq!(crate::decode_ascii_string(r#"" \M ""#), None);
    assert_eq!(crate::decode_ascii_string(r#"" \xT0 ""#), None);
    assert_eq!(crate::decode_ascii_string(r#"" \x0T ""#), None);
    assert_eq!(crate::decode_ascii_string(r#"" \x80 ""#), None);
}

#[test]
fn test_decode_utf8_string() {
    assert_eq!(crate::decode_utf8_string(r#""""#).unwrap(), "");
    assert_eq!(crate::decode_utf8_string(r#""123""#).unwrap(), "123");
    assert_eq!(crate::decode_utf8_string(r#"" \\ ""#).unwrap(), " \\ ");
    assert_eq!(crate::decode_utf8_string(r#"" \" ""#).unwrap(), " \" ");
    assert_eq!(crate::decode_utf8_string(r#"" \r\n\t ""#).unwrap(), " \r\n\t ");
    assert_eq!(crate::decode_utf8_string(r#"" \x00\x7F ""#).unwrap(), " \x00\x7F ");
    assert_eq!(crate::decode_utf8_string(r#"" \x1B\x1d ""#).unwrap(), " \x1B\x1D ");
    assert_eq!(crate::decode_utf8_string("\" \u{FF} \"").unwrap(), " \u{FF} ");
    assert_eq!(crate::decode_utf8_string(r#"" \u{FF} ""#).unwrap(), " \u{FF} ");
    assert_eq!(crate::decode_utf8_string(r#"" \u{ff} ""#).unwrap(), " \u{FF} ");

    assert_eq!(crate::decode_utf8_string(""), None);
    assert_eq!(crate::decode_utf8_string(r#"""#), None);
    assert_eq!(crate::decode_utf8_string(r#""" "#), None);
    assert_eq!(crate::decode_utf8_string(r#""\""#), None);
    assert_eq!(crate::decode_utf8_string(r#"" \M ""#), None);
    assert_eq!(crate::decode_utf8_string(r#"" \xT0 ""#), None);
    assert_eq!(crate::decode_utf8_string(r#"" \x0T ""#), None);
    assert_eq!(crate::decode_utf8_string(r#"" \x80 ""#), None);
}
