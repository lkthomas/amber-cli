# amber-cli

## What is this?

A Rust CLI tool to access [Amber energy's customer REST API](https://app.amber.com.au/developers/documentation/).

This tool will return price and usage data in JSON format based on the CLI command issued.

## Setup

1. Rename `config.toml.example` to `config.toml`.

2. You will need to create a API token in your account page first.
Then in the `config.toml` fill out the `apitoken` sections with your API token `name` and the key in the `psk` section.


## CLI syntax

**Note:**
* `site-details` has no sub commands and will just return JSON data for your site details.
* CLi syntax may change, use `--help` to check.




### Main options:
```
CLI tool to provide access to Amber Energy's customer REST API

Usage: amber-client --config-file <FILE> <COMMAND>

Commands:
  site-details
  price         Price window to query for data.(current, next, previous)
  usage         Date range to query history data for. (Using: yyyy-mm-dd format)
  renewables    Price window to query for data.(current, next, previous)
  help          Print this message or the help of the given subcommand(s)
```

### (price) Price data:
```
Usage: amber-client --config-file <FILE> price <COMMAND>

Commands:
  current   Current interval data
  previous  Previous interval data
  next      Forecast interval data
  help      Print this message or the help of the given subcommand(s)
```

### (usage) Historical data:
```
Usage: amber-client usage date-range <START_DATE> <END_DATE> [FILENAME_TO_EXPORT_TO]

Arguments:
  <START_DATE>             Start date to query from
  <END_DATE>               End date of query from
  [FILENAME_TO_EXPORT_TO]  [Optional] Path to save/export data in CSV format
```

**NOTE** 
The argument `FILENAME_TO_EXPORT_TO` is optional and will cause the tool to save data to disk for the selected date range.
If you do not specify the `FILENAME_TO_EXPORT_TO` argument, data will sent to your console/stdout.

Example:
```
$ amber-client -c config.toml usage date-range 2023-12-20 2023-12-21 /tmp/file-name.cvs
Writing to file: /tmp/file-name.cvs
Writing dataset headers to file...
Startng to write records to file...
Finished writing records to file
```

### (renewables) Renewables percentage in your state's grid:
```
Usage: amber-client --config-file <FILE> renewables <COMMAND>

Commands:
  current   Current interval data
  previous  Previous interval data
  next      Forecast interval data
  help      Print this message or the help of the given subcommand(s)
```

### Example output from the `prices` command:
```
[
  {
    "type": "CurrentInterval",
    "date": "2023-12-25T00:00:00.000Z",
    "duration": 30,
    "startTime": "2023-12-24T22:30:01.000Z",
    "endTime": "2023-12-24T23:00:00.000Z",
    "nemTime": "2023-12-24T23:00:00.000Z",
    "perKwh": 5.91618,
    "renewables": 73.719,
    "spotPerKwh": -4.60785,
    "channelType": "general",
    "spikeStatus": "none",
    "tariffInformation": {
      "period": "offPeak"
    },
    "descriptor": "extremelyLow",
    "estimate": true
  }
]
```


## What works now?

**Note:** all queries are fixed to the 30min resolution, as this is all Amber supports for now.

As of version 0.2.0, all of Ambers API end points have been covered and are supported.

* Getting Site details.
* Getting current usage. (current, next and previous 30min windows)
* Getting price forecasts. (current, next and previous 30min windows)
* Getting historical usage data for a given date range.
* Exporting historical data to file as a CSV file.
* Getting the percentage of renewables in the grid for your state.

## What is missing or not working?

* Test cases are a work in progress, not a source of truth(Yet!).

## What future features are planned?

* Daemon mode to emit price or usage data on a regular interval.
* Other output formats aside from JSON.
* Sending price alerts to local devices.
* Working with Home Assistant.
