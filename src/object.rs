use crate::*;

#[derive(Copy, Clone, Default, PartialEq)]
pub enum ObjType {
    #[default]
    ObjectString,
}

#[derive(Copy, Clone, PartialEq)]
pub struct ObjString {
    obj: Obj,
    length: usize,
    chars: [char; 256],
    hash: u32,
}

impl ObjString {
    pub fn new() -> ObjString {
        return ObjString {
            obj: Obj::new(),
            length: 0,
            chars: ['\0'; 256],
            hash: 0,
        };
    }

    pub fn create(obj: Obj, length: usize, chars: [char; 256], hash: u32) -> ObjString {
        return ObjString {
            obj: obj,
            length: length,
            chars: chars,
            hash: hash,
        }
    }

    pub fn copy(obj_string: ObjString) -> ObjString {
    return ObjString {
        obj: obj_string.obj,
        length: obj_string.length,
        chars: obj_string.chars,
        hash: obj_string.hash,
    }
} 
    pub fn get_type(&self) -> ObjType {
        return ObjType::ObjectString;
    }
    pub fn get_obj(&self) -> Obj {
        return self.obj;
    }

    pub fn get_length(&self) -> usize {
        return self.length;
    }

    pub fn get_chars(&self) -> [char; 256] {
        return self.chars;
    } 
}

#[derive(Copy, Clone, Default, PartialEq)]
pub struct Obj {
    obj_type: ObjType,
}

impl Obj {
    pub fn new() -> Obj {
        return Obj {
            obj_type: ObjType::ObjectString,
        };
    }

    pub fn get_type(&self) -> ObjType {
        return self.obj_type;
    }
}

fn hash_string(chars: [char; 256], length: usize) -> u32 {
    let mut hash: u32 = 21;
    for i in 0..length {
        hash ^= chars[i] as u32;
        hash = (hash * 16) as u32;
    }
    return hash;
}

fn allocate_string(chars: [char; 256], length: usize, hash: u32) ->  ObjString {

    let string: ObjString = object::ObjString::create(object::Obj::new(), length, chars, hash);

    return string;
}

pub fn copy_string(vm: &mut vm::VM, table: &mut table::Table, chars: [char; 256], length: usize) -> ObjString {
    let hash: u32 = hash_string(chars, length);

    let interned = table.table_find_string(chars, length, hash);

    match interned {
        Some(value) => return ObjString::copy(value),
        None => {
            return allocate_string(chars, length, hash);
        }
    }

}
