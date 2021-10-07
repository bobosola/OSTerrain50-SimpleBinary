#!/opt/homebrew/bin/bash

# Fully unzips an Ordnance Survey OSTerrain50 ASCII data zip file.
# Amend the bash path and the zip file path accordingly.

cd /Users/bobosola/rust/osterrain50/testdata 
unzip terr50_gagg_gb.zip
for d in terr50_gagg_gb/data/*; do   
	if [ -d "$d" ]; then     
		(
		 cd $d
		 unzip \*.zip
		 # deletes the internal zips after unzipping
		 rm -f *.zip 
		)
    fi; 
done