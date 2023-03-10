# Integration tests

The integration tests require the binary data file to have been built.

**NB:** The path for ``const DATA_FILE`` must be changed to your binary data file path.

The location tests retrieve elevations for various widely-spaced landmark locations around GB and compare the found elevations to the elevation values published on the [OS Get Outside](https://getoutside.ordnancesurvey.co.uk) website. The values are usually within 1m or so. However some outliers can vary up to a few metres of difference.

Note that the OS data does change by small amounts as updated versions of the data set are published.

The ``gb_infills`` test retrieves all 18,485 coordinates with elevations at 50m intervals for a straight line up the length of Great Britain from Niton Down (Isle of Wight) to Dùnan Mòr (Cape Wrath, Scotland).