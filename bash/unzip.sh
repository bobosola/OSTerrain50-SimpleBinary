#!/opt/homebrew/bin/bash
cd /Users/bobosola/rust/osterrain50/download
unzip terr50_gagg_gb.zip
for d in terr50_gagg_gb/data/*; do   
	if [ -d "$d" ]; then     
		(
		 cd $d
		 unzip \*.zip
		 #rm -f *.zip  # uncomment to delete the zip after unzipping
		)
    fi; 
done