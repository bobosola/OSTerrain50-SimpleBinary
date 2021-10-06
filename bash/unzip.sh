#!/opt/homebrew/bin/bash

# Recursively unzips an Ordnance Survey OSTerrain50 ASCII data zip file.
# NB: Amend the bash path above for your environment.

# Amend path to the zip file accordingly
cd /Users/bobosola/rust/osterrain50/download 
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