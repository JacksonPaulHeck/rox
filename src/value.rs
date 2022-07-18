use crate::*;

macro_rules! ternary {
    ($test:expr => $true_expr:expr; $false_expr:expr) => {
        if $test {
            $true_expr
        } else {
            $false_expr
        }
    };
}

#[derive(Copy, Clone)]
pub union Union {
    boolean: bool,
    number: i64,
    obj: object::ObjString,
}

impl Union {
    pub fn new() -> Union {
        return Union { boolean: false };
    }

    pub fn create_bool(value: bool) -> Union {
        return Union { boolean: value };
    }

    pub fn create_num(value: i64) -> Union {
        return Union { number: value };
    }

    pub fn create_obj(value:object::ObjString) -> Union {
        return Union { obj: value };
    }

    pub fn get_boolean(&self) -> bool {
        unsafe {
            return self.boolean;
        }
    }

    pub fn get_number(&self) -> i64 {
        unsafe {
            return self.number;
        }
    }

    pub fn get_obj(&mut self) -> object::ObjString {
        unsafe {
            return self.obj;
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum ValueType {
    ValBool,
    ValNil,
    ValNumber,
    ValObj,
}

#[derive(Copy, Clone)]
pub struct Value {
    value_type: ValueType,
    value: Union,
}

impl Value {
    pub fn new() -> Value {
        return Value {
            value_type: ValueType::ValNil,
            value: Union::new(),
        };
    }

    pub fn create(value_type: ValueType, union: Union) -> Value {
        return Value {
            value_type: value_type,
            value: union,
        };
    }

    pub fn get_value(&self) -> Union {
        return self.value;
    }

    pub fn get_value_type(&self) -> ValueType {
        return self.value_type;
    }

    pub fn equal(&self, other: Value) -> bool {
        if self.value_type != other.value_type {
            return false;
        }

        match self.value_type {
            ValueType::ValBool => unsafe { return self.value.boolean == other.value.boolean },
            ValueType::ValNil => return true,
            ValueType::ValNumber => unsafe { return self.value.number == other.value.number },
            ValueType::ValObj => unsafe { return self.value.obj == other.value.obj },
            _ => return false,
        }
    }

    pub fn print(&self) -> String {
        match &self.value_type {
            ValueType::ValBool => unsafe {
                return ternary!(self.value.boolean => "true".to_string(); "false".to_string());
            },
            ValueType::ValNumber => unsafe {
                return self.value.number.to_string();
            },
            ValueType::ValObj => unsafe {
                return self.value.obj.get_chars().into_iter().collect::<String>();
            },
            ValueType::ValNil => {
                return "nil".to_string();
            }
        }
    }

    pub fn write_value_array(&self, mut array: ValueArray) {
        array.values[array.count] = *self;
    }
}

#[derive(Copy, Clone)]
pub struct ValueArray {
    capacity: usize,
    count: usize,
    values: [Value; 256],
}

impl ValueArray {
    pub fn new() -> ValueArray {
        return ValueArray {
            capacity: 0,
            count: 0,
            values: [Value::new(); 256],
        };
    }

    pub fn get_value(&self, i: usize) -> Value {
        return self.values[i];
    }

    pub fn get_values(&self) -> [Value; 256] {
        return self.values;
    }

    pub fn get_count(&self) -> usize {
        return self.count;
    }
}
