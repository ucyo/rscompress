# rscompress
A library for compression of floating-point data.

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
3. Encode the data

Additionally, check if each step executed as expected.

```
                   +----------------+      lossless      +----------+
                   |                |                    |          |
Start   +------>   | Transformation |   +------------>   | Encoding |   +------>   End
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
