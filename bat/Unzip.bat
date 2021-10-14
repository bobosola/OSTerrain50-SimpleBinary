REM - Batch file for unzipping OS Terrain 50 data zip file
REM - Requires 7-Zip from http://www.7-zip.org/

set Unzip=C:\Program Files\7-zip\7z.exe
set OSData=C:\temp\OS\terr50_gagg_gb.zip
set Destination=C:\temp\OS\unzipped

"%Unzip%" x "%OSData%" -o"%Destination%"
cd "%Destination%"
FOR /D /r %%F in ("*") DO (
pushd %CD%
cd %%F
    FOR %%X in (*.zip) DO (
        "%Unzip%" x %%X
    )
popd
)
del /S /Q *.zip