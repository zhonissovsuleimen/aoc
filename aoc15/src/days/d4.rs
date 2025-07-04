use aoc_trait::Day;

pub struct D4;


/*
--- Day 4: The Ideal Stocking Stuffer ---

Santa needs help mining some AdventCoins (very similar to bitcoins) to use as gifts for all the economically forward-thinking little girls and boys.

To do this, he needs to find MD5 hashes which, in hexadecimal, start with at least five zeroes. The input to the MD5 hash is some secret key (your puzzle input, given below) followed by a number in decimal. To mine AdventCoins, you must find Santa the lowest positive number (no leading zeroes: 1, 2, 3, ...) that produces such a hash.

For example:

    If your secret key is abcdef, the answer is 609043, because the MD5 hash of abcdef609043 starts with five zeroes (000001dbbfa...), and it is the lowest such number to do so.
    If your secret key is pqrstuv, the lowest number it combines with to make an MD5 hash starting with five zeroes is 1048970; that is, the MD5 hash of pqrstuv1048970 looks like 000006136ef....
*/

impl Day for D4 {
  fn day(&self) -> usize {
    4
  }

  fn input(&self) -> Option<String> {
    Some(String::from("yzbqklnj"))
  }

  fn solution(&self) -> Option<String> {
    let Some(input) = self.input() else {
      return None;
    };

    None
  }

  fn solution_extra(&self) -> Option<String> {
    let Some(input) = self.input() else {
      return None;
    };

    None
  }
}

//extras
impl D4 {
  //https://en.wikipedia.org/wiki/MD5#Pseudocode
  fn md5(bytes: &[u8]) -> [u8; 16] {
    const S: [u32; 64] = [
      7, 12, 17, 22, 
      7, 12, 17, 22, 
      7, 12, 17, 22, 
      7, 12, 17, 22, 
      5, 9, 14, 20, 
      5, 9, 14, 20, 
      5, 9, 14, 20, 
      5, 9, 14, 20, 
      4, 11, 16, 23, 
      4, 11, 16, 23, 
      4, 11, 16, 23, 
      4, 11, 16, 23, 
      6, 10, 15, 21, 
      6, 10, 15, 21, 
      6, 10, 15, 21, 
      6, 10, 15, 21,
    ];

    const K: [u32; 64] = [
      0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 
      0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501, 
      0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 
      0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821, 
      0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 
      0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8, 
      0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed, 
      0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a, 
      0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c, 
      0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70, 
      0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05, 
      0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665, 
      0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039, 
      0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1, 
      0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 
      0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391,
    ];

    let mut a0 = 0x67452301_u32;
    let mut b0 = 0xefcdab89_u32;
    let mut c0 = 0x98badcfe_u32;
    let mut d0 = 0x10325476_u32;

    let mut msg = bytes.to_vec();
    let original_len = msg.len() as u64 * 8;

    msg.push(0x80);
    //same as (len * 8) % 512 != 448
    while (msg.len() % 64) != 56 {
      msg.push(0);
    }
    msg.extend_from_slice(&original_len.to_le_bytes());

    //512 bits = 64 bytes
    for chunk in msg.chunks(64) {
      let mut words = [0_u32; 16];

      for (i, minichunk) in chunk.chunks(4).enumerate() {
        words[i] = u32::from_le_bytes(minichunk.try_into().unwrap());
      }

      let mut a = a0;
      let mut b = b0;
      let mut c = c0;
      let mut d = d0;

      for i in 0..64 {
        let (mut f, g) = match i {
          0..=15 => ((b & c) | (!b & d), i),
          16..=31 => ((d & b) | (!d & c), (5 * i + 1) % 16),
          32..=47 => ((b ^ c ^ d), (3 * i + 5) % 16),
          48..=63 => ((c ^ (b | !d)), (7 * i) % 16),
          _ => unreachable!(),
        };

        f = f.wrapping_add(a).wrapping_add(K[i]).wrapping_add(words[g]);
        a = d;
        d = c;
        c = b;
        b = b.wrapping_add(f.rotate_left(S[i]));
      }

      a0 = a0.wrapping_add(a);
      b0 = b0.wrapping_add(b);
      c0 = c0.wrapping_add(c);
      d0 = d0.wrapping_add(d);
    }

    let mut digest = [0u8; 16];
    digest[..4].copy_from_slice(&a0.to_le_bytes());
    digest[4..8].copy_from_slice(&b0.to_le_bytes());
    digest[8..12].copy_from_slice(&c0.to_le_bytes());
    digest[12..].copy_from_slice(&d0.to_le_bytes());

    digest
  }

  fn bytes_to_string(bytes: [u8; 16]) -> String {
    bytes
      .iter()
      .map(|b| format!("{:02x}", b))
      .collect::<String>()
  }
}
