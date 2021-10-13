mod common;

use std::time::Instant;

/************************************
  Scotland
************************************/

// Binary data file
const DATA_FILE: &str = "/Users/bobosola/rust/osterrain50/testdata/OSTerrain50.bin";

#[test]
fn ronas_hill() {  
    // Shetland
    // https://getoutside.ordnancesurvey.co.uk/local/ronas-hill-shetland-islands
    // 443.3m (+5.5m difference)
    let clean_coords = common::parse_coords(&["430530, 1183500"]);
    if let Ok(coord_list) = common::get_elevations(DATA_FILE, &clean_coords, false) {
        if let Some(elevation) = coord_list[0].elevation {
            assert_eq!(448.8 as f32, elevation);
        }
    }
}

#[test]
fn ben_nevis() {
    // https://getoutside.ordnancesurvey.co.uk/local/ben-nevis-highland
    // 1345m (0 difference)  
    let clean_coords = common::parse_coords(&["NN 1669 7127"]);
    if let Ok(coord_list) = common::get_elevations(DATA_FILE, &clean_coords, false) {
        if let Some(elevation) = coord_list[0].elevation {
            assert_eq!(1345 as f32, elevation);
        }
    }    
}

#[test]
fn cairn_gorm() {
    // https://getoutside.ordnancesurvey.co.uk/local/cairn-gorm-highland
    // 1244m (-1.2m difference)     
    let clean_coords = common::parse_coords(&["300510, 804054"]);
    if let Ok(coord_list) = common::get_elevations(DATA_FILE, &clean_coords, false) {
        if let Some(elevation) = coord_list[0].elevation {
            assert_eq!(1242.8 as f32, elevation);
        }
    }  
}

// /************************************
//   Wales
// ************************************/

#[test]
fn mynydd_bodafon() {
    // Anglesey
    // https://getoutside.ordnancesurvey.co.uk/local/mynydd-bodafon-isle-of-anglesey-sir-ynys-mon-or-yr-arwydd-ll718bg
    // 173.8m (-0.7m difference)
    let clean_coords = common::parse_coords(&["247244, 385418"]);
    if let Ok(coord_list) = common::get_elevations(DATA_FILE, &clean_coords, false) {
        if let Some(elevation) = coord_list[0].elevation {
            assert_eq!(173.1 as f32, elevation);
        }
    }      
}

#[test]
fn mount_snowdon() {
   // https://getoutside.ordnancesurvey.co.uk/local/snowdon-summit-railway-station-gwynedd
   // 1056.8m (-0.1m difference)
    let clean_coords = common::parse_coords(&["SH 6094 5434"]);
    if let Ok(coord_list) = common::get_elevations(DATA_FILE, &clean_coords, false) {
        if let Some(elevation) = coord_list[0].elevation {
            assert_eq!(1056.7 as f32, elevation);
        }
    }   
}

#[test]
fn foel_cwmcerwyn() {
	// South Wales
    // https://getoutside.ordnancesurvey.co.uk/local/foel-cwmcerwyn-pembrokeshire-sir-benfro
    // 532.6m (0 difference)
    let clean_coords = common::parse_coords(&["209395, 231152"]);
    if let Ok(coord_list) = common::get_elevations(DATA_FILE, &clean_coords, false) {
        if let Some(elevation) = coord_list[0].elevation {
            assert_eq!(532.6 as f32, elevation);
        }
    } 
}

// /************************************
//   England
// ************************************/

#[test]
fn beacon_hill() {
	// Norfolk coast
    // https://getoutside.ordnancesurvey.co.uk/local/beacon-hill-kings-lynn-and-west-norfolk
    // 52m (-0.4m difference)
    let clean_coords = common::parse_coords(&["573218, 341864"]);
    if let Ok(coord_list) = common::get_elevations(DATA_FILE, &clean_coords, false) {
        if let Some(elevation) = coord_list[0].elevation {
            assert_eq!(51.6 as f32, elevation);
        }
    } 
}

#[test]
fn parliament_hill() {
	// London
    // https://getoutside.ordnancesurvey.co.uk/local/parliament-hill-camden
    // 101.6m (-0.9m difference)
    let clean_coords = common::parse_coords(&["528054, 186978"]);
    if let Ok(coord_list) = common::get_elevations(DATA_FILE, &clean_coords, false) {
        if let Some(elevation) = coord_list[0].elevation {
            assert_eq!(100.7 as f32, elevation);
        }
    } 
}

#[test]
fn ebrington_hill() {
	// Stratford-on-avon
    // https://getoutside.ordnancesurvey.co.uk/local/ebrington-hill-stratford-on-avon
    // 259.9m (+0.2m difference)
    let clean_coords = common::parse_coords(&["SP 1872 4258"]);
    if let Ok(coord_list) = common::get_elevations(DATA_FILE, &clean_coords, false) {
        if let Some(elevation) = coord_list[0].elevation {
            assert_eq!(260.1 as f32, elevation);
        }
    } 
}

#[test]
fn butser_hill() {
	// Hampshire
    // https://getoutside.ordnancesurvey.co.uk/local/butser-hill-east-hampshire
    // 270.4m (+0.3m difference)
    let clean_coords = common::parse_coords(&["SU 7166 2031"]);
    if let Ok(coord_list) = common::get_elevations(DATA_FILE, &clean_coords, false) {
        if let Some(elevation) = coord_list[0].elevation {
            assert_eq!(270.7 as f32, elevation);
        }
    } 
}

#[test]
fn skiddaw() {
	// Lake District
    // https://getoutside.ordnancesurvey.co.uk/local/skiddaw-allerdale
    // 927.9m (-0.8m difference)
    let clean_coords = common::parse_coords(&["326041, 529086"]);
    if let Ok(coord_list) = common::get_elevations(DATA_FILE, &clean_coords, false) {
        if let Some(elevation) = coord_list[0].elevation {
            assert_eq!(927.1 as f32, elevation);
        }
    } 
}

#[test]
fn lands_end() {
    // https://getoutside.ordnancesurvey.co.uk/local/lands-end-cornwall
    // 37.9m (+3.5m difference)
    let clean_coords = common::parse_coords(&["SW 3423 2523"]);
    if let Ok(coord_list) = common::get_elevations(DATA_FILE, &clean_coords, false) {
        if let Some(elevation) = coord_list[0].elevation {
            assert_eq!(41.4 as f32, elevation);
        }
    } 
}

// /************************************
//   Timing test
// ************************************/

#[test]
fn length_of_gb() {

    // Read all intermediate 50m elevations from
    // Niton Down, Isle of Wight: SZ 494 772 to 
    // Dùnan Mòr, Cape Wrath:     NC 261 740
    // (18,485 in total)

    let start_time = Instant::now();
    let clean_coords = common::parse_coords(&["SZ 494 772", "NC 261 740"]);
    if let Ok(coord_list) = common::get_elevations(DATA_FILE, &clean_coords, true) {
        let num_coords = coord_list.len();
        // run 'cargo test -- --nocapture' to see this line
        println!("Found {} elevations in {:?}.", num_coords, start_time.elapsed());     
        assert_eq!(18485, num_coords);
    } 
}