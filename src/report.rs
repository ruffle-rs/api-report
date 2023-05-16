use std::fs::File;
use std::path::Path;
use fnv::FnvHashMap;
use serde::Serialize;
use crate::specification::{Definition, TraitList};
use anyhow::Result;

#[derive(Serialize, Default)]
pub struct Report {
    summary: Summary,
    classes: FnvHashMap<String, ClassInfo>,
}

impl Report {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn write(&self, path: &Path) -> Result<()> {
        serde_json::to_writer_pretty(File::create(path)?, self)?;
        Ok(())
    }

    pub fn compare_class(&mut self, class_name: &str, specification: &Definition, implementation: Option<&Definition>) {
        let mut class_info = ClassInfo::default();

        self.summary.max_points += 1;
        class_info.summary.max_points += 1;
        if implementation.is_some() {
            self.summary.impl_points += 1;
            class_info.summary.impl_points += 1;
        }

        if let Some(traits) = &specification.instance_traits {
            self.compare_traits(&mut class_info, traits, implementation.and_then(|imp| imp.instance_traits.as_ref()), "");
        }
        if let Some(traits) = &specification.static_traits {
            self.compare_traits(&mut class_info, traits, implementation.and_then(|imp| imp.static_traits.as_ref()), "static ");
        }
        if let Some(traits) = &specification.prototype {
            self.compare_traits(&mut class_info, traits, implementation.and_then(|imp| imp.prototype.as_ref()), "prototype.");
        }

        self.classes.insert(class_name.to_string(), class_info);
    }

    fn compare_traits(&mut self, class_info: &mut ClassInfo, specification: &TraitList, implementation: Option<&TraitList>, prefix: &str) {
        let imp = implementation.map(|imp| imp.names()).unwrap_or_default();

        for (name, (_, suffix)) in specification.names() {
            self.summary.max_points += 1;
            class_info.summary.max_points += 1;

            if let Some((stubbed, _)) = imp.get(name) {
                self.summary.impl_points += 1;
                class_info.summary.impl_points += 1;
                if *stubbed {
                    self.summary.stub_penalty += 1;
                    class_info.summary.stub_penalty += 1;
                    class_info.stubbed.push(prefix.to_string() + name + suffix);
                }
            } else {
                class_info.missing.push(prefix.to_string() + name + suffix);
            }
        }
    }
}

#[derive(Serialize, Default)]
struct Summary {
    max_points: u32,
    impl_points: u32,
    stub_penalty: u32,
}

#[derive(Serialize, Default)]
struct ClassInfo {
    summary: Summary,
    missing: Vec<String>,
    stubbed: Vec<String>,
}