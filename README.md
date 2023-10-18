# acc_csv2bop
CLI tool that parses a csv spreadsheet into a bop.json (and vice versa) for use as a Custom BOP (Balance Of Performance) in Assetto Corsa Competizione

## Usage
```
Usage: acc_csv2bop.exe [OPTIONS]

Options:
  -b, --ballast <BALLAST>        ballast csv file (required)
  -r, --restrictor <RESTRICTOR>  restrictor csv file (optional)
  -o, --output <OUTPUT>          output file, defaults to bop.json
  -j, --json <JSON>              A bop.json to parse to a CSV file
  -v, --verbose                  verbose logging, use to make sure it parsed correctly
      --list-tracks              list all tracks and exit
      --list-carmodels           list all carmodel ids and exit
  -h, --help                     Print help
  -V, --version                  Print version
```

The csv has to be in this format (samples can be found in ``/samples/``):
| |track_id (like *brands_hatch*)|*further tracks...*|
|:-|:------------:|:---:|
|car_model | weight (integer)|...|
|*further cars...*|...|...|
  
You can use ``--list-carmodels`` and ``--list-tracks`` to find the values to set in those fields.  
  
``car_model`` can either be the carmodelid (like *50* for the Alpine A110), but you can also use the car name.  
Be aware that it breaks the name at each space and uses those as tokens to see which car name contains those FIRST.  
See ``--list-carmodels`` to see the order and words to match. When in doubt just use the model id.  
  
Additionally, don't use any special characters (like ", ', etc) to surround any values and names.  
Empty weight cells will be read as 0.  
You can use spaces instead of underscores and any captitalization for the track_id, but you have to refer to the track still with the correct name.  
  
Parsing a Restrictor csv requires a Ballast csv, though both are not required to contain the same tracks and cars (so you can just have a nearly empty ballast file if you only want to apply Restrictors).  
  
The generated bop.json will not include any entries that don't change any BOP.  

## "Planned" Features
- exporting restrictor values from bop json to csv

## Building
rustup (v1.70.0 or higher) with cargo required:
```
cargo build
```

If you want to build another tool for reading and processing bop.json files, then you can use ``/src/data.rs``, it contains the structs for bop.json  