# Benchmark Data

Data for benchmarking each crate.
Currently there are textual and numerical data with f32 and f64.

##  Textual data

The textual data is from the English Wikipedia page of 2006.
These are the first x bytes of the data.

## Numerical data

Generating the numerical data is a bit more complex.
The dataset must cover numerical and uniform distributions, different datatypes and numerical ranges.
The script [`generate.py`](./generate.py) will take the task in generating testdata for these cases.

**Usage:**
```
python generate.py <dtype> <param 1> <param 2> <size> <output filename> <distribution type>
```
For distribution type `uniform`, the `param 1` will be interpreted as `minium` value.
For distribution type `uniform`, the `param 2` will be interpreted as `maximum` value.
For distribution type `normal`, the `param 1` will be interpreted as `mean` value.
For distribution type `normal`, the `param 2` will be interpreted as `std` value.
