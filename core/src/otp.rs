/*
Copyright 2023, Mengxiao Lin <linmx0130@gmail.com>

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

use crypto::{hmac::Hmac, sha1::Sha1, mac::Mac};

/// Implementation of HMAC-based One-Time Password as it is described
/// in RFC 4226. 
pub fn hotp(key: &[u8], c: u64) -> String {
    let digest = Sha1::new();
    let mut hmac = Hmac::new(digest, key);
    hmac.input(&big_endian_u64(c));
    let hash = hmac.result();
    let length = hash.code().len();
    let offset = hash.code()[length-1] & 0xF;   
    let mut hotp_num= extract31(hash.code(), offset as usize);
    println!("{}", hotp_num);

    let mut hotp: Vec<u8> = Vec::new();
    for _i in 0..6 {
        let c = '0' as u8 + (hotp_num % 10) as u8;
        hotp_num = hotp_num / 10;
        hotp.push(c);
    }
    hotp.reverse();
    String::from_utf8(hotp).unwrap()
}

fn extract31(hash: &[u8], offset: usize) -> u32 {
    let mut value = 0u32;
    for i in 0..4 {
        let pos_shift = (3-i) * 8;
        value = value | ((hash[offset + i] as u32) << pos_shift);
    }
    value  & 0x7FFFFFFF
}

fn big_endian_u64(v: u64)-> [u8;8] {
    let mut r = [0u8;8];
    for i in 0..8 {
        let offset = (7 - i) * 8;
        let mask = 0xFFu64 << offset;
        r[i] = ((v & mask)>> offset) as u8;
    }
    return r;
}


mod test {
    use crate::base32;

    use super::{big_endian_u64, extract31, hotp};

    #[test]
    fn test_big_endian() {
        let v = big_endian_u64(0xdeadbeef12345678);
        assert_eq!(v[0], 0xde); 
        assert_eq!(v[1], 0xad); 
        assert_eq!(v[2], 0xbe); 
        assert_eq!(v[3], 0xef); 
        assert_eq!(v[4], 0x12); 
        assert_eq!(v[5], 0x34); 
        assert_eq!(v[6], 0x56); 
        assert_eq!(v[7], 0x78); 
    }

    #[test]
    fn test_extract31() {
        let v = big_endian_u64(0xdeadbeef12345678);
        let x = extract31(&v, 0);
        assert_eq!(x, 0x5eadbeef);
        let x = extract31(&v, 2);
        assert_eq!(x, 0x3eef1234);
    }
    
    #[test]
    fn test_hotp() {
        let key = big_endian_u64(0xdeadbeef12345678);
        let c = 19260817;
        let code = hotp(&key, c);
        assert_eq!(code, "649433");
    }

    #[test]
    fn test_hotp_google_auth() {
        // This test case is from Google Authenticator Android unit test.
        // See more in https://github.com/google/google-authenticator-android/blob/master/javatests/com/google/android/apps/authenticator/otp/PasscodeGeneratorTest.java
        let key = base32::decode("7777777777777777").unwrap();
        assert_eq!(hotp(&key, 0), "724477");
        assert_eq!(hotp(&key, 123456789123456789), "815107");
    }
}