use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BOP {
    pub entries: Vec<Entry>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Entry {
    pub track: String,
    #[serde(rename = "carModel")]
    pub car_model: u32,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ballastKg")]
    pub ballast_kg: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrictor: Option<i32>,
}

pub const TRACKS: [&str; 23] = [
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
    "oulton_park",
    "Paul_Ricard",
    "Silverstone",
    "snetterton",
    "Spa",
    "Suzuka",
    "Valencia",
    "watkins_glen",
    "Zandvoort",
    "Zolder",
];

pub const CARS: [(u32, &str); 47] = [
    (50, "Alpine A110 GT4"),
    (51, "AMR Vantage GT4"),
    (20, "AMR V8 Vantage GT3"),
    (12, "AMR V12 Vantage GT3"),
    (3, "Audi R8 LMS GT3 '15"),
    (19, "Audi R8 LMS GT3 Evo '19"),
    (31, "Audi R8 LMS GT3 Evo II"),
    (52, "Audi R8 GT4"),
    (11, "Bentley Continental GT3 '15"),
    (8, "Bentley Continental GT3 '18"),
    (27, "BMW M2 CS TCX"),
    (30, "BMW M4 GT3"),
    (53, "BMW M4 GT4"),
    (7, "BMW M6 GT3"),
    (55, "Chevrolet Camaro GT4"),
    (32, "Ferrari 296 GT3"),
    (26,"Ferrari Challenge Evo GTC"),
    (2, "Ferrari 488 GT3"),
    (24, "Ferrari 488 GT3 Evo"),
    (56, "Ginetta G55 GT4"),
    (17, "Honda NSX GT3"),
    (21, "Honda NSX GT3 Evo '19"),
    (57, "KTM Xbow GT4"),
    (14, "Jaguar G3 GT3"),
    (13, "Lamborghini Gallardo Rex GT3"),
    (4, "Lamborghini Huaracan GT3"),
    (16, "Lamborghini Huaracan GT3 Evo '19"),
    (33, "Lamborghini Huaracan GT3 Evo II"),
    (18, "Lamborghini Huaracan ST GTC"),
    (29, "Lamborghini Huaracan ST Evo GTC"),
    (15, "Lexus Rc-F GT3"),
    (58, "Maserati MC GT4"),
    (59, "McLaren 570S GT4"),
    (5, "McLaren 650S GT3"),
    (22, "McLaren 720S GT3 Special"),
    (35, "McLaren 720S GT3 Evo"),
    (1, "Mercedes AMG GT3"),
    (25, "Mercedes AMG GT3 Evo '20"),
    (60, "Mercedes AMG GT4"),
    (10, "Nissan GT-R GT3 '15"),
    (6, "Nissan GT-R GT3 '18"),
    (61, "porsche_718_cayman_gt4_mr"),
    (0, "Porsche 991 GT3R"),
    (23, "Porsche 991-II GT3R"),
    (34, "Porsche 992 GT3R"),
    (9, "Porsche 991.2 GT3Cup GTC"),
    (28, "Porsche 992 GT3Cup GTC"),
];