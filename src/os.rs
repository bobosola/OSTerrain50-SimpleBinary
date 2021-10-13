
/*         
    The full data grid has 7 columns x 13 rows = 91 grids which might potentially contain data.
    However only 55 currently have data as shown below:
    
    |--------------|-----|
    |.. .. .. .. HP|.. ..|
    |.. .. .. HT HU|.. ..|
    |.. HW HX HY HZ|.. ..|
    |--------------|-----|            
    |NA NB NC ND   |.. ..|
    |NF NG NH NJ NK|.. ..|   
    |NL NM NN NO   |.. ..|
    |   NR NS NT NU|.. ..|
    |   NW NX NY NZ|OV ..|
    |--------------|-----|
    |.. .. .. SD SE|TA ..|
    |.. .. SH SJ SK|TF TG|
    |.. SM SN SO SP|TL TM|
    |.. SR SS ST SU|TQ TR|
    |SV SW SX SY SZ|TV ..|
    |--------------|-----|       
*/ 

// The full 91 100km² OS grids from W to E and S to N.
pub const GRID_100: [&str; 91] = [
    "SV","SW","SX","SY","SZ","TV","TW",
    "SQ","SR","SS","ST","SU","TQ","TR",
    "SL","SM","SN","SO","SP","TL","TM",
    "SF","SG","SH","SJ","SK","TF","TG",
    "SA","SB","SC","SD","SE","TA","TB",
    "NV","NW","NX","NY","NZ","OV","OW",
    "NQ","NR","NS","NT","NU","OQ","OR",
    "NL","NM","NN","NO","NP","OL","OM",
    "NF","NG","NH","NJ","NK","OF","OG",
    "NA","NB","NC","ND","NE","OA","OB",
    "HV","HW","HX","HY","HZ","JV","JW",
    "HQ","HR","HS","HT","HU","JQ","JR",
    "HL","HM","HN","HO","HP","JL","JM"
];

// Constants specific to OS grids and elevation data

pub const FILE_SIG: &[u8; 11]       = b"OSTerrain50";    // Identifying file signature at start of output file
pub const OUTPUT_FILE_NAME: &str    = "OSTerrain50.bin"; // Name of the binary data output file
pub const FILE_SUFFIX: &str         = ".asc" ;           // OS data file suffix
pub const INNER_DATA_DIR: &str      = "data";            // The single child directory of the top data directory
pub const OS_NEW_LINE: &str         = "\r\n";            // Data row separator in data files
pub const OS_DATA_SEPARATOR: &str   = " ";               // Elevation data separator in data rows
pub const ELEVATIONS_PER_ROW: usize = 200;               // No. of elevation values in each data row (and column)
pub const MAX_NUM_DATA_FILES: i64   = 100;               // Maximum number of data files per 10km² grid
pub const ROWS_IN_10_GRID: i64      = 10;                // No. of files per row (and column) per 10km² grid
pub const GRID_IDENT_LEN: i64       = 2;                 // Length of a grid identifer ("SV" etc.)
pub const ADDRESS_LENGTH: i64       = 4;                 // Length of data addresses stored in the output file