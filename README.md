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
Usage: amber-client --config-file <FILE> <COMMAND>

Commands:
  site-details   
  current-price  Price window to query for data.(current, next, previous)
  usage          Date range to query history data for. (Using: yyyy-mm-dd format)
  help           Print this message or the help of the given subcommand(s)
```

### (current-price) Price window options:
```
Usage: amber-client --config-file <FILE> current-price <COMMAND>

Commands:
  current   Current interval pricing estimate
  previous  Actual interval pricing
  next      Forecast interval pricing
  help      Print this message or the help of the given subcommand(s)
```

### (usage) Historical data:
```
Usage: amber-client usage date-range <START_DATE> <END_DATE>

Arguments:
  <START_DATE>  Start date to query from
  <END_DATE>    End date of query from
```

### Example output from the `current-prices` command:
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

* Getting Site details
* Getting current usage
* Getting price forecasts 
* Getting historical usage data for a given date range

## What is missing or not working?

* Getting data on the current % of renewables in the grid.
* Test cases are a work in progress, not a source of truth(Yet!).

## What future features are planned?

* Save historical prices to a CSV file.
* Daemon mode to emit price or usage data on a regular interval.
* Other output formats aside from JSON.
* Sending price alerts to local devices.
* Working with Home Assistant.
