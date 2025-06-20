/// ライブラリファイルの
/// - コメント削除
/// - テスト削除
/// - 空行削除
/// - インデント挿入
/// を行う
pub fn simplify(source: String) -> String {
    let mut res = vec![];

    let mut is_in_comment = false;

    for line in source.lines() {
        let mut line = line.to_string();

        if is_in_comment {
            if line.find("*/").is_some() {
                is_in_comment = false;
            }
            line.clear();
        } else {
            if let Some(_) = line.find("/*") {
                is_in_comment = true;
                line.clear();
            }
            if let Some(pos) = line.find("//") {
                line = line[..pos].to_string();
            }
        }

        if line.find("#[test]").is_some() || line.find("#[cfg(test)]").is_some() {
            break;
        }

        if !line.trim().is_empty() {
            res.push(line);
        }
    }

    res.join("\n")
}
