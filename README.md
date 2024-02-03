# rcal

A calendar that runs on the command line.

![rcal01](rcal_01.png)

The order of month and year is arbitrary.
```
rcal 3 2024
rcal 2024 3
rcal 3
```


## HELP

```
> rcal --help
rcal 0.1.0

USAGE:
    rcal.exe [OPTIONS] [ARGS]

ARGS:
    <MONTH>
            Month number : 1-12

    <YEAR>
            Year : 1900-

OPTIONS:
    -c, --column <MONTH_COLUMN>
            The number of calendar columns.

            [default: 3]

    -h, --help
            Print help information

    -n, --num <MONTH_NUM>
            Number of months to display.

            [default: 3]

    -V, --version
            Print version information

    -z, --nocolor
            No colorize.
```
