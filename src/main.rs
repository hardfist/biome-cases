use std::collections::HashSet;

use rename::rename_top_level;
pub mod rename;

fn main() {
    let code1 = r#"
      let a = 10;
      let b = 20;
    "#;
    let code2 = r#"
      let b = 30;
      let c = 40;
    "#;
    let code3 = r#"
      let a = 1;
      let b = 2;
      let c = 3;
      let d = 4;
    "#;
    let code_list = [code1,code2,code3];
    let mut new_code_list = vec![];
    let mut name_cache: HashSet<String> = HashSet::new();
    for code in code_list {
        let new_code = rename_top_level(code, &mut name_cache);
        new_code_list.push(new_code);
    }
    dbg!(new_code_list);
    

}
