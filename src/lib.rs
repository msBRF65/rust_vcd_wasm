use std::{fs::File, io::BufReader};
use vcd::{Command, Parser};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct SignalDataScalar {
    id_code: String,
    value: vcd::Value,
}
#[wasm_bindgen]
pub struct SignalDataVector {
    id_code: String,
    value: vcd::Vector,
}
#[wasm_bindgen]
pub struct SignalDataReal {
    id_code: String,
    value: f64,
}
#[wasm_bindgen]
pub struct SignalDataString {
    id_code: String,
    value: String,
}

#[wasm_bindgen]
pub struct SignalDataAndTime {
    time: u64,
    data_array_scalar: Vec<SignalDataScalar>,
    data_array_vector: Vec<SignalDataVector>,
    data_array_real: Vec<SignalDataReal>,
    data_array_string: Vec<SignalDataString>,
}

#[wasm_bindgen]
pub struct VcdData {
    header: vcd::Header,
    signal: Vec<SignalDataAndTime>,
}

#[wasm_bindgen]
pub fn parse(vcd_path: &str) -> VcdData {
    let file = File::open("/home/members/saito/elastic_cgra_mapper/output/cgra.vcd");
    let file = match file {
        Ok(file) => file,
        Err(error) => panic!("error !"),
    };

    let mut reader = Parser::new(BufReader::new(file));
    let header = reader.parse_header();
    let header = match header {
        Ok(header) => header,
        Err(error) => panic!("vcd header is invalid"),
    };

    const INIT_SIGNAL_DATA_AND_TIME: SignalDataAndTime = SignalDataAndTime {
        time: 0,
        data_array_scalar: Vec::new(),
        data_array_vector: Vec::new(),
        data_array_real: Vec::new(),
        data_array_string: Vec::new(),
    };

    let mut signal: Vec<SignalDataAndTime> = Vec::new();
    let mut tmp_signal_data_and_time = INIT_SIGNAL_DATA_AND_TIME;
    while let Some(command) = reader.next().transpose().ok() {
        match command.unwrap() {
            Command::Timestamp(t) => {
                if (tmp_signal_data_and_time.data_array_scalar.len()
                    + tmp_signal_data_and_time.data_array_vector.len()
                    + tmp_signal_data_and_time.data_array_real.len()
                    + tmp_signal_data_and_time.data_array_string.len()
                    > 0)
                {
                    signal.push(tmp_signal_data_and_time);
                }

                tmp_signal_data_and_time = INIT_SIGNAL_DATA_AND_TIME;
                tmp_signal_data_and_time.time = t;
            }
            Command::ChangeScalar(i, v) => {
                let data = SignalDataScalar {
                    id_code: i.to_string(),
                    value: v,
                };
                tmp_signal_data_and_time.data_array_scalar.push(data);
            }
            Command::ChangeVector(i, v) => {
                let data = SignalDataVector {
                    id_code: i.to_string(),
                    value: v,
                };
                tmp_signal_data_and_time.data_array_vector.push(data);
            }
            Command::ChangeReal(i, v) => {
                let data = SignalDataReal {
                    id_code: i.to_string(),
                    value: v,
                };
                tmp_signal_data_and_time.data_array_real.push(data);
            }
            Command::ChangeString(i, v) => {
                let data = SignalDataString {
                    id_code: i.to_string(),
                    value: v,
                };
                tmp_signal_data_and_time.data_array_string.push(data);
            }
            command => println!(
                "Unexpected {command:?} at line {line}",
                line = reader.line()
            ),
        }
    }

    let vcd_data = VcdData {
        header: header,
        signal: signal,
    };

    return vcd_data;
}