# 010 Editor template

The [010 Editor](https://www.sweetscape.com/010editor/) is a very powerful commercial text/hex editor. It supports the use of custom templates to enable much easier binary file navigation, jumping to addresses, and automatic translation of binary data for viewing and debugging. The 010 editor and the template supplied here are not a requirement for using the code, but they do help enormously if you want to dig around in the data structure of the binary output file.

## Instructions for use

1) Download the template file.
2) Open the OS binary data file in the 010 Editor.
3) Tick  ``Template Results`` in the 010 Editor ``View`` menu to open up the Template Results window. 
4) In the  ``Templates`` menu:
    * select ``Open Template`` and select the downloaded template file.
    * select ``Run Template``.

The OS binary data file should now be colour-highlighted. You can right-click anywhere in the main window and select ``Jump To Template Variable`` to read the automatically-translated values in the template results panel.

Where the value is an address, you can right-click the address and select ``GoTo`` to jump to the address of a data block. You can then right-click a value and select ``Jump To Template Variable`` to read translated data values.