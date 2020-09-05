use std::fmt::Debug;
use std::collections::BTreeMap;

use regex::Regex;
use scraper::element_ref::ElementRef;

use crate::error::Result;
use crate::{element_within, inner_text};
use crate::Page;

type Specification<T> = Option<(T, String)>;

pub(crate) struct Vehicle {
    aspiration: Option<String>,
    body_type: Option<String>,
    bore_stroke: Specification<String>,
    carfolio_id: Option<String>,
    compression_ratio: Option<String>,
    curb_weight: Specification<u16>,
    displacement: Specification<f32>,
    door_count: Option<u8>,
    drive_wheel_config: Option<String>,
    engine_code: Option<String>,
    engine_config: Option<String>,
    engine_construction: Option<String>,
    engine_coolant: Option<String>,
    engine_layout: Option<String>,
    engine_manufacturer: Option<String>,
    engine_position: Option<String>,
    engine_type: Option<String>,
    final_drive_ratio: Option<f32>,
    fuel_capacity: Specification<f32>,
    ground_clearance: Specification<u16>,
    height: Specification<u16>,
    length: Specification<u16>,
    mpg: Option<(f32, f32, f32)>,
    power: BTreeMap<String, Specification<u16>>,
    power_to_weight_ratio: Specification<f32>,
    steering_config: Option<String>,
    tires: BTreeMap<String, Option<String>>,
    top_gear_ratio: Option<f32>,
    torque: BTreeMap<String, Specification<u16>>,
    track: BTreeMap<String, Specification<u16>>,
    transmission: Option<String>,
    valve_config: Option<String>,
    weight_to_power_ratio: Specification<f32>,
    wheel_size: BTreeMap<String, Option<String>>,
    wheelbase: Specification<u16>,
    width: Specification<u16>
}

impl Vehicle {
    pub(super) fn new(page: Page) -> Result<Vehicle> {
        let overview = element_within(page.html.root_element(), &["div h3 span.automobile"])?;
        let make = extract_model_make(overview)?;
        let model = extract_model_name(overview)?;
        let year = extract_model_year(overview)?;
        info!("Parsing Model specifications for {} {} {}", year, make, model);

        let mut specifications = extract_model_specifications_table(page)?;
        info!("Specifications for {} {} {}:\n{:#?}", year, make, model, specifications);

        let vehicle = Vehicle {
            aspiration: specification(&mut specifications, "aspiration", extract_string),

            body_type: specification(&mut specifications, "body_type", extract_string),

            bore_stroke: specification(&mut specifications, "bore_×_stroke", extract_bore_stroke),

            carfolio_id: specification(&mut specifications, "carfolio.com_id", extract_string),

            compression_ratio: specification(&mut specifications, "compression_ratio", extract_string),

            curb_weight: specification(&mut specifications, "kerb_weight", extract_u16_with_unit),

            displacement: specification(&mut specifications, "capacity", extract_displacement),

            door_count: specification(&mut specifications, "number_of_doors", extract_u8),

            drive_wheel_config: specification(&mut specifications, "drive_wheels", extract_string),

            engine_code: specification(&mut specifications, "engine_code", extract_string),

            engine_config: specification(&mut specifications, "cylinders", extract_string),

            engine_construction: specification(&mut specifications, "engine_construction", extract_string),

            engine_coolant: specification(&mut specifications, "engine_coolant", extract_string),

            engine_layout: specification(&mut specifications, "engine_layout", extract_string),

            engine_manufacturer: specification(&mut specifications, "engine_manufacturer", extract_string),

            engine_position: specification(&mut specifications, "engine_position", extract_string),

            engine_type: specification(&mut specifications, "engine_type", extract_string),

            final_drive_ratio: specification(&mut specifications, "final_drive_ratio", extract_f32),

            fuel_capacity: specification(&mut specifications, "fuel_tank_capacity", extract_f32_with_unit),

            ground_clearance: specification(&mut specifications, "ground_clearance", extract_u16_with_unit),

            height: specification(&mut specifications, "height", extract_u16_with_unit),

            length: specification(&mut specifications, "length", extract_u16_with_unit),

            mpg: specification(&mut specifications, "us_mpg", extract_mpg),

            power: specification(&mut specifications, "maximum_power_output", extract_power).unwrap(),

            power_to_weight_ratio: specification(&mut specifications, "power-to-weight_ratio", extract_power_to_weight_ratio),

            steering_config: specification(&mut specifications, "steering", extract_string),

            tires: vec![
                ("Front".to_string(), specification(&mut specifications, "tyres_front", extract_string)),
                ("Rear".to_string(), specification(&mut specifications, "tyres_rear", extract_string))
            ].into_iter().collect(),

            top_gear_ratio: specification(&mut specifications, "top_gear_ratio", extract_f32),

            torque: specification(&mut specifications, "maximum_torque", extract_torque).unwrap(),

            track: vec![
                ("Front".to_string(), specification(&mut specifications, "track/tread_(front)", extract_u16_with_unit)),
                ("Rear".to_string(), specification(&mut specifications, "track/tread_(rear)", extract_u16_with_unit))
            ].into_iter().collect(),

            transmission: specification(&mut specifications, "gearbox", extract_string),

            valve_config: specification(&mut specifications, "valve_gear", extract_string),

            weight_to_power_ratio: specification(&mut specifications, "weight-to-power_ratio", extract_f32_with_unit),

            wheel_size: vec![
                ("Front".to_string(), specification(&mut specifications, "wheel_size_front", extract_string)),
                ("Rear".to_string(), specification(&mut specifications, "wheel_size_rear", extract_string))
            ].into_iter().collect(),

            wheelbase: specification(&mut specifications, "wheelbase", extract_u16_with_unit),

            width: specification(&mut specifications, "width", extract_u16_with_unit),
        };

        let unused_keys = specifications.iter().filter_map(|(k, v)| {
            if *v != "" {
                Some((k, v))
            } else {
                None
            }
        }).collect::<Vec<(&String, &String)>>();
        warn!("Unused fields from Details map: {:#?}", unused_keys);

        Ok(vehicle)
    }
}

fn specification<T: Debug>(map: &mut BTreeMap<String, String>, key: &str, parse_using: fn(String) -> Option<T>) -> Option<T> {
    let string = map.remove(key).unwrap_or_default();
    info!("{} unparsed: {}", key, string);
    let parsed_value = parse_using(string);

    if parsed_value.is_none() {
        warn!("{} was unable to be parsed", key);
    } else {
        info!("{} parsed: {:?}", key, parsed_value);
    }

    parsed_value
}

fn split_string(string: String) -> Vec<String> {
    let re = Regex::new(r"[, ]+").unwrap();
    re.split(&string).map(|s| s.to_string()).collect()
}

fn extract_mpg(string: String) -> Option<(f32, f32, f32)> {
    let split_string = split_string(string);
    let mpg_string = split_string.get(0)?;
    let mpg_splits: Vec<String> = mpg_string.split("/").map(|s| s.to_string()).collect();
    let city = mpg_splits.get(0)?.parse::<f32>().ok()?;
    let highway = mpg_splits.get(1)?.parse::<f32>().ok()?;
    let combined = mpg_splits.get(2)?.parse::<f32>().ok()?;
    
    Some((city, highway, combined))
}

fn extract_power_to_weight_ratio(string: String) -> Specification<f32> {
    let split: Vec<&str> = string.split(",").collect();
    let string = split.get(1)?;

    extract_f32_with_unit(String::from(string.trim()))
}

fn extract_torque(string: String) -> Option<BTreeMap<String, Specification<u16>>> {
    extract_power_or_torque(string, Regex::new(r"(\d+ Nm).*\s(\d+ rpm)").unwrap())
}

fn extract_power(string: String) -> Option<BTreeMap<String, Specification<u16>>> {
    extract_power_or_torque(string, Regex::new(r"(\d+ kW).*\s(\d+ rpm)").unwrap())
}

fn extract_power_or_torque(string: String, re: Regex) -> Option<BTreeMap<String, Specification<u16>>> {
    let mut map = BTreeMap::new();
    let mut value: Specification<u16> = None;
    let mut rpm: Specification<u16> = None;

    match re.captures(&string) {
        Some(caps) => {
            value = match caps.get(1) {
                Some(str) => extract_u16_with_unit(str.as_str().to_string()),
                None      => {
                    warn!("Value was unable to be parsed from '{}' with regex '{}'", string, re);
                    None
                }
            };
            rpm = match caps.get(2) {
                Some(str) => extract_u16_with_unit(str.as_str().to_string()),
                None      => {
                    warn!("RPM was unable to be parsed from '{}' with regex '{}'", string, re);
                    None
                }
            };
        },
        None => warn!("Unable to find matches in '{}' with regex '{}'", string, re)
    };

    map.insert(String::from("Value"), value);
    map.insert(String::from("RPM"), rpm);
    Some(map)
}

fn extract_bore_stroke(string: String) -> Specification<String> {
    extract_string_with_unit(string.replace(" x ", "x"))
}

fn extract_displacement(string: String) -> Specification<f32> {
    let re = Regex::new(r"(\d+.\d+ litre)").unwrap();
    match re.captures(&string) {
        Some(caps) => {
            match caps.get(1) {
                Some(str) => extract_f32_with_unit(str.as_str().to_string()),
                None      => {
                    warn!("Could not parse displacement from '{}' with regex '{}'", string, re);
                    None
                }
            }
        },
        None => {
            warn!("Could not parse displacement from '{}' with regex '{}'", string, re);
            None
        }
    }
}

fn extract_string(string: String) -> Option<String> {
    Some(string)
}

fn extract_string_with_unit(string: String) -> Specification<String> {
    let splits = split_string(string);
    let amount_str = splits.get(0)?;
    let unit = splits.get(1)?;
    Some((String::from(amount_str), String::from(unit)))
}

fn extract_f32(string: String) -> Option<f32> {
    let amount = string.parse::<f32>().ok()?;
    Some(amount)
}

fn extract_f32_with_unit(string: String) -> Specification<f32> {
    let (amount_str, unit) = extract_string_with_unit(string)?;
    let amount = amount_str.parse::<f32>().ok()?;
    Some((amount, unit))
}

fn extract_u8(string: String) -> Option<u8> {
    let amount = string.parse::<u8>().ok()?;
    Some(amount)
}

fn extract_u16_with_unit(string: String) -> Specification<u16> {
    let (amount_str, unit) = extract_string_with_unit(string)?;
    let amount = amount_str.parse::<u16>().ok()?;
    Some((amount, unit))
}

fn extract_model_year(span: ElementRef) -> Result<String> {
    let elem = element_within(span, &["span.Year", "span.modelyear"])?;
    Ok(inner_text(elem)[..4].to_string())
}

fn extract_model_make(span: ElementRef) -> Result<String> {
    let elem = element_within(span, &["span.manufacturer"])?;
    Ok(elem.inner_html())
}

fn extract_model_name(span: ElementRef) -> Result<String> {
    let elem = element_within(span, &["span.model.name"])?;
    Ok(elem.inner_html())
}

fn extract_model_specifications_table(page: Page) -> Result<BTreeMap<String, String>> {
    let mut table = BTreeMap::new();
    for row in page.elements("table.specstable tbody tr") {
        let spec_name = element_within(row, &["th:not(.sechead)"]);

        if spec_name.is_ok() {
            let spec_name = lower_underscore(inner_text(spec_name.unwrap()));
            if spec_name != "" {
                let td = match element_within(row, &["td"]) {
                    Ok(string) => sanitize_text(inner_text(string)),
                    Err(_)    => {
                        warn!("Unable to find `td` for `row`:\n{}", row.html());
                        "".to_string()
                    }
                };
                table.insert(spec_name, td);
            }
        };
    }

    Ok(table)
}

fn sanitize_text(string: String) -> String {
    string.trim().replace("\n", ", ").replace("×", "x").replace("No information available", "")
}

fn lower_underscore(string: String) -> String {
    string.to_lowercase().replace(" ", "_")
}
