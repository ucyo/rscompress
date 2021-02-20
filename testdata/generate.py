from array import array
import random
import sys

TYPES = {
    "f64": "d", "f32": "f",
    "u64": "Q", "u32": "I", "u16": "H", "u8": "B",
    "i64": "q", "i32": "i", "i16": "h", "i8": "b",
}

MINISMAXIS = {
    # Using random seem to be broken for floats.
    # The returned values are too often close to the max value
    # "f64" : (sys.float_info.min, sys.float_info.max),
    # "f32" : (1.175494351e-38, 3.402823466e+38),
    "u64" : (0, 2**64 - 1),
    "u32" : (0, 2**32 - 1),
    "u16" : (0, 2**16 - 1),
    "u8"  : (0, 2**8 - 1),
    "i64" : (- 2**32, 2**32 - 1),
    "i32" : (- 2**16, 2**16 - 1),
    "i16" : (- 2**8, 2**8 - 1),
    "i8" : (- 2**4, 2**4 - 1),
}

def main():
    dtype = sys.argv[1]
    assert dtype in TYPES.keys(), f"Unknown type {dtype}"
    distr = sys.argv[6]
    assert distr in ["normal", "uniform"], f"Unknown distribution {distr}"

    if distr == "uniform" and sys.argv[2] == "none":
        sys.argv[2] = MINISMAXIS[dtype][0]
    if distr == "uniform" and sys.argv[3] == "none":
        sys.argv[3] = MINISMAXIS[dtype][1]

    param1 = float(sys.argv[2])
    param2 = float(sys.argv[3])
    size = int(sys.argv[4])
    output = sys.argv[5]
    if len(sys.argv) > 7:
        seed = sys.argv[7]
        random.seed(seed)

    if distr == "uniform":
        data = generate_uniform(param1, param2, size, dtype)
    else:
        data = generate_gaussian(param1, param2, size, dtype)
    write_to_disk(data, output)
    print(len(data))

def write_to_disk(data, filename):
    with open(filename, 'wb') as file:
        data.tofile(file)

def generate_uniform(mini, maxi, size, dtype):
    print(mini, maxi, size, dtype)
    if dtype not in ["f32", "f64"]:
        a = [random.randint(mini, maxi) for x in range(size)]
    else:
        a = [random.uniform(mini, maxi) for x in range(size)]
    return array(TYPES[dtype], a)

def generate_gaussian(mu, sigma, size, dtype):
    assert dtype in ["f32", "f64"], f"Gaussian can only be real-valued, not {dtype}"
    a = [random.gauss(mu, sigma) for x in range(size)]
    return array(TYPES[dtype], a)

if __name__ == '__main__':
    main()
