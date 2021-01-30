# rscompress
A library for compression of floating-point data.

## Usage

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
