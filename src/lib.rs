use js_sys;
use vcd::{Command, Parser};
use wasm_bindgen::prelude::*;

extern crate console_error_panic_hook;

#[wasm_bindgen]
#[derive(Clone)]
pub enum VcdValue {
    V0 = 0,
    V1 = 1,
    X = 2,
    Z = 3,
}

fn get_vcd_value(input: vcd::Value) -> VcdValue {
    match input {
        vcd::Value::V0 => return VcdValue::V0,
        vcd::Value::V1 => return VcdValue::V1,
        vcd::Value::X => return VcdValue::X,
        vcd::Value::Z => return VcdValue::Z,
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct SignalDataScalar {
    id_code: String,
    value: VcdValue,
}

#[wasm_bindgen]
impl SignalDataScalar {
    pub fn new(_id_code: String, _value: VcdValue) -> SignalDataScalar {
        SignalDataScalar {
            id_code: _id_code,
            value: _value,
        }
    }

    pub fn get_id_code(&self) -> String {
        self.id_code.clone()
    }

    pub fn get_value(&self) -> VcdValue {
        self.value.clone()
    }
}

fn get_vcd_vector(input: vcd::Vector) -> Vec<VcdValue> {
    return input.iter().map(|ele| get_vcd_value((ele))).collect();
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct SignalDataVector {
    id_code: String,
    value: Vec<VcdValue>,
}

#[wasm_bindgen]
impl SignalDataVector {
    fn new(_id_code: String, _value: Vec<VcdValue>) -> SignalDataVector {
        SignalDataVector {
            id_code: _id_code,
            value: _value,
        }
    }

    pub fn get_id_code(&self) -> String {
        self.id_code.clone()
    }

    pub fn get_value_size(&self) -> usize {
        self.value.len()
    }

    pub fn get_value_data(&self, i: usize) -> VcdValue {
        self.value[i].clone()
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct SignalDataReal {
    id_code: String,
    value: f64,
}

#[wasm_bindgen]
impl SignalDataReal {
    pub fn new(_id_code: String, _value: f64) -> SignalDataReal {
        SignalDataReal {
            id_code: _id_code,
            value: _value,
        }
    }

    pub fn get_id_code(&self) -> String {
        self.id_code.clone()
    }

    pub fn get_value(&self) -> f64 {
        self.value.clone()
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct SignalDataString {
    id_code: String,
    value: String,
}

#[wasm_bindgen]
impl SignalDataString {
    pub fn new(_id_code: String, _value: String) -> SignalDataString {
        SignalDataString {
            id_code: _id_code,
            value: _value,
        }
    }

    pub fn get_id_code(&self) -> String {
        self.id_code.clone()
    }

    pub fn get_value(&self) -> String {
        self.value.clone()
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct SignalDataAndTime {
    time: u64,
    data_array_scalar: Vec<SignalDataScalar>,
    data_array_vector: Vec<SignalDataVector>,
    data_array_real: Vec<SignalDataReal>,
    data_array_string: Vec<SignalDataString>,
}

#[wasm_bindgen]
impl SignalDataAndTime {
    fn new(
        _time: u64,
        _data_array_scalar: Vec<SignalDataScalar>,
        _data_array_vector: Vec<SignalDataVector>,
        _data_array_real: Vec<SignalDataReal>,
        _data_array_string: Vec<SignalDataString>,
    ) -> SignalDataAndTime {
        SignalDataAndTime {
            time: _time,
            data_array_scalar: _data_array_scalar,
            data_array_vector: _data_array_vector,
            data_array_real: _data_array_real,
            data_array_string: _data_array_string,
        }
    }

    pub fn get_time(&self) -> u64 {
        self.time
    }

    pub fn get_data_array_scalar_size(&self) -> usize {
        self.data_array_scalar.len()
    }

    pub fn get_data_array_scalar_id_code(&self, i: usize) -> String {
        self.data_array_scalar[i].get_id_code()
    }

    pub fn get_data_array_scalar_value(&self, i: usize) -> VcdValue {
        self.data_array_scalar[i].get_value()
    }

    pub fn get_data_array_vector_size(&self) -> usize {
        self.data_array_vector.len()
    }

    pub fn get_data_array_vector_id_code(&self, i: usize) -> String {
        self.data_array_vector[i].get_id_code()
    }

    pub fn get_data_array_vector_value_size(&self, i: usize) -> usize {
        self.data_array_vector[i].get_value_size()
    }

    pub fn get_data_array_vector_value_data(&self, i: usize, j: usize) -> VcdValue {
        self.data_array_vector[i].get_value_data(j)
    }

    pub fn get_data_array_real_size(&self) -> usize {
        self.data_array_real.len()
    }

    pub fn get_data_array_real_id_code(&self, i: usize) -> String {
        self.data_array_real[i].get_id_code()
    }

    pub fn get_data_array_real_value(&self, i: usize) -> f64 {
        self.data_array_real[i].get_value()
    }

    pub fn get_data_array_string_size(&self) -> usize {
        self.data_array_string.len()
    }

    pub fn get_data_array_string_id_code(&self, i: usize) -> String {
        self.data_array_string[i].get_id_code()
    }

    pub fn get_data_array_string_value(&self, i: usize) -> String {
        self.data_array_string[i].get_value()
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub enum VcdTimescaleUnit {
    S = 0,
    MS = 1,
    US = 2,
    NS = 3,
    PS = 4,
    FS = 5,
}

#[wasm_bindgen]
#[derive(Clone)]
pub enum VcdScopeType {
    Module = 0,
    Task = 1,
    Function = 2,
    Begin = 3,
    Fork = 4,
    Other = 5,
}

fn get_vcd_scope_type(input: vcd::ScopeType) -> VcdScopeType {
    match input {
        vcd::ScopeType::Module => return VcdScopeType::Module,
        vcd::ScopeType::Task => return VcdScopeType::Task,
        vcd::ScopeType::Function => return VcdScopeType::Function,
        vcd::ScopeType::Begin => return VcdScopeType::Begin,
        vcd::ScopeType::Fork => return VcdScopeType::Fork,
        _ => return VcdScopeType::Other,
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct VcdScope {
    scope_type: VcdScopeType,
    identifier: String,
    items: Vec<VcdScopeItem>,
}

#[wasm_bindgen]
impl VcdScope {
    fn new(_scope_type: VcdScopeType, _identifier: String, _items: Vec<VcdScopeItem>) -> VcdScope {
        VcdScope {
            scope_type: _scope_type,
            identifier: _identifier,
            items: _items,
        }
    }

    pub fn get_scope_type(&self) -> VcdScopeType {
        self.scope_type.clone()
    }

    pub fn get_identifier(&self) -> String {
        self.identifier.clone()
    }

    pub fn get_items_size(&self) -> usize {
        self.items.len()
    }

    pub fn get_items_data(&self, i: usize) -> VcdScopeItem {
        self.items[i].clone()
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct VcdScopeItem {
    scope: Option<VcdScope>,
    var: Option<VcdVar>,
}

#[wasm_bindgen]
impl VcdScopeItem {
    fn new(_scope: Option<VcdScope>, _var: Option<VcdVar>) -> VcdScopeItem {
        VcdScopeItem {
            scope: _scope,
            var: _var,
        }
    }

    pub fn get_scope(&self) -> Option<VcdScope> {
        self.scope.clone()
    }

    pub fn get_var(&self) -> Option<VcdVar> {
        self.var.clone()
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub enum VcdVarType {
    Event = 0,
    Integer = 1,
    Parameter = 2,
    Real = 3,
    Reg = 4,
    Supply0 = 5,
    Supply1 = 6,
    Time = 7,
    Tri = 8,
    TriAnd = 9,
    TriOr = 10,
    TriReg = 11,
    Tri0 = 12,
    Tri1 = 13,
    WAnd = 14,
    Wire = 15,
    WOr = 16,
    String = 17,
    Other = 18,
}

fn get_vcd_var_type(input: vcd::VarType) -> VcdVarType {
    match input {
        vcd::VarType::Event => return VcdVarType::Event,
        vcd::VarType::Integer => return VcdVarType::Integer,
        vcd::VarType::Parameter => return VcdVarType::Parameter,
        vcd::VarType::Real => return VcdVarType::Real,
        vcd::VarType::Reg => return VcdVarType::Reg,
        vcd::VarType::Supply0 => return VcdVarType::Supply0,
        vcd::VarType::Supply1 => return VcdVarType::Supply1,
        vcd::VarType::Time => return VcdVarType::Time,
        vcd::VarType::Tri => return VcdVarType::Tri,
        vcd::VarType::TriAnd => return VcdVarType::TriAnd,
        vcd::VarType::TriOr => return VcdVarType::TriOr,
        vcd::VarType::TriReg => return VcdVarType::TriReg,
        vcd::VarType::Tri0 => return VcdVarType::Tri0,
        vcd::VarType::Tri1 => return VcdVarType::Tri1,
        vcd::VarType::WAnd => return VcdVarType::WAnd,
        vcd::VarType::Wire => return VcdVarType::Wire,
        vcd::VarType::WOr => return VcdVarType::WOr,
        vcd::VarType::String => return VcdVarType::String,
        _ => return VcdVarType::Other,
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct VcdVar {
    var_type: VcdVarType,
    size: u32,
    code: String,
    reference: String,
    index_range_begin: i32,
    index_range_end: i32,
}

#[wasm_bindgen]
impl VcdVar {
    fn new(
        _var_type: VcdVarType,
        _size: u32,
        _code: String,
        _reference: String,
        _index_range_begin: i32,
        _index_range_end: i32,
    ) -> VcdVar {
        VcdVar {
            var_type: _var_type,
            size: _size,
            code: _code,
            reference: _reference,
            index_range_begin: _index_range_begin,
            index_range_end: _index_range_end,
        }
    }

    pub fn get_var_type(&self) -> VcdVarType {
        self.var_type.clone()
    }

    pub fn get_size(&self) -> u32 {
        self.size.clone()
    }

    pub fn get_var_code(&self) -> String {
        self.code.clone()
    }

    pub fn get_reference(&self) -> String {
        self.reference.clone()
    }

    pub fn get_index_range_begin(&self) -> i32 {
        self.index_range_begin.clone()
    }

    pub fn get_index_range_end(&self) -> i32 {
        self.index_range_end.clone()
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct VcdHeader {
    date: String,
    version: String,
    timescale_value: u32,
    timescale_unit: VcdTimescaleUnit,
    items: Vec<VcdScopeItem>,
}

#[wasm_bindgen]
impl VcdHeader {
    fn new(
        _date: String,
        _version: String,
        _timescale_value: u32,
        _timescale_unit: VcdTimescaleUnit,
        _items: Vec<VcdScopeItem>,
    ) -> VcdHeader {
        VcdHeader {
            date: _date,
            version: _version,
            timescale_value: _timescale_value,
            timescale_unit: _timescale_unit,
            items: _items,
        }
    }

    pub fn get_date(&self) -> String {
        self.date.clone()
    }

    pub fn get_version(&self) -> String {
        self.version.clone()
    }
    pub fn get_timescale_value(&self) -> u32 {
        self.timescale_value.clone()
    }
    pub fn get_timescale_unit(&self) -> VcdTimescaleUnit {
        self.timescale_unit.clone()
    }
    pub fn get_items_size(&self) -> usize {
        self.items.len()
    }

    pub fn get_items_data(&self, i: usize) -> VcdScopeItem {
        self.items[i].clone()
    }
}

#[wasm_bindgen]
pub struct VcdData {
    header: VcdHeader,
    signal: Vec<SignalDataAndTime>,
}

#[wasm_bindgen]
impl VcdData {
    fn new(_header: VcdHeader, _signal: Vec<SignalDataAndTime>) -> VcdData {
        VcdData {
            header: _header,
            signal: _signal,
        }
    }

    pub fn get_vcd_header(&self) -> VcdHeader {
        self.header.clone()
    }

    pub fn get_vcd_signal_size(&self) -> usize {
        self.signal.len()
    }

    pub fn get_vcd_signal_data(&self, i: usize) -> SignalDataAndTime {
        self.signal[i].clone()
    }
}

fn get_vcd_time_scale_unit(input: vcd::TimescaleUnit) -> VcdTimescaleUnit {
    match input {
        vcd::TimescaleUnit::S => return VcdTimescaleUnit::S,
        vcd::TimescaleUnit::MS => return VcdTimescaleUnit::MS,
        vcd::TimescaleUnit::US => return VcdTimescaleUnit::US,
        vcd::TimescaleUnit::NS => return VcdTimescaleUnit::NS,
        vcd::TimescaleUnit::PS => return VcdTimescaleUnit::PS,
        vcd::TimescaleUnit::FS => return VcdTimescaleUnit::FS,
    }
}

fn get_vcd_scope_item(input: vcd::ScopeItem) -> VcdScopeItem {
    let mut scope: Option<VcdScope> = None;
    let mut var: Option<VcdVar> = None;
    match input {
        vcd::ScopeItem::Scope(s) => scope = Some(get_vcd_scope(s)),
        vcd::ScopeItem::Var(v) => var = Some(get_vcd_var(v)),
        vcd::ScopeItem::Comment(_) => {}
        _ => {}
    }

    let result = VcdScopeItem {
        scope: scope,
        var: var,
    };

    return result;
}

fn get_vcd_scope_items(input: Vec<vcd::ScopeItem>) -> Vec<VcdScopeItem> {
    let mut result: Vec<VcdScopeItem> = Vec::new();
    input.iter().for_each(|ele| {
        result.push(get_vcd_scope_item(ele.clone()));
    });

    return result;
}

fn get_vcd_scope(input: vcd::Scope) -> VcdScope {
    let result = VcdScope {
        scope_type: get_vcd_scope_type(input.scope_type),
        identifier: input.identifier,
        items: get_vcd_scope_items(input.items),
    };

    return result;
}

fn get_vcd_var(input: vcd::Var) -> VcdVar {
    let mut index_range_begin: i32 = 0;
    let mut index_range_end: i32 = 0;
    match input.index {
        None => {}
        ReferenceIndex => match input.index.unwrap() {
            vcd::ReferenceIndex::BitSelect(b) => {
                index_range_begin = b;
                index_range_end = b;
            }
            vcd::ReferenceIndex::Range(begin, end) => {
                index_range_begin = begin;
                index_range_end = end;
            }
        },
    }

    let result: VcdVar = VcdVar {
        var_type: get_vcd_var_type(input.var_type),
        size: input.size,
        code: input.code.to_string(),
        reference: input.reference,
        index_range_begin: index_range_begin,
        index_range_end: index_range_end,
    };

    return result;
}

fn get_vcd_header(input: vcd::Header) -> VcdHeader {
    let scope_items: Vec<VcdScopeItem> = Vec::new();
    let var_items: Vec<VcdVar> = Vec::new();

    let result = VcdHeader {
        date: input.date.unwrap(),
        version: input.version.unwrap(),
        timescale_value: input.timescale.unwrap().0,
        timescale_unit: get_vcd_time_scale_unit(input.timescale.unwrap().1),
        items: get_vcd_scope_items(input.items),
    };
    return result;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn parse(vcd_text: &str) -> VcdData {
    console_error_panic_hook::set_once();

    let mut reader = Parser::new(vcd_text.as_bytes());
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
        match command {
            None => {
                break;
            }
            Some(command) => match command {
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
                        value: get_vcd_value(v),
                    };
                    tmp_signal_data_and_time.data_array_scalar.push(data);
                }
                Command::ChangeVector(i, v) => {
                    let data = SignalDataVector {
                        id_code: i.to_string(),
                        value: get_vcd_vector(v),
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
            },
        }
    }

    let vcd_data = VcdData {
        header: get_vcd_header(header),
        signal: signal,
    };

    return vcd_data;
}
