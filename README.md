<h1>OS Terrain 50 Simple Binary Data</h1>

This repo contains Rust code to build a command-line application which produces a simple binary elevation data file using data imported from the freely-available  [Ordnance Survey OS Terrain 50](https://www.ordnancesurvey.co.uk/business-government/products/terrain-50) data set.  This allows for easy (and very fast) reading of elevation data for Great Britain.

Why Rust? Because it produces small, cross-platform, very high performance, stand-alone executables with the minimum of fuss. Compiled versions of this application (for Windows, Mac & Ubuntu) are available to [download from the author's site](TODO).

<h2>What problem does this solve?</h2>

The OS Terrain 50 data is already available in a number of formats. Most are complex binary formats for consumption by specific software. The only non-binary format is the ASCII Grid format which contains thousands of CSV-like files. These take up around 650MB of storage space and are expensive to parse in real time.

This application will:

* (optionally) unzip the [OS zipped ASCII Grid data zip file](https://osdatahub.os.uk/downloads/open/Terrain50) along with its thousands of child zip files
* create a space-efficient 229MB simple binary data file using the unzipped OS data

<h2>Usage</h2>

1) ``./app_name  <path to OS zip file>`` unzips an OS Terrain 50 data zip file then creates the binary data file ``OSTerrain50.bin`` from the unzipped data directory.

2) ``./app_name  <path to data directory>`` creates the binary data file ``OSTerrain50.bin`` from an existing OS Terrain 50 data directory.

<h2>Reading the binary data file</h2>

PHP demo code to consume the data is available from the repo [OSTerrain50-PHP](). Why PHP? Because it can run anywhere and is easily readable by anyone familiar with C-ish syntax. The code is heavily commented to allow for translation to any language which supports binary file reads with file pointers.

<h2>Description of the binary file format</h2>

This section is aimed at developers who may wish to read the binary data for their own purposes. It assumes some familiarity with the Ordnance Survey National Grid. Wikipedia has some [diagrams of the OS grid](https://en.wikipedia.org/wiki/Ordnance_Survey_National_Grid) which are recommended viewing help to clarify the following explanations.

The repo includes a [010 Editor](https://www.sweetscape.com/010editor/) template to help navigate the ouput file for data checking and debugging during development. This enables easy jumping to addresses and automatically translates values. It is highly recommended for viewing and understanding binary files.

The binary data file consists of:

* a file signature
* a header section 
* a data section

Addresses and elevation values are stored in little-endian byte order.

The file sig is``OSTerrain50`` as 11 bytes. This is for simple confirmation of the correct file type.

The header follows the sig. It contains 91 contiguous blocks of 402 bytes, each one representing the full set of 100km² grids from SV to JM, going west to east and south to north as per the OS grid pattern. Each block contains:

* a two-byte 100km² grid identifier (SV etc.)
* 100 four-byte data address placeholders

The inclusion of the grid identifier is primarily for use with the included 010 Editor template to help navigate the ouput file.       

The 100 address placeholders within a header grid block are ordered W to E and S to N. They may contain anything from 0 to 100 data addresses depending on how many 10km² OS data files are available for that grid. If no data file exists then the data address is left blank. Each populated address points to a data block containg elevation values.

<b>Example:</b> the header looks like this (with pipe symbols added for visual clarity which do not exist in the binary). Both the addresses and the blanks are 4 bytes long. Grid SV has only four data files: SV80, SV81, SV90, SV91. Their data addresses are stored as shown below:

```
OSTerrain50|SV| | | | | | | | |<addr80>|<addr90>| | | | | | | | |<addr81>|<addr91>|... <80 more blanks> ...|
SW|... < 100 four-byte blocks> ...|
...
JL|... < 100 four-byte blocks> ...|
JM|... < 100 four-byte blocks> ...|
```
The data section follows the header section. It comprises contiguous data blocks each representing an imported OS data file. Each elevation data point within a data block is stored from E to W and S to N. Each block contains 40,000 elevations representing the data imported from one ASCI data file, which contain 200 rows by 200 columns of elevation data.

The elevation data as supplied by OS is either a whole number or a decimal value to one decimal place. So all stored values have been multiplied by 10 to allow for storage as space-saving 16 bit integers rather than 32 bit floats. 16 bit half floats were considered for storing exact values but were rejected on the grounds of lack of native support in many popular languages (necessitating writing complex bit-shifting unpacking functions). Division by 10 in comparison is very simple and is universally supported.

<b>Example:</b> the following represents a data block starting at the south west corner going east (then south to north after 200 data points), where the actual supplied elevation values were 12, 12.5, 13.1, and 13. Hence all retrieved values must be divided by 10 before use.

```
|120|125|131|130|... <39,996 more> ...|
```