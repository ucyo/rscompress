use std::collections::HashMap;

use crate::{Transform, TransformError};
use log::debug;
use suffix_array::SuffixArray;

/// Burrow-Wheeler transformation
///
/// Implementation of the Burrow-Wheeler Transformation as
/// described [here](https://en.wikipedia.org/wiki/Burrows%E2%80%93Wheeler_transform).
///
/// # Algorithm
/// In the following is a rough sketch of the algorithm and its inner workings.
///
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
/// apple, e, le, ple, pple > apple, eappl, leapp, pleap, pleap, pplea > [e, l, p, p, a]
/// ```
/// The kean reader might have observed that the last letter
/// is always the one previous in the original word.
/// Therefore the last characters of `[5, 4, 4, 3, 2, 1]` are `[4, -1, 3, 2, 1, 0]`.
/// Since the special character `$` is not important the final vector information is `[e, l, p, p, a]` and `1`
/// for the index position.
///
/// ## ◀ Reverse
/// TODO: Improve explanation
/// The reverse algorithm of BWT uses three vectors of length `N` with `N` being the number of data vector.
/// The first vector is the actual vector to be reversed i.e. last characters of the suffix array.
/// The second vector is the first vector sorted.
/// The third and last vector is a count of the second vector.
/// The third vector saves the information on the position of the byte.
/// The following is an example for the `apple` string as above.
///
/// ```text
/// [e, l, p, p, a] > [1, 1, 1, 2, 1]
/// ```
///
///
/// [e, l, p, p, a]
/// [a, e, l, p, p]
/// [1, 1, 1, 2, 1]
/// i: 0,
/// The first output is the letter at the index position of the second vector.
/// In the above example this is pos `1` in `[$, a, e, l, p, p]` and therefore  `a`.
/// The second output is at the index position of the `n`th occurence of the letter just output in the first vector.
/// The `n`th occurence is described by the third vector.
/// In this case it is the first occurence of `a` in vector 1.
/// This is at the last position. Therefore the output is `p`.
/// After that is the first occurence of the letter `p` in vector 1.
/// This is at position 2.
///
/// # Example
/// ```rust
/// use rscompress_transformation::BurrowWheeler;
/// ```
#[derive(Debug)]
pub struct BurrowWheeler {
    ix: Option<usize>,
    size: usize,
}

impl BurrowWheeler {
    pub fn new() -> Self {
        BurrowWheeler { ix: None, size: 0 }
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
        self.size = source.len();
        if source.is_empty() {
            return Err(TransformError::EmptyBufferError);
        }
        debug!("Input {:?}", source);
        let (_, mut sarr) = SuffixArray::new(source).into_parts();
        debug!("SARR {:?}", sarr);
        self.ix = sarr.iter().position(|&x| x == 0);
        sarr[self.ix.unwrap()] = sarr.len() as u32 + 1;
        let mut last_column: Vec<_> = sarr
            .iter()
            .map(|&x| *source.get((x - 1) as usize).unwrap_or(&0u8))
            .collect();
        last_column.remove(self.ix.unwrap());
        Ok(last_column)
    }
    /// Reversing the initial transformation
    fn reverse(&mut self, source: &[u8]) -> Result<Vec<u8>, TransformError> {
        // generate sorted vector
        let mut sorted = source.to_vec();
        sorted.sort_unstable();

        // generate counts vector
        let counts = get_counts(&sorted);

        // generate mapping
        let mapping = get_position_map(source);

        debug!("Source: {:?}", source);
        debug!("Sorted: {:?}", sorted);
        debug!("Counts: {:?}", counts);
        debug!("Self: {:?}", self);
        let mut result: Vec<u8> = vec![0u8; self.size];
        let mut pos = self.ix.unwrap() - 1;
        for r in result.iter_mut() {
            let reversed = sorted[pos];
            *r = reversed;
            let c = counts[pos];
            let ix = *mapping.get(r).unwrap().get(c).unwrap();
            pos = ix - (ix != 0 && ix < self.ix.unwrap()) as usize;
            debug!("{:?}", r);
        }
        Ok(result)
    }
}


fn get_position_map(data: &[u8]) -> HashMap<u8, Vec<usize>> {
    let mut result: HashMap<u8, Vec<usize>> = HashMap::new();
    for (i, d) in data.iter().enumerate() {
        let v = result.get_mut(d);
        match v {
            Some(v) => v.push(i),
            None => {
                let k = vec![i];
                let _ = result.insert(*d, k);},
        };
    }
    result
}


fn get_counts(sorted: &[u8]) -> Vec<usize> {
    let mut v = *sorted.first().unwrap();
    let mut counter = 0;

    let mut result: Vec<usize> = sorted[1..]
        .iter()
        .map(|&x| {
            if x == v {
                counter += 1
            } else {
                counter = 0;
                v = x
            }
            counter
        })
        .collect();

    result.insert(0, 0);
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{random_roundtrip, roundtrip, transform};

    #[test]
    fn test_counts() {
        let mut data: [u8; 7] = [123, 139, 39, 62, 139, 139, 139];
        data.sort_unstable();

        let counts = get_counts(&data);
        assert_eq!(counts, [0, 0, 0, 0, 1, 2, 3])
    }

    #[test]
    fn test_easy_transforms() {
        transform::<BurrowWheeler>(&[123, 139, 39, 62, 139], &[139, 139, 39, 62, 123]);
        transform::<BurrowWheeler>(&[230, 183, 108, 102, 230], &[230, 108, 183, 230, 102]);
        transform::<BurrowWheeler>("banana".as_bytes(), "annbaa".as_bytes());
        transform::<BurrowWheeler>("compressioncode".as_bytes(), "enodrsooccimpse".as_bytes());
    }

    // Simple reverse tests will not be working, since information needs to
    // be transmitted from each tranformed block to successfully reverse it.
    // TODO: Add tests for multi step transformations.

    #[test]
    fn test_easy_roundtrip() {
        roundtrip::<BurrowWheeler>(&[34, 10, 0, 206, 40]);
        roundtrip::<BurrowWheeler>(&[123, 139, 39, 62, 139]);
        roundtrip::<BurrowWheeler>(&[230, 183, 108, 102, 230]);
        roundtrip::<BurrowWheeler>("compressioncode".as_bytes());
        roundtrip::<BurrowWheeler>("apple".as_bytes());
        roundtrip::<BurrowWheeler>("banana".as_bytes());
    }
    #[test]
    fn test_random_roundtrip() {
        random_roundtrip::<BurrowWheeler>(100, 10_000);
        random_roundtrip::<BurrowWheeler>(100, 10_000);
        random_roundtrip::<BurrowWheeler>(100, 10_000);
        random_roundtrip::<BurrowWheeler>(100, 10_000);
        random_roundtrip::<BurrowWheeler>(100, 10_000);
    }
}
