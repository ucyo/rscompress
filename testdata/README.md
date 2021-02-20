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

**Examples:**

Generate f32 data with mean @ 256 and std @ 25.6 with 10_000_000 datapoints
```
python generate.py f32 256.0 25.6 2500000 numerical/f32_256normal7.raw normal 42
```

Generate u32 data with whole possible range with 10_000_000 datapoints
```
python generate.py u32 none none 2500000 numerical/u32_uniform7.raw uniform 42
```
