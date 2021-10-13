# 010 Editor template

Neither the [010 Editor](https://www.sweetscape.com/010editor/) nor the template supplied here are required, but they do enable much easier binary file navigation, jumping to addresses, and automatic translation of binary data for viewing and debugging.

First ensure that ``Template Results`` is ticked in the 010 Editor ``View`` menu.

Open the OS binary data file in the 010 Editor, then in the ``Templates`` menu:
* select ``Open Template`` and select template file
* select ``Run Template``

The OS binary data file should now be colour-highlighted. You can right-click anywhere in the main window and select ``Jump To Template Variable`` to read the automatically-translated values in the template results panel.

Where the value is an address, you can right-click the address and select ``GoTo`` to jump to that address.

You are now at the start of a data block. You can right-click the 1st value and select ``Jump To Template Variable`` to read all the data values.