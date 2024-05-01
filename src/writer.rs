use std::{collections::HashMap, path::Path};

use crate::parser::Object;

//k8s manifest come as YAML but we don't know 
//maybe in the feature they might be XML though that'll be a backward shit-move
//but it's fine this way...
pub trait YamlWriter {
    fn yaml_pretty_print(
        &mut self,
        dict: &HashMap<String, Object>,
        result: &mut String,
        prefix: &str,
    );
}

#[derive(Debug)]
pub struct OutputWriter {
    pub output_dict: Object,
    pub output_result: String,
}

impl OutputWriter {
    pub fn new(obj: Object) -> Self {
        Self {
            output_dict: obj,
            output_result: String::new(),
        }
    }

    fn write_output<P: AsRef<Path>>(&mut self, path: P) {
        //actual todo of the function here...
        std::fs::write(path, self.output_result.clone()).expect("Could not write output yaml file");
    }

    pub fn write_output_yaml<P: AsRef<Path>>(&mut self, path: P) {
        let mut result: String = String::new();
        let dict = self.output_dict.clone().into();
        self.yaml_pretty_print(&dict, &mut result, "");
        self.write_output(path);
    }
}

impl YamlWriter for OutputWriter {
    fn yaml_pretty_print(
        &mut self,
        dict: &HashMap<String, Object>,
        result: &mut String,
        prefix: &str,
    ) {
        if dict.is_empty() {
            return;
        }

        for (key, value) in dict {
            let key_with_prefix = format!("{}{}", prefix, key);
            match value {
                Object::Dictionary(d) => {
                    //value.pretty_print(result, &format!("{}__", key_with_prefix));
                    let de = d;
                    self.yaml_pretty_print(de, result, &format!("{}__", key_with_prefix));
                }
                _ => {
                    //writeln!(result, "{}: \"{}\"", key_with_prefix, value).unwrap();
                    result.push_str(format!("{}:\"{}\"\n", key_with_prefix, value).as_str());
                    self.output_result.push_str(
                        format!("  - name: {}\n  - value: {}\n", key_with_prefix, value).as_str(),
                    );
                }
            }
            result.clear();
        }
    }
}
