use fnv::{FnvHashMap};
use serde::Deserialize;

pub type Specification = FnvHashMap<String, Definition>;

#[derive(Deserialize, Default)]
#[serde(default, deny_unknown_fields)]
pub struct Definition {
    pub classinfo: Option<ClassInfo>,
    #[serde(rename = "static")]
    pub static_traits: Option<TraitList>,
    #[serde(rename = "instance")]
    pub instance_traits: Option<TraitList>,
    #[serde(rename = "prototype")]
    pub prototype: Option<TraitList>,
    pub constructor: Option<ConstructorInfo>,
}

#[derive(Deserialize, Default)]
#[serde(default, deny_unknown_fields)]
pub struct TraitList {
    #[serde(rename = "const")]
    pub constants: FnvHashMap<String, VariableInfo>,
    #[serde(rename = "var")]
    pub variables: FnvHashMap<String, VariableInfo>,
    pub function: FnvHashMap<String, FunctionInfo>,
    pub getter: FnvHashMap<String, VariableInfo>,
    pub setter: FnvHashMap<String, VariableInfo>,
}

impl TraitList {
    pub fn names(&self) -> FnvHashMap<&str, (bool, &'static str)> {
        let mut result: FnvHashMap<&str, (bool, &'static str)> = FnvHashMap::default();

        for (key, value) in self.constants.iter() {
            result.insert(key, (value.stubbed, ""));
        }
        for (key, value) in self.variables.iter() {
            result.insert(key, (value.stubbed, ""));
        }
        for (key, value) in self.function.iter() {
            result.insert(key, (value.stubbed, "()"));
        }
        for (key, value) in self.getter.iter() {
            result.insert(key, (value.stubbed, ""));
        }
        for (key, value) in self.setter.iter() {
            result.insert(key, (value.stubbed, ""));
        }

        result
    }
}

#[derive(Deserialize, Default)]
#[serde(default, deny_unknown_fields)]
pub struct ParamInfo {
    #[serde(rename = "type")]
    pub type_info: String,
    #[serde(rename = "default")]
    pub value: Option<String>,
    pub variadic: bool,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FunctionInfo {
    pub args: Vec<ParamInfo>,
    pub returns: String,
    #[serde(default)]
    pub stubbed: bool,
}

#[derive(Deserialize, Default)]
#[serde(default, deny_unknown_fields)]
pub struct ClassInfo {
    pub dynamic: bool,
    pub extends: Option<String>,
    pub implements: Option<String>,
    #[serde(rename = "final")]
    pub is_final: bool,
}

#[derive(Deserialize, Default)]
#[serde(default, deny_unknown_fields)]
pub struct VariableInfo {
    #[serde(rename = "type")]
    pub type_info: Option<String>,
    pub value: Option<String>,
    pub stubbed: bool,
}

#[derive(Deserialize, Default)]
#[serde(default, deny_unknown_fields)]
pub struct ConstructorInfo {
    pub args: Vec<ParamInfo>,
}