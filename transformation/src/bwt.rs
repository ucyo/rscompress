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

fn fix_suffix_array(sarr: &mut [u32], source: &[u8]) {
    let positions = get_indices_to_be_fixed(sarr.as_ref(), source);
    for (pos, length) in positions.iter() {
        fix_suffix_array_at_position(sarr, *pos, *length)
    }
}

fn get_indices_to_be_fixed(sarr: &[u32], source: &[u8]) -> Vec<(usize, usize)>{
    let mut first_column: Vec<u8> = sarr.iter().map(|&x| source[x as usize]).collect();
    first_column.sort_unstable();

    let mut result: Vec<(usize, usize)> = Vec::new();
    let (v, first_column) = first_column.split_first().unwrap();
    let mut value = *v;
    let mut c = 0usize;
    for (i, &x) in first_column.iter().enumerate() {
        println!("F[{:?}]={:?} (count: {:?}, last_value: {:?})", i, x, c, value);
        if x == value {
            c += 1
        } else {
            if c != 0 {result.push((i+1-c, c+1))}
            c = 0;
            value = x;
        }
    }
    if c != 0 {result.push((first_column.len() - c, c + 1))}
    result

}


impl Transform for BurrowWheeler {
    /// Transformation of the initial source data
    fn transform(&mut self, source: &[u8]) -> Result<Vec<u8>, TransformError> {
        if source.is_empty() {
            return Err(TransformError::EmptyBufferError);
        }
        debug!("{:?}", source);
        let (k, mut sarr) = SuffixArray::new(source).into_parts();
        println!("Ori Suffixtable: {:?} ({}) {:?}", sarr, sarr.len(), k);
        sarr.remove(0);
        self.ix = sarr.iter().position(|&x| x == 0);
        debug!("000 Suffixtable: {:?} ({})", sarr, sarr.len());
        sarr[self.ix.unwrap()] = source.len() as u32;
        debug!("Rep Suffixtable: {:?} ({})", sarr, sarr.len());
        for x in sarr.iter_mut() {
            *x = *x - 1;
        }
        let result: Vec<u8> = sarr.iter().map(|x| source[*x as usize]).collect();
        println!("OLD last column: {:?} {:?}", result, sarr);
        fix_suffix_array(&mut sarr, source);
        let result: Vec<u8> = sarr.iter().map(|x| source[*x as usize]).collect();
        println!("NEW last column: {:?} {:?}", result, sarr);
        debug!("{:?} {:?}", result, source);
        debug!("Suffixtable Index Position: {:?}", self.ix);
        Ok(result)
    }
    /// Reversing the initial transformation
    fn reverse(&mut self, source: &[u8]) -> Result<Vec<u8>, TransformError> {
        // generate sorted vector
        let mut sorted = source.to_vec();
        sorted.sort_unstable();

        // generate counts vector
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

        println!("Source: {:?}", source);
        println!("Sorted: {:?}", sorted);
        println!("Counts: {:?}", counts);
        println!("Self: {:?}", self);
        let mut result: Vec<u8> = Vec::with_capacity(source.len());
        let suf = SuffixArray::new(source);
        for _ in 0..source.len() {
            let reversed = sorted[self.ix.unwrap()];
            result.push(reversed);
            let c = counts[self.ix.unwrap()];
            let ff = [reversed; 1];
            println!("Search {:?}th letter of {:?}", c + 1, reversed,);
            let mut pos = suf.search_all(&ff).to_vec();
            pos.sort_unstable();
            println!("Positions {:?}", pos);
            self.ix = Some(pos[c] as usize);
            println!("New ix: {:?}", self.ix);
        }
        Ok(result)
    }
}

/// Fixes `errors' made by the suffix array
///
/// The current crate uses a special sign to show the end of a word.
/// This sign is often treated like the lowest value in lexicographical order.
/// What follows out of this is, that the rotations considered for the BWT
/// are not fully compared.
/// This function fixes this issue
///
/// # Notes
/// Given the vector `[123, 139, 39, 62, 139]` the
/// suffix array returns `[1, 2, 4, 3, 0]` which maps to [139, 39, 139, 62, 123].
/// The first column is `[39, 62, 123, 139, 139]`.
/// The correct rotational matrix is though the following:
/// ```text
/// [123, 139,  39,  62, 139],
/// -------------------------
/// [ 39,  62, 139, 123, 139],
/// [ 62, 139, 123, 139,  39],
/// [123, 139,  39,  62, 139], with original vector @ ix = 2
/// [139,  39,  62, 139, 123],
/// [139, 123, 139,  39,  62],
/// ```
/// This `feature' is due to the special symbol considered
/// in the implementation of the suffix array construction.
///
/// # Solution
/// The solution is to look at the indices `x+1` and their position in the SA.
/// And based on this reorder the original SA.
///
/// # Example
/// ```no_run
/// let data     = [123, 139, 39, 62, 139];
/// let expected = [139, 39, 139, 123, 62];
///
/// let mut sa: [usize; 5] = [1, 2, 4, 3, 0];
/// fix_suffix_array(&mut sa, 3, 2);
///
/// let result: Vec<u8> = sa.iter().map(|&k| data[k]).collect();
/// assert_eq!(result, expected);
/// ```
fn fix_suffix_array_at_position(sa: &mut [u32], pos: usize, length: usize) {
    let mm: Vec<u32> = sa.to_vec().into_iter().collect();
    println!("checking {:?} {} {}", sa, pos, length);
    sa[pos..pos + length]
        .sort_by_cached_key(|k| mm.iter().position(|&x| x ==( (k + 1) % mm.len() as u32)).unwrap());
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
    fn test_fix_suffix() {
        let data = [123, 139, 39, 62, 139];
        let expected = [139, 39, 139, 123, 62];
        let mut sa: [u32; 5] = [1, 2, 4, 3, 0];
        fix_suffix_array_at_position(&mut sa, 3, 2);
        let result: Vec<u8> = sa.iter().map(|&k| data[k as usize]).collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_need_to_be_fixed() {
        let data = [123, 139, 39, 62, 139];
        let sa: [u32; 5] = [1, 2, 4, 3, 0];
        let result = get_indices_to_be_fixed(&sa, &data);
        assert_eq!(result, vec![(3,2)])
    }

    #[test]
    fn test_counts() {
        let mut data: [u8; 7] = [123, 139, 39, 62, 139, 139, 139];
        data.sort();

        let counts = get_counts(&data);
        assert_eq!(counts, [0, 0, 0, 0, 1, 2, 3])
    }

    #[test]
    fn test_easy_transforms() {
        transform::<BurrowWheeler>(&[123, 139, 39, 62, 139], &[139, 39, 139, 62, 123]);
        transform::<BurrowWheeler>(&[230, 183, 108, 102, 230], &[108, 183, 230, 102, 230]);
        transform::<BurrowWheeler>("banana".as_bytes(), "nnbaaa".as_bytes());
        transform::<BurrowWheeler>("compressioncode".as_bytes(), "neodrsooccimpse".as_bytes());
    }

    // Simple reverse tests will not be working, since information needs to
    // be transmitted from each tranformed block to successfully reverse it.
    // TODO: Add tests for multi step transformations.

    #[test]
    fn test_easy_roundtrip() {
        // roundtrip::<BurrowWheeler>(&[123, 139, 39, 62, 139]);
        roundtrip::<BurrowWheeler>(&[230, 183, 108, 102, 230]);
        roundtrip::<BurrowWheeler>("compressioncode".as_bytes());
        roundtrip::<BurrowWheeler>("apple".as_bytes());
        roundtrip::<BurrowWheeler>("banana".as_bytes());
    }
    #[test]
    fn test_random_roundtrip() {
        random_roundtrip::<BurrowWheeler>(100, 5);
        random_roundtrip::<BurrowWheeler>(100, 5);
        random_roundtrip::<BurrowWheeler>(100, 5);
        random_roundtrip::<BurrowWheeler>(100, 5);
        random_roundtrip::<BurrowWheeler>(100, 5);
    }
}
