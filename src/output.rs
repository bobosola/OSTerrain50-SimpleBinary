use std::{error::Error, path};
use std::fs::File;
use std::collections::HashMap;

// All code related to writing the OS binary file

const FILE_SIG: &[u8; 11]        = b"OSTerrain50";  // 11 byte file sig at start of file
const DATA_SUFFIX: &str          = ".asc";          // OS data file suffix
const DATA_LINES: i32            = 200;             // Number of data lines in a 10km² data .ASC file
const DATA_VALS_PER_LINE: i32    = 200;             // Number of data values in each data line        	
const ROWS_COLS_IN_500_GRID: i32 = 5;               // Number of 100km² rows and columns in 500km² grid
const GRIDS_100K_COUNT: i32      = 91;              // Number of 100km² grids to cover UK as a 7 x 13 rectangle

// See http://en.wikipedia.org/wiki/Ordnance_Survey_National_Grid
// for explanation of OS grid structure.
// The diagrams on the above page will visually explain the arrays below:

// All the 500km² grids covering the OS data area going from W to E and S to N
const GRID_500: [[u8; 2]; 3] = [
    [b'S', b'T'], 
    [b'N', b'O'], 
    [b'H', b'J']
];

// All the 100km² grid within a 500km² grid going W to E and S to N (note 'I' is not used).
// Note that 6 x 25 = 150 grids, but only 91 contain data. Empty grids will be ignored.
const GRID_100: [u8; 25] = [
    b'V', b'W', b'X', b'Y', b'Z',
    b'Q', b'R', b'S', b'T', b'U', 
    b'L', b'M', b'N', b'O', b'P',
    b'F', b'G', b'H', b'J', b'K',
    b'A', b'B', b'C', b'D', b'E'
];

pub fn build_output_file(data_dir: &path::Path) -> Result<path::PathBuf, Box<dyn Error>> {

    // Vector to hold the full 7 x 13 100km² grid names from 'SV' 
    // in the south west corner up to 'JM' in the north east corner
    let full_grid: Vec<&str> = Vec::new();

    // Hash map to hold the addresses of the elevation data for each 10km² grid
    let mut offsets: HashMap<[u8; 2], u32> = HashMap::new();

   let first = GRID_100[1];
   let second = GRID_500[0][0];
   let grid_name = [first, second];

    Ok(data_dir.to_path_buf())
}

// fn grid_names_match(grid1: [u8; 2], grid2: [u8; 2]) -> bool {

//     for i in 0..2 {
//         if grid1[i] != grid2[i] {
//             return false;
//         }
//     }
//     true
// }

// fn run() -> Result<path::PathBuf, Box<dyn Error>>  {
//     let file_path = "";
//     let file = File::open(file_path)?;
//     let mut rdr = csv::ReaderBuilder::new()
//         .has_headers(false)
//         .delimiter(b' ')
//         .double_quote(false)
//         .flexible(false)
//         .from_reader(file);

//     for result in rdr.records().skip(5) {
//         let record = result?;
//         println!("{:?}", record);
//     }
//     Ok(())
// }
