mod common;

/************************************
   Test elevations for various 
   well-known peaks around the UK
************************************/

// Set the path to your binary data file
const DATA_FILE: &str = "/Users/bobosola/Downloads/OSTerrain50.bin";

// Set the minimum acceptable difference (in metres) between the found data from the
// binary data file and the externally published OS elevation data for a given location.
// NB: published data on external sites may differ slightly from the binary data file
// elevation values, so we need to make a small allowance for any differences
const MIN_ELEV_DIFF: f32 = 1.5;

/************************************
  Scotland
************************************/

#[test]
fn ben_nevis() {
    // https://getoutside.ordnancesurvey.co.uk/local/ben-nevis-highland
    let published_elev = 1345.0;
    let coords = "NN 1669 7127";
    let diff = common::get_elev_diff(published_elev, coords, DATA_FILE);
    assert!(diff <= MIN_ELEV_DIFF, "Elevation difference is {}m", format!("{0:.1}", diff));
}

#[test]
fn cairn_gorm() { 
    // https://getoutside.ordnancesurvey.co.uk/local/cairn-gorm-highland
    let published_elev = 1244.0;
    let coords = "300510, 804054";
    let diff = common::get_elev_diff(published_elev, coords, DATA_FILE);
    assert!(diff <= MIN_ELEV_DIFF, "Elevation difference is {}m", format!("{0:.1}", diff));
}

/************************************
   Wales
************************************/

#[test]
fn mynydd_bodafon() {
    // Anglesey
    // https://getoutside.ordnancesurvey.co.uk/local/mynydd-bodafon-isle-of-anglesey-sir-ynys-mon-or-yr-arwydd-ll718bg
    let published_elev = 173.8;
    let coords = "247244, 385418";
    let diff = common::get_elev_diff(published_elev, coords, DATA_FILE);
    assert!(diff <= MIN_ELEV_DIFF, "Elevation difference is {}m", format!("{0:.1}", diff));
}

#[test]
fn mount_snowdon() {
    // Snowdonia
    // https://getoutside.ordnancesurvey.co.uk/local/snowdon-summit-railway-station-gwynedd
    let published_elev = 1056.8;
    let coords = "SH 6094 5434";
    let diff = common::get_elev_diff(published_elev, coords, DATA_FILE);
    assert!(diff <= MIN_ELEV_DIFF, "Elevation difference is {}m", format!("{0:.1}", diff));
}

#[test]
fn foel_cwmcerwyn() {
    // South Wales
    // https://getoutside.ordnancesurvey.co.uk/local/foel-cwmcerwyn-pembrokeshire-sir-benfro
    let published_elev = 532.6;
    let coords = "209395, 231152";
    let diff = common::get_elev_diff(published_elev, coords, DATA_FILE);
    assert!(diff <= MIN_ELEV_DIFF, "Elevation difference is {}m", format!("{0:.1}", diff));
}

/************************************
  England
************************************/

#[test]
fn beacon_hill() {
    // Norfolk coast
    // https://getoutside.ordnancesurvey.co.uk/local/beacon-hill-kings-lynn-and-west-norfolk
    let published_elev = 52.0;
    let coords = "573218, 341864";
    let diff = common::get_elev_diff(published_elev, coords, DATA_FILE);
    assert!(diff <= MIN_ELEV_DIFF, "Elevation difference is {}m", format!("{0:.1}", diff));
}

#[test]
fn parliament_hill() {
    // London
    // https://getoutside.ordnancesurvey.co.uk/local/parliament-hill-camden
    let published_elev = 101.6;
    let coords = "528054, 186978";
    let diff = common::get_elev_diff(published_elev, coords, DATA_FILE);
    assert!(diff <= MIN_ELEV_DIFF, "Elevation difference is {}m", format!("{0:.1}", diff));
}

#[test]
fn ebrington_hill() {
    // Stratford-on-avon
    // https://getoutside.ordnancesurvey.co.uk/local/ebrington-hill-stratford-on-avon
    let published_elev = 259.9;
    let coords = "SP 1872 4258";
    let diff = common::get_elev_diff(published_elev, coords, DATA_FILE);
    assert!(diff <= MIN_ELEV_DIFF, "Elevation difference is {}m", format!("{0:.1}", diff));
}

#[test]
fn butser_hill() {
    // Hampshire
    // https://getoutside.ordnancesurvey.co.uk/local/butser-hill-east-hampshire
    let published_elev = 270.4;
    let coords = "SU 7166 2031";
    let diff = common::get_elev_diff(published_elev, coords, DATA_FILE);
    assert!(diff <= MIN_ELEV_DIFF, "Elevation difference is {}m", format!("{0:.1}", diff));
}

#[test]
fn skiddaw() {
    // Lake District
    // https://getoutside.ordnancesurvey.co.uk/local/skiddaw-allerdale
    let published_elev = 927.9;
    let coords = "326041, 529086";
    let diff = common::get_elev_diff(published_elev, coords, DATA_FILE);
    assert!(diff <= MIN_ELEV_DIFF, "Elevation difference is {}m", format!("{0:.1}", diff));
}

#[test]
fn black_rock() {
    // Cornwall
    // https://getoutside.ordnancesurvey.co.uk/local/black-rock-cornwall-tr114wz
    let published_elev = -1.6;
    let coords = "183381, 31669";
    let diff = common::get_elev_diff(published_elev, coords, DATA_FILE);
    assert!(diff <= MIN_ELEV_DIFF, "Elevation difference is {}m", format!("{0:.1}", diff));
}

/************************************
   Test infill elevations generated
   for the entire length of the UK
************************************/

#[test]
fn gb_infills() {

    // Read all intermediate 50m elevations starting at Niton Down, Isle of Wight: SZ 494 772
    // finishing at Dùnan Mòr, Cape Wrath: NC 261 740. Should produce 18,485 elevation values.

    let start_and_finish_coords = common::parse_coords(&["SZ 494 772", "NC 261 740"]);
    let mut num_elevations_found: usize = 0;
    if let Ok(elevations_found) = common::read_elevations(DATA_FILE, &start_and_finish_coords, true) {
        num_elevations_found = elevations_found.len();
    }
    assert_eq!(18_485, num_elevations_found,  "Elevations found: {}", num_elevations_found);
}
