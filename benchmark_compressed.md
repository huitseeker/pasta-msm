# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [GPU](#gpu)

## Benchmark Results

### GPU

|                            | `random`                  | `lurkrs`                         | `lurkrs compressed`              |
|:---------------------------|:--------------------------|:---------------------------------|:-------------------------------- |
| **`#1, 7941351 scalars`**  | `121.32 ms` (✅ **1.00x**) | `547.39 ms` (❌ *4.51x slower*)   | `19.74 ms` (🚀 **6.15x faster**)  |
| **`#2, 9699051 scalars`**  | `141.87 ms` (✅ **1.00x**) | `145.90 ms` (✅ **1.03x slower**) | `38.80 ms` (🚀 **3.66x faster**)  |
| **`#5, 7941351 scalars`**  | `117.80 ms` (✅ **1.00x**) | `550.36 ms` (❌ *4.67x slower*)   | `18.25 ms` (🚀 **6.45x faster**)  |
| **`#6, 9699051 scalars`**  | `137.59 ms` (✅ **1.00x**) | `321.07 ms` (❌ *2.33x slower*)   | `52.13 ms` (🚀 **2.64x faster**)  |
| **`#9, 7941351 scalars`**  | `120.01 ms` (✅ **1.00x**) | `555.16 ms` (❌ *4.63x slower*)   | `17.04 ms` (🚀 **7.04x faster**)  |
| **`#10, 9699051 scalars`** | `144.52 ms` (✅ **1.00x**) | `345.90 ms` (❌ *2.39x slower*)   | `61.81 ms` (🚀 **2.34x faster**)  |
| **`#13, 7941351 scalars`** | `120.17 ms` (✅ **1.00x**) | `554.94 ms` (❌ *4.62x slower*)   | `17.12 ms` (🚀 **7.02x faster**)  |
| **`#14, 9699051 scalars`** | `142.90 ms` (✅ **1.00x**) | `463.73 ms` (❌ *3.25x slower*)   | `57.67 ms` (🚀 **2.48x faster**)  |
| **`#17, 7941351 scalars`** | `118.52 ms` (✅ **1.00x**) | `557.10 ms` (❌ *4.70x slower*)   | `16.43 ms` (🚀 **7.21x faster**)  |
| **`#18, 9699051 scalars`** | `144.78 ms` (✅ **1.00x**) | `560.78 ms` (❌ *3.87x slower*)   | `65.20 ms` (🚀 **2.22x faster**)  |
| **`#21, 7941351 scalars`** | `118.18 ms` (✅ **1.00x**) | `555.62 ms` (❌ *4.70x slower*)   | `17.44 ms` (🚀 **6.78x faster**)  |
| **`#22, 9699051 scalars`** | `147.36 ms` (✅ **1.00x**) | `610.48 ms` (❌ *4.14x slower*)   | `65.04 ms` (🚀 **2.27x faster**)  |
| **`#25, 7941351 scalars`** | `119.47 ms` (✅ **1.00x**) | `555.26 ms` (❌ *4.65x slower*)   | `16.50 ms` (🚀 **7.24x faster**)  |
| **`#26, 9699051 scalars`** | `142.61 ms` (✅ **1.00x**) | `675.52 ms` (❌ *4.74x slower*)   | `66.13 ms` (🚀 **2.16x faster**)  |
| **`#29, 7941351 scalars`** | `119.86 ms` (✅ **1.00x**) | `554.61 ms` (❌ *4.63x slower*)   | `16.63 ms` (🚀 **7.21x faster**)  |
| **`#30, 9699051 scalars`** | `146.12 ms` (✅ **1.00x**) | `698.63 ms` (❌ *4.78x slower*)   | `67.42 ms` (🚀 **2.17x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

