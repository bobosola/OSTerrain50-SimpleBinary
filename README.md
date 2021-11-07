# OS Terrain 50 Simple Binary Data

This repo contains Rust code to build a command-line application which produces a simple binary elevation data file using data imported from the freely-available ASCII  [Ordnance Survey OS Terrain 50](https://www.ordnancesurvey.co.uk/business-government/products/terrain-50) data set.  This allows for very fast reading of elevation data for all of Great Britain.

Why Rust? Because it produces small, cross-platform, very high performance native executables with the minimum of fuss. Compiled ready-to-run versions of this application for Windows, Mac & Ubuntu are available to [download from the author's site](https://osola/org/uk/osterrain50).

Rust demo code to read the binary elevations file is also available in the repo. On a 2020 Mac mini M1, this retrieved 18,485 elevations at 50m intervals along a line between Niton Down (Isle of Wight) and Dùnan Mòr (Cape Wrath, Scotland) in just under 30ms.

A PHP class to read the binary elevations file is also available from the repo [OSTerrain50-PHP](https://github.com/bobosola/OSTerrain50-PHP). Both the Rust and PHP file reading code are heavily commented to allow for easy translation to other languages. 

## What problem does this solve?

The OS Terrain 50 data is already available in a number of formats. However most are complex binary formats for consumption by specific software. The only non-binary format is the ASCII Grid format which contains thousands of small CSV-like files which require around 650MB of storage space.

This application creates a space-efficient 229MB simple binary data file containing all the OS data for use in any language capable of file pointer jumps and binary reads.

## Building the application

1) [Install Rust](https://www.rust-lang.org/learn/get-started)
2) ``git clone https://github.com/bobosola/OSTerrain50-SimpleBinary`` or download and unzip this repository to a directory called ``OSTerrain50-SimpleBinary``
3) ``cd OSTerrain50-SimpleBinary``
4) ``cargo build --release``
5) The compiled executable will be in ``OSTerrain50-SimpleBinary/target/release``

## Usage

1\) ``./<application>   <path to OS zip file>`` unzips an [OS Terrain 50 ASCII Grid zip file](https://osdatahub.os.uk/downloads/open/Terrain50) then creates the binary data file ``OSTerrain50.bin`` from the unzipped data directory.

2 \)``./<application>  <path to data directory>`` creates the binary data file ``OSTerrain50.bin`` from an existing (fully-unzipped) OS Terrain 50 data directory.

## Description of the binary data file format

This section is for developers who may wish to read the binary data for their own purposes. It assumes some familiarity with the Ordnance Survey National Grid. A diagram of [the OS grid](https://en.wikipedia.org/wiki/Ordnance_Survey_National_Grid) is recommended to be viewed alongside this page.

The repo also includes an [010 Editor](https://www.sweetscape.com/010editor/) template to help navigate the ouput file for data checking and debugging during development. More information is in ``010template/README.md``. This is not a requirement however.

The binary data file consists of:

* a file signature
* a header section
* a data section

Addresses and elevation values are stored in little-endian byte order.

The elevation data is in metres. It is supplied either as a whole number or a decimal value to one decimal place up to a maximum value of 1345m at [Ben Nevis](https://getoutside.ordnancesurvey.co.uk/local/ben-nevis-highland). Coastal waterline values vary (see the [OS User Guide](https://www.ordnancesurvey.co.uk/documents/product-support/user-guide/os-terrain-50-user-guide.pdf) for more information on this) so small negative coastline values of e.g. -1.5m may be found.

All supplied elevation values have been multiplied by 10 to allow for storage as 16 bit integers rather than 32 bit floats. This approach requires half the storage space of floats while maintaining full data accuracy. All retrieved values must therefore be divided by 10 before use. (16 bit half floats were also tested for storage purposes, but they [run out of decimal place accuracy](https://en.wikipedia.org/wiki/Half-precision_floating-point_format#Precision_limitations_on_decimal_values_in_[1,_2048]) beyond elevation values of 64m.)

### File sig

The sig is the characters ``OSTerrain50`` as 11 bytes. This is for simple confirmation of the correct file type.

### Header section

The header section contains 91 contiguous blocks of 402 bytes, each one representing the full set of 100km² grids from SV to JM, going west to east and south to north as per the OS grid pattern. Each block contains:

* 2 bytes for a pair of grid identifier characters ('SV', 'SW' etc.)
* 100 four-byte data address placeholders

The inclusion of the grid identifier is primarily for use with the included 010 Editor template to help navigate the ouput file.       

The 100 address placeholders within a header grid block are ordered W to E and S to N. They may contain anything from 0 to 100 data addresses depending on how many 10km² OS data files are available for that grid. If no data file exists (i.e it's a 100% sea area), then the data address is left blank. Each populated address is stored as a 32 bit unsigned int and points to a data block containg elevation values.

### Data section

The data section comprises contiguous data blocks, each representing an imported OS data file which contains 200 rows by 200 columns of elevation data. Each elevation data value within a data block is stored from W to E and S to N and is stored as a 16 bit signed integer. 

### File layout

The following is an attempt to visually demonstrate the file layout. Pipe symbols (which do not exist in the file) have been added for clarity. Both the addresses and the address blanks are 4 bytes long: 

```
OSTerrain50|SV|    |    |    |    |    |    |    |    |{ addr80 }|{ addr90 }|↵
    |    |    |    |    |    |    |    |{ addr81 }|{ addr91 }|... 80 more blanks ...|↵
SW|... 100 4-byte blocks ...|↵
... 88 more 402-byte blocks ...↵
JM|... 100 4-byte blocks ...|↵         <- end of header, start of data section
120|127|131|130|... 39,996 more ...|↵  <- SV80 elevations (not real values)
... 40,000 elevations ...|↵            <- SV90 elevations
... 40,000 elevations ...|↵            <- SV81 elevations
... 40,000 elevations ...|↵            <- SV91 elevations
... 40,000 elevations ...|↵            <- first SW elevation block
...continue until EOF
```

Note that grid SV has only four data files: SV80, SV81, SV90, SV91. When stored W to E and S to N their data addresses are stored as shown above, which match their layout in the OS grid.

The elevations in a data block start at the south west corner and run W to E and then S to N (after 200 data points), where the demo supplied elevation values above were 12, 12.7, 13.1, and 13.

## Reading the data

Conceptually, an elevation is retrieved thus:

* calculate the applicable header grid section
* calculate which address placeholder in the grid section holds the data address
* jump to the address placeholder and read the data address as a u32
* if there is no data address, then it's a 100% sea area so return 0
* if there is a data address, calculate the required offset in the data block for the exact location
* add the offset to the data address and jump to that location
* read the elevation data as an i16 then multiply by 10

The function ``read_elevations()`` in ``tests\common\mod.rs`` has examples of how to make the various calculations.
