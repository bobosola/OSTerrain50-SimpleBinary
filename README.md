# OS Terrain 50 Simple Binary Data

This repo contains Rust code to build a command-line application which produces a simple efficient binary elevation data file from the freely-available ASCII [Ordnance Survey OS Terrain 50](https://www.ordnancesurvey.co.uk/business-government/products/terrain-50) data set. This allows for very fast reading of elevation data for all of Great Britain.

Why Rust? Because it produces small, cross-platform, very high performance native executables with the minimum of fuss.

The repo also contains Rust demo code to read the binary elevations file.

A PHP class to read the binary elevations file is also available from the repo [OSTerrain50-PHP](https://github.com/bobosola/OSTerrain50-PHP). Both the Rust and PHP file-reading code are heavily commented to assist translations to other languages. 

## Compiled executables

Compiled ready-to-run executables are available for download for Windows, Mac & Ubuntu in the ``binaries`` directory. The Mac version is a Universal Binary for both Intel and Apple silicon Macs.

## What problem does this solve?

The OS Terrain 50 data is already available in a number of formats. However most are complex binary formats for consumption by specialized GIS software. The only non-binary format is the ASCII Grid format which contains thousands of small files which require around 650MB of storage space. Each data file holds 40,000 elevation values of unequal length separated by spaces. So in order to obtain an elevation, the appropriate file would have to be identified, read in full as a string, and split on spaces in order to try to find individual data values. Elevation profiles across even moderate distances would thus require the identifying and reading of many data files in succession.

So this application offers a simpler alternative: it creates a single space-efficient 229MB binary data file containing all the OS elevation data packed in the same geographical order as the OS grids, from the SW corner of England to the Shetland Islands (off the NE corner of Scotland). This can be used by any language which is capable of a binary file read and file pointer jumps to the desired elevation values. It can be zipped down to around 128MB for easier transport.

On a 2020 Mac mini M1, the demo Rust code can retrieve 18,485 elevations at 50m intervals along the length of Great Britain between Niton Down (Isle of Wight) and Dùnan Mòr (Cape Wrath, Scotland) in just under 30ms - much faster than trying to parse several hundred ASCII files.

## Building the application

1) [Install Rust](https://www.rust-lang.org/learn/get-started)
2) ``git clone https://github.com/bobosola/OSTerrain50-SimpleBinary`` or download and unzip this repository to a directory called ``OSTerrain50-SimpleBinary``
3) ``cd OSTerrain50-SimpleBinary``
4) ``cargo build --release``
5) The compiled executable will be in ``OSTerrain50-SimpleBinary/target/release``

## Usage

1\) ``./{application} {path to OS zip file}`` unzips an [OS Terrain 50 ASCII Grid zip file](https://osdatahub.os.uk/downloads/open/Terrain50) then creates the binary data file ``OSTerrain50.bin`` from the unzipped data directory.

2 \)``./{application} {path to data directory}`` creates the binary data file ``OSTerrain50.bin`` from an existing (fully-unzipped) OS Terrain 50 data directory. This is just a convenience option if the data has already been unzipped.

## Description of the binary data file format

This section is for developers who may wish to read the binary data for their own purposes. It assumes some familiarity with the Ordnance Survey National Grid. A diagram of [the OS grid](https://en.wikipedia.org/wiki/Ordnance_Survey_National_Grid) is recommended to be viewed alongside this page.

The repo also includes an [010 Editor](https://www.sweetscape.com/010editor/) template to help navigate the ouput file for data checking and debugging during development. More information is in ``010template/README.md``. This is not a requirement however.

The binary data file consists of:

* a file signature
* a header section
* a data section

Addresses and elevation values are stored in little-endian byte order.

The elevation data is in metres. It is supplied in the OS ASCII data files either as a whole number or as a decimal value to one decimal place up to a maximum value of 1345m at [Ben Nevis](https://getoutside.ordnancesurvey.co.uk/local/ben-nevis-highland). Coastal waterline values vary (see the [OS User Guide](https://www.ordnancesurvey.co.uk/documents/product-support/user-guide/os-terrain-50-user-guide.pdf) for more information on this) so small negative coastline values of e.g. -1.5m may be found.

So for storage efficiency, all OS elevation values have been multiplied by 10 to allow for storage as 16 bit integers rather than floats. This approach requires half the storage space of floats while maintaining full data accuracy. All retrieved raw values must therefore be divided by 10 in your code before use. 16 bit half floats were also considered for storage purposes, but were rejected because they [run out of decimal place accuracy](https://en.wikipedia.org/wiki/Half-precision_floating-point_format#Precision_limitations_on_decimal_values_in_[1,_2048]) for this use case and are also not supported in many languages.

### File sig

The 11 byte sig is the characters ``OSTerrain50``. This is for simple confirmation of the correct file type in code or when using a hex editor.

### Header section

The header section contains 91 contiguous blocks of 402 bytes, each one representing the full set of 100km² grids from SV to JM, going west to east and south to north as per the OS grid pattern. Each block contains:

* 2 bytes for a pair of grid identifier characters (e.g. 'SV', 'SW' etc.)
* 100 four-byte data address placeholders

The inclusion of the grid identifier is primarily for use with the included 010 Editor template to help navigate the ouput file while debugging.      

The 100 address placeholders within a header grid block are ordered W to E and S to N. They may contain anything from 0 to 100 data addresses depending on how many 10km² OS data files are available for that grid. If no data file exists (i.e. it's a 100% sea area), then the data address is left blank. Each populated address is stored as a 32 bit unsigned int and points to a data block containing the elevation values for that particular 10km² area.

### Data section

The data section comprises contiguous data blocks, each representing an imported OS data file representing a 10km² area with elevations every 50m. Thus each block contains 200 rows by 200 columns of elevation data. Each elevation data value within a data block is stored from W to E and S to N and is stored as a 16 bit signed integer. 

### File layout

The following is an attempt to visually demonstrate the file layout. Pipe symbols (which do not exist in the file) have been added for clarity. Both the addresses and the address blanks are 4 bytes long. 

Note that grid SV has only four data files: SV80, SV81, SV90, SV91. When stored W to E and S to N their data addresses are stored as shown below, which match their layout in the OS grid.

```
OSTerrain50|SV|    |    |    |    |    |    |    |    |addr80|addr90|↵
    |    |    |    |    |    |    |    |addr81|addr91|↵
... 80 more 4 byte blanks ...|↵        <- end of SV header block
SW|... 100 4-byte blocks ...|↵         <- SW header block
... 88 more 402-byte blocks ...↵       <- SX to JL header blocks
JM|... 100 4-byte blocks ...|↵         <- end of JM header, start of data section
120|127|131|130|... 39,996 more ...|↵  <- SV80 elevations (not real values)
... 40,000 elevations ...|↵            <- SV90 elevations
... 40,000 elevations ...|↵            <- SV81 elevations
... 40,000 elevations ...|↵            <- SV91 elevations
... 40,000 elevations ...|↵            <- 1st populated SW elevation block
... continue until EOF
```

The 40,000 elevation values in a data block are stored from the south west corner and run W to E for 200 values and then S to N to represent 200 rows of 200 values stored linearly.

NB: the demo supplied elevation values above represent 12m, 12.7m, 13.1m, and 13m.

## Reading the data

Conceptually, an elevation is retrieved thus:

* calculate the applicable 100km² header grid section (SV etc.)
* calculate which of the 100 10km² address placeholders in the grid section holds the data address
* jump to the 10km² data address placeholder and read the address as an unsigned 32 bit integer
* if there is no data address, then it's a 100% sea area so return 0
* if there is a data address, calculate the required offset in the data block for the exact location
* add the offset to the data address and jump to that location
* read the elevation data as a signed 16 bit integer then divide by 10

The function ``read_elevations()`` in ``tests\common\mod.rs`` has example Rust code of how to make the various calculations. The PHP repo [OSTerrain50-PHP](https://github.com/bobosola/OSTerrain50-PHP) also contains PHP code of how to make the calculations.
