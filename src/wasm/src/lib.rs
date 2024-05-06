use wasm_bindgen::prelude::*;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export function find(heystack: string, needle: string): string[];
"#;
#[wasm_bindgen(skip_typescript)]
pub fn find(heystack: &str, needle: &str) -> Box<[JsValue]> {
    // 検索対象のコンテンツを改行単位で整形する
    let splited_heystack: Vec<&str> = heystack.split("\n").collect();
    let non_empty_lines: Vec<&str> = splited_heystack.into_iter().filter(|&line| !line.is_empty()).collect();

    let threshold = 10;
    // 検索文字列をもとにあいまい検索を行う
    let mut scored_texts: Vec<ScoredText> = non_empty_lines.into_iter().map(|line| fuzzy_search(line, needle)).collect();
    // 閾値未満のスコアを持つ行を除外する
    scored_texts.retain(|scored_text| scored_text.score > threshold);
    // スコアの高い順にソートする
    scored_texts.sort_by(|a, b| b.score.cmp(&a.score));

    let result: Vec<JsValue> = scored_texts.into_iter().map(|scored_text| JsValue::from_str(&scored_text.text)).collect();
    result.into_boxed_slice()
}

struct ScoredText {
    score: i64,
    text: String,
}

fn fuzzy_search(heystack: &str, needle: &str) -> ScoredText {
    let matcher = SkimMatcherV2::default();
    let result = matcher.fuzzy_match(heystack, needle);

    if let Some(score) = result {
        return ScoredText {
            score,
            text: heystack.to_string(),
        };
    } else {
        return ScoredText {
            score: 0,
            text: heystack.to_string(),
        };
    }
}
