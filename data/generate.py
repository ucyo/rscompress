from array import array
import random
import sys

TYPES = {
    "f64": "d", "f32": "f",
    "u64": "Q", "u32": "I", "u16": "H", "u8": "B",
    "i64": "q", "i32": "i", "i16": "h", "i8": "b",
}

def main():
    dtype = sys.argv[1]
    assert dtype in TYPES.keys(), f"Unknown type {dtype}"
    param1 = int(sys.argv[2])
    param2 = int(sys.argv[3])
    size = int(sys.argv[4])
    output = sys.argv[5]
    distr = sys.argv[6]
    assert distr in ["normal", "uniform"], f"Unknown distribution {distr}"
    if len(sys.argv) > 7:
        seed = sys.argv[7]
        random.seed(seed)

    if distr == "uniform":
        data = generate_uniform(param1, param2, size, dtype)
    else:
        data = generate_gaussian(param1, param2, size, dtype)
    write_to_disk(data, output)
    print(data)

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
