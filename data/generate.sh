#!/usr/bin/env bash

echo "Generate uniform integer data"
python generate.py u64 none none 1250000 numerical/u64_uniform7.raw uniform 42
python generate.py u32 none none 2500000 numerical/u32_uniform7.raw uniform 42
python generate.py u16 none none 5000000 numerical/u16_uniform7.raw uniform 42
python generate.py u8 none none 10000000 numerical/u8_uniform7.raw uniform 42
python generate.py i8 none none 10000000 numerical/i8_uniform7.raw uniform 42
python generate.py i16 none none 5000000 numerical/i16_uniform7.raw uniform 42
python generate.py i32 none none 2500000 numerical/i32_uniform7.raw uniform 42
python generate.py i64 none none 1250000 numerical/i64_uniform7.raw uniform 42

echo "Generate gaussian floating-point data"
python generate.py f64 256.0 25.6 1250000 numerical/f64_256normal7.raw normal 42
python generate.py f32 256.0 25.6 2500000 numerical/f32_256normal7.raw normal 42
python generate.py f64 1.0 10.0 1250000 numerical/f64_1normal7.raw normal 42
python generate.py f32 1.0 10.0 2500000 numerical/f32_1normal7.raw normal 42
