# Integration tests

These integration tests require the binary data file to have been built. ``const DATA_FILE`` must have the value of the binary data file path.

12 identical tests retrieve elevations for 12 random landmark locations around GB and compare the found elevation to the official OS local values. The values are usually very close (within 1m difference) if not identical. However some outliers can vary up to a few metres difference.

The ``length_of_gb`` test retrieves all 18,485 coordinates with elevations for a straight line between Niton Down (Isle of Wight) to Dùnan Mòr (Cape Wrath, Scotland) at 50m intervals.