#[derive(Debug)]
#[derive(PartialEq)]
pub enum FileType {
    XML,
    Binary
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Value {
    StringValue,
    BoolValue,
    NumberValue,
    IntValue
}

impl FileType {
    pub fn detect(file_type: &str) -> Result<FileType, &str> {
        match file_type {
            "xml" | "rbxmx" => Ok(FileType::XML),
            "binary" | "rbxm" => Ok(FileType::Binary), 
            _ => Err("Invalid file type: only valid values are xml, rbxmx, binary, and rbxm")
        }
    }
}

impl Value {
    pub fn detect(input: &str) -> Result<Value, &str> {
        match input {
            "string" => Ok(Value::StringValue),
            "bool" => Ok(Value::BoolValue),
            "number" => Ok(Value::NumberValue),
            "int" => Ok(Value::IntValue),
            _ => Err("Invalid value input: only valid values are string, bool, number, and int")
        }
    }
}