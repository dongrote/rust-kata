use std::collections::VecDeque;

fn bytes_hash(b: &[u8], q: u64) -> u64 {
  let base: u64 = 10;
  let mut sum: u64 = 0;

  let mut i: u32 = 0;
  for b in b.iter().rev() {
    let coefficient = base.pow(i);
    sum = u64::wrapping_add(sum, coefficient * u64::from(*b));
    i = i + 1;
  }

  sum % q
}

fn str_hash(s: &str, q: u64) -> u64 {
  bytes_hash(s.as_bytes(), q)
}

pub fn search_stream(needle: &str, haystack: &mut (impl std::io::Read + std::io::Seek)) -> Vec<usize> {
  let q = 52;
  let mut match_offset = 0;
  let mut offsets: Vec<usize> = vec![];
  let siglen = needle.len();
  let needle_hash = str_hash(needle, q);
  let mut buf: Vec<u8> = Vec::new();
  buf.resize(siglen, 0);
  match haystack.read(&mut buf) {
    Ok(rxbytes) => {
      if rxbytes < buf.len() {
        return offsets;
      }

      let mut bytes: VecDeque<u8> = VecDeque::from(buf.clone());
      loop {
        let mut byte_array: [u8; 1] = [0];
        {
          let bytes_contig = bytes.make_contiguous();
          let buf_hash = bytes_hash(&bytes_contig, q);
          if buf_hash == needle_hash {
            // println!("hash hit at {}: '{}'", buf_idx, s);
            if bytes_contig == needle.as_bytes() {
              println!("match at {}: '{:?}'", match_offset, bytes);
              offsets.push(match_offset);
            }
          }
        }

        bytes.pop_front().unwrap_or_default();
        match_offset += 1;
        match haystack.read(&mut byte_array) {
          Ok(byte_read_count) => {
            if byte_read_count == 0 {
              break;
            }

            bytes.push_back(byte_array[0]);
          },
          Err(e) => {
            eprintln!("byte read error: {}", e);
            break;
          },
        }
      }
    },
    Err(err) => eprintln!("initial read failed: {}", err),
  }

  offsets
}



#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_str_hash() {
    assert_eq!(str_hash("needle", 13), 6);
    assert_ne!(str_hash("needle", 13), str_hash("haystack", 13));
  }
}
