# acc_csv2bop
CLI tool that parses a csv spreadsheet into a bop.json for use as a Custom BOP (Balance Of Performance) in Assetto Corsa Competizione

## Usage
```
Usage: acc_csv2bop.exe [OPTIONS]

Options:
  -b, --ballast <BALLAST>  ballast csv file (required)
  -o, --output <OUTPUT>    output file, defaults to bop.json
  -v, --verbose            verbose logging, use to make sure it parsed correctly
      --list-tracks        list all tracks and exit
      --list-carmodels     list all carmodel ids and exit
  -h, --help               Print help
  -V, --version            Print version
```

The csv has to be in this format (sample can be found in ``/samples/``):
| |track_id (like *brands_hatch*)|*further tracks...*|
|:-|:------------:|:---:|
|car_model_id (like *50* for the A110)| weight (integer)|...|
|*further cars*|...|...|
  
You can use ``--list-carmodels`` and ``--list-tracks`` to find the values to set in those columns.  
Don't use any special characters (like ", ', etc) to surround any values and names.  
Empty weight cells will be read as 0.  
You can use spaces instead of underscores and any captitalization for the track_id, but you have to refer to the track still with the correct name.  
  
The generated bop.json will not include any entries that don't change any BOP.  

## "Planned" Features
- restrictor csv
- allowing to use car names instead of having to look up car model ids
- exporting json to csv

## Building
rustup (v1.70.0 or higher) with cargo required:
```
cargo build
```

If you want to build another tool for reading and processing bop.json files, then you can use ``/src/data.rs``, it contains the structs for bop.json  