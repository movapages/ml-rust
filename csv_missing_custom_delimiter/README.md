&#8743;  [Start README](../README.md)

#### Polars: Counting Missing Values in CSV

Technically, meaning of _missing_ may be slightly different in some cases, but here
it means an empty field in CSV file row.

The Rust snipped presented here has a deliberately corrupt CSV file with randomly
emptied fields. So, we must replace _custom_ field separator in the source file 
from ';' with ',', and then process a newly written CSV with _Polars_ library 
standard methods.

> NB: ***more*** sophisticated CSV file fields manipulations with custom delimiters, quoted field values, etc. should be carried out with a crates like `csv`.