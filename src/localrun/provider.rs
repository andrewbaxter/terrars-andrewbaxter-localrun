use serde::Serialize;

#[derive(Serialize)]
struct ProviderLocalrunData {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alias: std::option::Option<std::string::String>,
}

struct ProviderLocalrun_ {
    data: std::cell::RefCell<ProviderLocalrunData>,
}

pub struct ProviderLocalrun(std::rc::Rc<ProviderLocalrun_>);

impl ProviderLocalrun {
    pub fn provider_ref(&self) -> std::string::String {
        let data = self.0.data.borrow();
        if let std::option::Option::Some(alias) = &data.alias {
            format!("{}.{}", "localrun", alias)
        } else {
            "localrun".into()
        }
    }

    pub fn set_alias(self, alias: impl std::string::ToString) -> Self {
        self.0.data.borrow_mut().alias = std::option::Option::Some(alias.to_string());
        self
    }
}

impl terrars::Provider for ProviderLocalrun_ {
    fn extract_type_tf_id(&self) -> std::string::String {
        "localrun".into()
    }

    fn extract_provider_type(&self) -> serde_json::Value {
        serde_json::json!({
            "source": "andrewbaxter/localrun",
            "version": "0.1.0",
        })
    }

    fn extract_provider(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildProviderLocalrun {}

impl BuildProviderLocalrun {
    pub fn build(self, stack: &mut terrars::Stack) -> ProviderLocalrun {
        let out =
            ProviderLocalrun(
                std::rc::Rc::new(
                    ProviderLocalrun_ {
                        data: std::cell::RefCell::new(ProviderLocalrunData { alias: std::option::Option::None }),
                    },
                ),
            );
        stack.add_provider(out.0.clone());
        out
    }
}
