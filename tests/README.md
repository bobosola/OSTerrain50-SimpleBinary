# Integration tests

These integration tests require the binary data file to have been built. ``const DATA_FILE`` must be adjusted to the binary data file path.

12 location tests retrieve elevations for 12 widely-spaced landmark locations around GB and compare the found elevations to the official OS peak elevation values. The values are usually very close (within 1m difference) and are sometimes identical. However some outliers can vary up to a few metres of difference. OS are [activley working](https://www.ordnancesurvey.co.uk/business-government/tools-support/terrain-50-support) to include known summit heights in the data so these small disrepancies should disappear over time.

The ``length_of_gb`` test retrieves all 18,485 coordinates with elevations for a straight line between Niton Down (Isle of Wight) to Dùnan Mòr (Cape Wrath, Scotland) at 50m intervals.