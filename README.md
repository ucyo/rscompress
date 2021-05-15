<h1 align='center'>rscompress</h1>

<p align=center>
  A compression library in Rust with focus on scientific data. <br><br>
  <img src="https://raw.githubusercontent.com/ucyo/rscompress/master/assets/logo.svg" height="368"/>
</p>

## Disclaimer
This is a rewrite and merge of several compression algorithms developed during my time as a phd student:

- https://github.com/ucyo/huffman
- https://github.com/ucyo/pzip-cli
- https://github.com/ucyo/pzip-bwt
- https://github.com/ucyo/pzip-redux
- https://github.com/ucyo/pzip-huffman
- https://github.com/ucyo/pzip
- https://github.com/ucyo/rust-compress
- https://github.com/ucyo/adaptive-lossy-compression
- https://github.com/ucyo/information-spaces
- https://github.com/ucyo/cframework
- https://github.com/ucyo/xor-and-residual-calculation
- https://github.com/ucyo/climate-data-analysis

The dissertation can be downloaded from https://doi.org/10.5445/IR/1000105055

## Architecture

The library is split into one base and four supporting libraries.
The base library orchestrates the supporting libraries.
All compression algorithms follow the same basic structure:

1. Decorrelate data using transformations
2. Approximate the data, if lossy compression is needed
3. Code the data

Additionally, check if each step executed as expected.

```
                   +----------------+      lossless      +----------+
                   |                |                    |          |
Start   +------>   | Transformation |   +------------>   |  Coding  |   +------>   End
                   |                |                    |          |
                   +----------------+                    +----------+

                           +                                   ^
                           |                                   |
                           |  lossy                            |
                           |                                   |
                           v                                   |
                                                               |
                   +---------------+                           |
                   |               |                           |
                   | Approximation |  +------------------------+
                   |               |
                   +---------------+
```
This library will follow the same principles.

### Transformations
Transformations are algorithms which represent the same information using a different alphabet.
Good transformation algorithms eliminate redundant information in the data.
A mathematical function can be seen as a transformation of a series of data.
The series `1 1 2 3 5 8 13 21 ..` can expressed as `f(x) = f(x-1) + f(x-2)`.
We mapped the information represented in alphabet A (integers) to an alphabet B (letters + integers) which is more compact.
It is important to note that all transformations must have two properties:

- Applying a transformation algorithm to data, does not loose information.
- All transformation algorithms are reversible, such that the original representation can be reconstructed from the new alphabet.

### Approximations
Approximations are algorithms which loose information for the sake of better compression.
Given a threshold `theta` (this can be absolute or relative), the algorithm maps the data from alphabet A to B with an information lose within the expected threshold.
An example for an approximation is the `~=` operator known from primary school e.g. `1/3 ~= 0.3`.
Approximations have the following properties:

- Applying an approximation algorithm to data, results in information loss.
- Approximation algorithms are not reversible.
- The information loss is guaranteed to be within the threshold `theta`

### Codings
Codings are algorithms where the actual compression happens.
The information is being saved on disk as compact as possible.
Examples are [Huffman](https://en.wikipedia.org/wiki/Huffman_coding) or
[Arithmetic](https://en.wikipedia.org/wiki/Arithmetic_coding) coding.

### Checksums
Checksums are algorithms to check the integrity of the data at each step e.g. [Adler-32](https://en.wikipedia.org/wiki/Adler-32).
