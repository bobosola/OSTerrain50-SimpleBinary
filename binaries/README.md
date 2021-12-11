# Precompiled Binaries

## Requirements
Download the **ASCII Grid & GML (Grid)** OS Terrain 50 data file from
https://osdatahub.os.uk/downloads/open/Terrain50. This is currently
named as ``terr50_gagg_gb.zip``.

## What this does
This executable will unzip the OS data file plus all its child zips and create the binary data file ``osterrain50.bin`` from the unzipped data.

The downloaded zip file and unzipped OS Terrain 50 data can then 
be deleted.

#### Windows

Unzip the .exe file then run in PowerShell as:
``.\osterrain50.exe path\to\terr50_gagg_gb.zip``

#### Mac

Drag the executable **osterrain50** from the dmg and run in a terminal as:
``./osterrain50 path/to/terr50_gagg_gb.zip``

#### Ubuntu

Extract the executable **osterrain50**  and run in a terminal as:
``./osterrain50 path/to/terr50_gagg_gb.zip``

## Optional argument usage

If you already have a fully unzipped OS Terrain 50 data set you wish to use for some reason, then running as 
``./osterrain50 path/to/fully-unzipped-OS-Terrain-50-data-directory``
will also produces the binary data file.