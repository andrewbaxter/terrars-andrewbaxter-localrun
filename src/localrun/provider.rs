use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct ProviderLocalrunData {
    #[serde(skip_serializing_if = "Option::is_none")]
    alias: Option<String>,
}

struct ProviderLocalrun_ {
    data: RefCell<ProviderLocalrunData>,
}

pub struct ProviderLocalrun(Rc<ProviderLocalrun_>);

impl ProviderLocalrun {
    pub fn provider_ref(&self) -> String {
        let data = self.0.data.borrow();
        if let Some(alias) = &data.alias {
            format!("{}.{}", "localrun", alias)
        } else {
            "localrun".into()
        }
    }

    pub fn set_alias(self, alias: impl ToString) -> Self {
        self.0.data.borrow_mut().alias = Some(alias.to_string());
        self
    }
}

impl Provider for ProviderLocalrun_ {
    fn extract_type_tf_id(&self) -> String {
        "localrun".into()
    }

    fn extract_provider_type(&self) -> serde_json::Value {
        serde_json::json!({
            "source": "andrewbaxter/localrun",
            "version": "0.0.6",
        })
    }

    fn extract_provider(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildProviderLocalrun {}

impl BuildProviderLocalrun {
    pub fn build(self, stack: &mut Stack) -> ProviderLocalrun {
        let out =
            ProviderLocalrun(
                Rc::new(ProviderLocalrun_ { data: RefCell::new(ProviderLocalrunData { alias: None }) }),
            );
        stack.add_provider(out.0.clone());
        out
    }
}
