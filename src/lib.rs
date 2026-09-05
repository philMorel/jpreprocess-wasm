use jpreprocess::{
    kind::JPreprocessDictionaryKind, normalize_text_for_naist_jdic, DefaultTokenizer,
    JPreprocess as CoreJPreprocess, SystemDictionaryConfig,
};
use jpreprocess_njd::NJDNode;
use js_sys::Error as JsError;
use serde::Serialize;
use wasm_bindgen::prelude::*;

type Engine = CoreJPreprocess<DefaultTokenizer>;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NjdNodeResult {
    pub surface: String,
    pub pos: String,
    pub pos_group1: String,
    pub pos_group2: String,
    pub pos_group3: String,
    pub ctype: String,
    pub cform: String,
    pub read: Option<String>,
    pub pronunciation: String,
    pub accent: usize,
    pub mora_size: usize,
    pub chain_rule: String,
    pub chain_flag: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct AnalysisResult {
    pub nodes: Vec<NjdNodeResult>,
}

#[derive(Debug, Serialize)]
pub struct InspectResult {
    pub before: Vec<NjdNodeResult>,
    pub after: Vec<NjdNodeResult>,
}

fn js_error(error: impl std::fmt::Display) -> JsValue {
    JsError::new(&error.to_string()).into()
}

fn pos_parts(node: &NJDNode) -> (String, String, String, String) {
    let pos = node.get_pos().to_string();
    let groups = pos.split(',').map(str::to_owned).collect::<Vec<_>>();
    (
        pos,
        groups.get(1).cloned().unwrap_or_else(|| "*".to_string()),
        groups.get(2).cloned().unwrap_or_else(|| "*".to_string()),
        groups.get(3).cloned().unwrap_or_else(|| "*".to_string()),
    )
}

fn serialize<T: Serialize>(value: &T) -> Result<JsValue, JsValue> {
    serde_wasm_bindgen::to_value(value).map_err(js_error)
}

fn node_result(node: &NJDNode) -> NjdNodeResult {
    let (pos, pos_group1, pos_group2, pos_group3) = pos_parts(node);
    NjdNodeResult {
        surface: node.get_string().to_string(),
        pos,
        pos_group1,
        pos_group2,
        pos_group3,
        ctype: node.get_ctype().to_string(),
        cform: node.get_cform().to_string(),
        read: node.get_read().map(str::to_owned),
        pronunciation: node.get_pron().to_pure_string(),
        accent: node.get_pron().accent(),
        mora_size: node.get_pron().mora_size(),
        chain_rule: node.get_chain_rule().to_string(),
        chain_flag: node.get_chain_flag(),
    }
}

fn node_results(nodes: &[NJDNode]) -> Vec<NjdNodeResult> {
    nodes.iter().map(node_result).collect()
}

#[wasm_bindgen]
pub struct JPreprocess {
    engine: Engine,
}

#[wasm_bindgen]
impl JPreprocess {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<JPreprocess, JsValue> {
        let system = SystemDictionaryConfig::Bundled(JPreprocessDictionaryKind::NaistJdic)
            .load()
            .map_err(js_error)?;
        Ok(Self {
            engine: CoreJPreprocess::with_dictionaries(system, None),
        })
    }

    #[wasm_bindgen]
    pub fn analyze(&self, text: &str, preprocess: Option<bool>) -> Result<JsValue, JsValue> {
        let mut njd = self.engine.text_to_njd(text).map_err(js_error)?;
        if preprocess.unwrap_or(false) {
            njd.preprocess();
        }
        serialize(&AnalysisResult {
            nodes: node_results(&njd.nodes),
        })
    }

    #[wasm_bindgen]
    pub fn inspect(&self, text: &str) -> Result<JsValue, JsValue> {
        let mut njd = self.engine.text_to_njd(text).map_err(js_error)?;
        let before = node_results(&njd.nodes);
        njd.preprocess();
        let after = node_results(&njd.nodes);
        serialize(&InspectResult { before, after })
    }

    #[wasm_bindgen]
    pub fn normalize(&self, text: &str) -> String {
        normalize_text_for_naist_jdic(text)
    }

    #[wasm_bindgen(js_name = "fullContext")]
    pub fn full_context(&self, text: &str) -> Result<JsValue, JsValue> {
        let labels = self.engine.extract_fullcontext(text).map_err(js_error)?;
        let labels = labels
            .into_iter()
            .map(|label| label.to_string())
            .collect::<Vec<_>>();
        serialize(&labels)
    }
}

#[cfg(test)]
mod tests {
    use super::{node_result, JPreprocess};

    #[test]
    fn exposes_read_and_pronunciation_before_and_after_preprocess() {
        let engine = JPreprocess::new().expect("bundled dictionary");
        let before = engine.engine.text_to_njd("例").expect("analysis");
        let before_node = node_result(&before.nodes[0]);
        assert_eq!(before_node.read.as_deref(), Some("レイ"));
        assert_eq!(before_node.pronunciation, "レー");

        let mut after = engine.engine.text_to_njd("1組").expect("analysis");
        after.preprocess();
        let after_node = node_result(&after.nodes[0]);
        assert_eq!(after_node.read.as_deref(), Some("イチ"));
        assert_eq!(after_node.pronunciation, "イッ");
    }
}
