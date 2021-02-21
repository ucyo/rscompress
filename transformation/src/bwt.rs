use std::str;
use suffix;
use crate::{Transform, TransformError};
use log::debug;

/// Burrow-Wheeler transformation
///
/// Implementation of the Burrow-Wheeler Transformation as
/// described [here](https://en.wikipedia.org/wiki/Burrows%E2%80%93Wheeler_transform).
///
/// # Algorithm
/// In the following is a rough sketch of the algorithm and its inner workings.

/// ## ▶ Transformation
/// The transformation process involves three steps:
///
/// 1. Create a `N x N` matrix with rotations of data `d`, where `N` is the length of `d`
/// 2. Sort the rows of this matrix
/// 3. Output the last characters of each row + the final row index of `d`
///
/// The last characters and the row index is enough information to reproduce `d`.
///
/// ### Implementation
/// Most implementations of the Burrow-Wheeler Transform use
/// [Suffix Arrays](https://en.wikipedia.org/wiki/Suffix_array) for generating the
/// transformations. A suffix appears often in concatenation with strings and substrings.
/// A substring is a string appearing in the original string.
/// While `pp`, `ppl`, or `le` are all a substring of `apple`; `ale` does not appear in `apple`.
/// If the substring appears at the beginning of the word, it is called a *prefix* e.g. `a`, `ap`, or `app`.
/// If it appears at the end of the word, it is called a *suffix* e.g. `e`, `le`, or `ple`.
/// This definition of prefix and suffix are also being used in context of byte vectors.
///
/// Suffix Arrays are sorted matrices of all suffixes of the original data.
/// Often they are represented by their row indices vector of the length of the original data.
/// The following example shows the sorted suffix indices.
///
/// Example
/// ```text
/// apple$ > $, apple, e, le, ple, pple > [5, 0, 4, 3, 2, 1]
/// ```
///
/// Some implementations of suffix arrays also include an end character often symbolized using `$`.
/// This can, but must not be included for the Burrow Wheeler Transform.
/// From this array the last chars and the index position need to be calculated.
///
/// The latter is simply the position of `0` in the suffix indices vector.
/// In this it is at index position `1`. The former is a bit trickier.
/// What is needed is the last character in a rotated suffix list.
/// The sorted suffix list is given above. For the BWT algorithm these words need to be filled
/// with the actual word, to the length of the original data.
///
/// ```text
/// $, apple, e, le, ple, pple > $apple, apple$, e$appl, le$app, ple$ap, pple$a > [e, $, l, p, p, a]
/// ```
/// The kean reader might have observed that the last letter
/// is always the one previous in the original word.
/// Therefore the last characters of `[5, 0, 4, 3, 2, 1]` are `[4, -1, 3, 2, 1, 0]`.
/// Since the special character `$` is not important the final vector information is `[e, l, p, p, a]` and `1`
/// for the index position.
///
/// ## ◀ Reverse
///
/// # Example
/// ```rust
/// use rscompress_transformation::BurrowWheeler;
/// ```
#[derive(Debug)]
pub struct BurrowWheeler {
    ix: Option<usize>,
}

impl BurrowWheeler {
    pub fn new() -> Self {
        BurrowWheeler { ix: None }
    }
    pub fn reset(&mut self) {
        self.ix = None
    }
}

impl Default for BurrowWheeler {
    fn default() -> Self {
        Self::new()
    }
}

impl Transform for BurrowWheeler {
    /// Transformation of the initial source data
    fn transform(&mut self, source: &[u8]) -> Result<Vec<u8>, TransformError> {
        if source.is_empty() {
            return Err(TransformError::EmptyBufferError)
        }
        debug!("{:?}", source);
        let mut result = Vec::with_capacity(source.len());
        let temp = str::from_utf8(source).unwrap();
        let s = suffix::SuffixTable::new(temp);
        let table = s.table();
        debug!("Suffixtable: {:?} ({})", table, table.len());
        for ix in table.iter() {
            if *ix as usize == 0 {
                let val = source[source.len() - 1];
                result.push(val);
        } else {
                let val = source[*ix as usize - 1];
                result.push(val);
        }
        }
        self.ix = table.iter().position(|&x| x==0);
        debug!("Suffixtable Index Position: {:?}", self.ix);
        Ok(result)
    }
    /// Reversing the initial transformation
    fn reverse(&mut self, source: &[u8]) -> Result<Vec<u8>, TransformError> {
        let mut sorted = source.to_vec();
        sorted.sort_unstable();
        let mut counts = vec![0usize; sorted.len()];

        let mut counter = 0;
        let mut last_letter = sorted.first().unwrap();
        let mut ix: usize = 1;
        for sym in sorted[1..].iter() {
            if sym == last_letter {
                counter += 1;
            } else {
                counter = 0;
                last_letter = sym;
            }
            counts[ix] = counter;
            ix += 1;
        }

        debug!("Source: {:?}", source);
        debug!("Sorted: {:?}", sorted);
        debug!("Counts: {:?}", counts);
        debug!("Self: {:?}", self);
        let mut result: Vec<u8> = Vec::with_capacity(source.len());
        let tmp = str::from_utf8(source).unwrap();
        let suf = suffix::SuffixTable::new(tmp);
        for _ in 0..source.len() {
            let reversed = sorted[self.ix.unwrap()];
            let c = counts[self.ix.unwrap()];
            let ff = [reversed;1];
            let utf_reversed = str::from_utf8(&ff).unwrap();
            debug!("Search {:?}th letter of {:?} ({:?})", c + 1, reversed, utf_reversed);
            result.push(reversed);
            let mut pos = suf.positions(utf_reversed).to_vec();
            pos.sort_unstable();
            debug!("Positions {:?}", pos);
            self.ix = Some(pos[c] as usize);
            debug!("New ix: {:?}", self.ix);
        }
        Ok(result)

    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{random_roundtrip, reverse, roundtrip, transform};

    #[test]
    fn test_easy_transforms() {
        transform::<BurrowWheeler>("banana".as_bytes(), "nnbaaa".as_bytes());
        transform::<BurrowWheeler>("compressioncode".as_bytes(), "neodrsooccimpse".as_bytes());
    }

    #[test]
    fn test_easy_reverse() {
        reverse::<BurrowWheeler>("neodrsooccimpse".as_bytes(), "compressioncode".as_bytes());
    }

    #[test]
    fn test_easy_roundtrip() {
        roundtrip::<BurrowWheeler>("compressioncode".as_bytes());
    }
    #[test]
    fn test_random_roundtrip() {
        random_roundtrip::<BurrowWheeler>(100);
        random_roundtrip::<BurrowWheeler>(100);
        random_roundtrip::<BurrowWheeler>(100);
        random_roundtrip::<BurrowWheeler>(100);
        random_roundtrip::<BurrowWheeler>(100);
    }
}
