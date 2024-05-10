use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BOP {
    pub entries: Vec<Entry>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Entry {
    pub track: String,
    #[serde(rename = "carModel")]
    pub car_model: u32,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ballastKg")]
    pub ballast_kg: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrictor: Option<i32>,
}

pub const TRACKS: [&str; 25] = [
    "Barcelona",
    "brands_hatch",
    "cota",
    "donington",
    "Hungaroring",
    "Imola",
    "indianapolis",
    "Kyalami",
    "Laguna_Seca",
    "misano",
    "monza",
    "mount_panorama",
    "nurburgring",
    "nurburgring_24h",
    "oulton_park",
    "Paul_Ricard",
    "red_bull_ring",
    "Silverstone",
    "snetterton",
    "Spa",
    "Suzuka",
    "Valencia",
    "watkins_glen",
    "Zandvoort",
    "Zolder",
];

// This list is not quite alphabetically sorted, instead sorted in a way so if someone searches for the short name they get the new car
pub const CARS: [(u32, &str); 54] = [
    (50, "Alpine A110 GT4"),
    (20, "Aston Martin AMR V8 Vantage GT3"),
    (12, "Aston Martin AMR V12 Vantage GT3"),
    (51, "Aston Martin AMR Vantage GT4"),
    (31, "Audi R8 LMS GT3 Evo II 2"),
    (19, "Audi R8 LMS GT3 Evo 2019"),
    (3, "Audi R8 LMS GT3 2015"),
    (52, "Audi R8 LMS GT4"),
    (80, "Audi R8 LMS GT2"),
    (8, "Bentley Continental GT3 2018"),
    (11, "Bentley Continental GT3 2015"),
    (30, "BMW M4 GT3"),
    (53, "BMW M4 GT4"),
    (7, "BMW M6 GT3"),
    (27, "BMW M2 CS TCX"),
    (55, "Chevrolet Camaro GT4"),
    (32, "Ferrari 296 GT3"),
    (24, "Ferrari 488 GT3 Evo 2020"),
    (2, "Ferrari 488 GT3 2018"),
    (26,"Ferrari 488 Challenge Evo GTC"),
	(36, "Ford Mustang GT3"),
    (56, "Ginetta G55 GT4"),
    (21, "Honda NSX GT3 Evo 2019"),
    (17, "Honda NSX GT3 2017"),
    (57, "KTM Xbow GT4"),
    (82, "KTM Xbow GT2"),
    (14, "Jaguar G3 GT3"),
    (33, "Lamborghini Huaracan GT3 Evo II 2"),
    (16, "Lamborghini Huaracan GT3 Evo 2019"),
    (4, "Lamborghini Huaracan GT3 2015"),
    (29, "Lamborghini Huaracan Super Trofeo ST Evo GTC"),
    (18, "Lamborghini Huaracan Super Trofeo ST GTC"),
    (13, "Lamborghini Gallardo Rex GT3"),
    (15, "Lexus Rc-F GT3"),
    (58, "Maserati MC GT4"),
    (83, "Maserati GT2"),
    (35, "McLaren 720S GT3 Evo"),
    (22, "McLaren 720S GT3 Special 2019"),
    (5, "McLaren 650S GT3"),
    (59, "McLaren 570S GT4"),
    (25, "Mercedes AMG GT3 Evo 2020"),
    (1, "Mercedes AMG GT3 2015"),
    (60, "Mercedes AMG GT4"),
    (84, "Mercedes AMG GT2"),
    (6, "Nissan GT-R GT3 2018"),
    (10, "Nissan GT-R GT3 2015"),
    (34, "Porsche 992 GT3R"),
    (23, "Porsche 991-II GT3R"),
    (0, "Porsche 991 GT3R"),
    (61, "Porsche 718 Cayman GT4"),
    (85, "Porsche 991-II GT2 RS CS Evo"),
    (86, "Porsche 935 GT2"),
    (28, "Porsche 992 GT3Cup GTC"),
    (9, "Porsche 991.2 GT3Cup GTC")
];
