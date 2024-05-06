use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export function find(heystack: string, needle: string): string[];
"#;
#[wasm_bindgen(skip_typescript)]
pub fn find(heystack: &str, needle: &str) -> Box<[JsValue]> {
    // 検索対象のコンテンツを改行単位で整形する
    let splited_heystack: Vec<&str> = heystack.split("\n").collect();
    let non_empty_lines: Vec<&str> = splited_heystack.into_iter().filter(|&line| !line.is_empty()).collect();

    let threshold = 0.5;
    // 検索文字列をもとにあいまい検索を行う
    // スコアが閾値以上の場合は、検索結果に含める
    // スコア順にソートする
    let result = vec!["検索結果1", "検索結果2", "検索結果3"];

    format!("{} {}", heystack, needle);
    result.into_iter().map(|line| JsValue::from_str(line)).collect::<Vec<JsValue>>().into_boxed_slice()
}

struct ScoredText {
    score: f64,
    text: String,
}