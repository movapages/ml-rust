&#8743;  [Start README](../README.md)

#### Polars: Read in CSV File

Almost everything one may need to start processing source data set is available in Polars. Check the program output below:

```
DF: shape: (8_763, 26)
┌────────────┬─────┬────────┬─────────────┬───┬────────────────┬───────────────┬─────────────────────┬───────────────────┐
│ Patient ID ┆ Age ┆ Sex    ┆ Cholesterol ┆ … ┆ Country        ┆ Continent     ┆ Hemisphere          ┆ Heart Attack Risk │
│ ---        ┆ --- ┆ ---    ┆ ---         ┆   ┆ ---            ┆ ---           ┆ ---                 ┆ ---               │
│ str        ┆ i64 ┆ str    ┆ i64         ┆   ┆ str            ┆ str           ┆ str                 ┆ i64               │
╞════════════╪═════╪════════╪═════════════╪═══╪════════════════╪═══════════════╪═════════════════════╪═══════════════════╡
│ BMW7812    ┆ 67  ┆ Male   ┆ 208         ┆ … ┆ Argentina      ┆ South America ┆ Southern Hemisphere ┆ 0                 │
│ CZE1114    ┆ 21  ┆ Male   ┆ 389         ┆ … ┆ Canada         ┆ North America ┆ Northern Hemisphere ┆ 0                 │
│ BNI9906    ┆ 21  ┆ Female ┆ 324         ┆ … ┆ France         ┆ Europe        ┆ Northern Hemisphere ┆ 0                 │
│ JLN3497    ┆ 84  ┆ Male   ┆ 383         ┆ … ┆ Canada         ┆ North America ┆ Northern Hemisphere ┆ 0                 │
│ GFO8847    ┆ 66  ┆ Male   ┆ 318         ┆ … ┆ Thailand       ┆ Asia          ┆ Northern Hemisphere ┆ 0                 │
│ …          ┆ …   ┆ …      ┆ …           ┆ … ┆ …              ┆ …             ┆ …                   ┆ …                 │
│ MSV9918    ┆ 60  ┆ Male   ┆ 121         ┆ … ┆ Thailand       ┆ Asia          ┆ Northern Hemisphere ┆ 0                 │
│ QSV6764    ┆ 28  ┆ Female ┆ 120         ┆ … ┆ Canada         ┆ North America ┆ Northern Hemisphere ┆ 0                 │
│ XKA5925    ┆ 47  ┆ Male   ┆ 250         ┆ … ┆ Brazil         ┆ South America ┆ Southern Hemisphere ┆ 1                 │
│ EPE6801    ┆ 36  ┆ Male   ┆ 178         ┆ … ┆ Brazil         ┆ South America ┆ Southern Hemisphere ┆ 0                 │
│ ZWN9666    ┆ 25  ┆ Female ┆ 356         ┆ … ┆ United Kingdom ┆ Europe        ┆ Northern Hemisphere ┆ 1                 │
└────────────┴─────┴────────┴─────────────┴───┴────────────────┴───────────────┴─────────────────────┴───────────────────┘

```

> NB: *DF: shape: (8_763, 26)*: "DF" - data frame, "8_763" - src file record count, "26" - number of fields.

This basic example, or basic CSV data processor, is taken from [Polars official documentation](https://docs.pola.rs/user-guide/io/csv/#read-write).

