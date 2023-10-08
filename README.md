# acc_csv2bop
CLI tool that parses a csv spreadsheet into a bop.json for use as a custom bop in Assetto Corsa Competizione

## Usage
```
Usage: acc_csv2bop.exe [OPTIONS] --ballast <BALLAST>

Options:
  -b, --ballast <BALLAST>  ballast csv file
  -o, --output <OUTPUT>    output file, defaults to bop.json
  -v, --verbose            verbose logging, use to make sure it parsed correctly
  -h, --help               Print help
  -V, --version            Print version
```

The csv has to be in this format (sample can be found in ``/samples/``):
| |track_id (like "brands_hatch")|*further tracks...*|
|:-|:------------:|:---:|
|car_model_id (like 50 for the A110)| weight (integer)|...|
|*further cars*|...|...|

## Building:
rustup (v1.70.0 or higher) with cargo required:
```
cargo build
```
