use std::collections::HashSet;

use biome_js_analyze::utils::rename::RenameSymbolExtensions;
use biome_js_parser::{parse, Parse};
use biome_js_semantic::{semantic_model, SemanticModelOptions};
use biome_js_syntax::{JsFileSource, JsIdentifierBinding, JsImportNamedClause, AnyJsRoot};
use biome_rowan::{AstNode, BatchMutationExt};
fn try_new_name(old_name: &str, global_names: &mut HashSet<String>) -> String {
    let mut i = 0;
    loop {
        let new_name = format!("{}${}", old_name, i);
        if global_names.contains(&new_name) {
            i += 1;
            continue;
        } else {
            global_names.insert(new_name.clone());
            return new_name;
        }
    }
}
pub fn rename_ast(r: AnyJsRoot) -> AnyJsRoot{
    let model = semantic_model(&r, SemanticModelOptions::default());
    let bindings: Vec<JsIdentifierBinding> = r
        .syntax()
        .descendants()
        .filter_map(JsIdentifierBinding::cast)
        .filter(|x| {
            let t = x.syntax().ancestors().find_map(JsImportNamedClause::cast);
            t.is_none()
        })
        .collect();
    let mut batch = r.begin();

    for binding in bindings {
        if let Ok(token) = binding.name_token() {
            let old_name = token.text_trimmed();
            let new_name = format!("{old_name}${old_name}");
            // let new_name = try_new_name(old_name, global_names);
            batch.rename_node_declaration(&model, binding, &new_name);
        }
    }
    let root = batch.commit();
    return AnyJsRoot::cast(root).unwrap();
}
pub fn rename_top_level(source: &str, global_names: &mut HashSet<String>) -> String {
    let root = parse(source, JsFileSource::js_module(), biome_js_parser::JsParserOptions::default()).tree();
    let r1 = rename_ast(root);
    let r2 = rename_ast(r1);
    dbg!(r2.to_string())
}
