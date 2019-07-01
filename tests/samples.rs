use insta::assert_debug_snapshot_matches;
use rust_dbg_enhancer::enhance;

#[test]
fn pest_pairs() {
    assert_debug_snapshot_matches!(
        "struct",
        enhance(r#"Pair { rule: xkb_types_item }"#).unwrap()
    );

    assert_debug_snapshot_matches!(
        "array",
        enhance(r#"[Pair { rule: xkb_types_item }]"#).unwrap()
    );

    assert_debug_snapshot_matches!(
        "full",
        enhance(r#"[Pair { rule: xkb_types_item, span: Span { str: "override type \"TWO_LEVEL\" {\n    modifiers = Shift+Lock;\n    map[Shift] = Level2;\n    preserve[Lock]= Lock;\n    level_name[Level1] = \"Base\";\n    level_name[Level2] = \"Shift\";\n};", start: 50, end: 226 }, inner: [Pair { rule: type_item, span: Span { str: "override type \"TWO_LEVEL\" {\n    modifiers = Shift+Lock;\n    map[Shift] = Level2;\n    preserve[Lock]= Lock;\n    level_name[Level1] = \"Base\";\n    level_name[Level2] = \"Shift\";\n};", start: 50, end: 226 }, inner: [Pair { rule: type_item_prefix, span: Span { str: "override", start: 50, end: 58 }, inner: [] }, Pair { rule: string, span: Span { str: "\"TWO_LEVEL\"", start: 64, end: 75 }, inner: [Pair { rule: string_content, span: Span { str: "TWO_LEVEL", start: 65, end: 74 }, inner: [] }] }, Pair { rule: type_component, span: Span { str: "modifiers = Shift+Lock;", start: 82, end: 105 }, inner: [Pair { rule: modifiers, span: Span { str: "modifiers = Shift+Lock;", start: 82, end: 105 }, inner: [Pair { rule: key_combo, span: Span { str: "Shift+Lock", start: 94, end: 104 }, inner: [Pair { rule: ident, span: Span { str: "Shift", start: 94, end: 99 }, inner: [] }, Pair { rule: ident, span: Span { str: "Lock", start: 100, end: 104 }, inner: [] }] }] }] }, Pair { rule: type_component, span: Span { str: "map[Shift] = Level2;", start: 110, end: 130 }, inner: [Pair { rule: map, span: Span { str: "map[Shift] = Level2;", start: 110, end: 130 }, inner: [Pair { rule: key_combo, span: Span { str: "Shift", start: 114, end: 119 }, inner: [Pair { rule: ident, span: Span { str: "Shift", start: 114, end: 119 }, inner: [] }] }, Pair { rule: ident, span: Span { str: "Level2", start: 123, end: 129 }, inner: [] }] }] }, Pair { rule: type_component, span: Span { str: "preserve[Lock]= Lock;", start: 135, end: 156 }, inner: [Pair { rule: preserve, span: Span { str: "preserve[Lock]= Lock;", start: 135, end: 156 }, inner: [Pair { rule: key_combo, span: Span { str: "Lock", start: 144, end: 148 }, inner: [Pair { rule: ident, span: Span { str: "Lock", start: 144, end: 148 }, inner: [] }] }, Pair { rule: key_combo, span: Span { str: "Lock", start: 151, end: 155 }, inner: [Pair { rule: ident, span: Span { str: "Lock", start: 151, end: 155 }, inner: [] }] }] }] }, Pair { rule: type_component, span: Span { str: "level_name[Level1] = \"Base\";", start: 161, end: 189 }, inner: [Pair { rule: level_name, span: Span { str: "level_name[Level1] = \"Base\";", start: 161, end: 189 }, inner: [Pair { rule: group, span: Span { str: "[Level1]", start: 171, end: 179 }, inner: [Pair { rule: ident, span: Span { str: "Level1", start: 172, end: 178 }, inner: [] }] }, Pair { rule: string, span: Span { str: "\"Base\"", start: 182, end: 188 }, inner: [Pair { rule: string_content, span: Span { str: "Base", start: 183, end: 187 }, inner: [] }] }] }] }, Pair { rule: type_component, span: Span { str: "level_name[Level2] = \"Shift\";", start: 194, end: 223 }, inner: [Pair { rule: level_name, span: Span { str: "level_name[Level2] = \"Shift\";", start: 194, end: 223 }, inner: [Pair { rule: group, span: Span { str: "[Level2]", start: 204, end: 212 }, inner: [Pair { rule: ident, span: Span { str: "Level2", start: 205, end: 211 }, inner: [] }] }, Pair { rule: string, span: Span { str: "\"Shift\"", start: 215, end: 222 }, inner: [Pair { rule: string_content, span: Span { str: "Shift", start: 216, end: 221 }, inner: [] }] }] }] }] }] }]"#).unwrap()
    );
}
