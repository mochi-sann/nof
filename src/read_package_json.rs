use serde_json::Value;
use std::fs;

// パッケージのpackage.jsonからscriptsを取得する関数
pub fn get_scripts(file_path: String) -> Vec<(String, String)> {
    // パッケージのpackage.jsonを取得
    let data = get_package_json(file_path);
    // scriptsオブジェクトを取得
    let scripts = data["scripts"].as_object().unwrap();

    // scriptsオブジェクトのキーと値を含むタプルのベクターを作成
    let script_names: Vec<(String, String)> = scripts
        .iter()
        .map(|(k, v)| (k.to_string(), v.as_str().unwrap().to_string()))
        .collect();

    // script_namesベクターを返す
    script_names
}

// パッケージのpackage.jsonをパースする関数
fn get_package_json(file_path: String) -> Value {
    // ファイルの内容を文字列として取得
    let contents = fs::read_to_string(&file_path).expect("Something went wrong reading the file");

    // パースしたJSONを返す
    serde_json::from_str(&contents).unwrap()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_scripts() {
        // テスト用のpackage.jsonファイルのパス
        let file_path = String::from("./test_files/aaa.json");

        // テスト用のscripts
        let expected_scripts = vec![
            (String::from("build"), String::from("webpack --config webpack.config.js")),
            (String::from("start"), String::from("node server.js")),
            (String::from("test"), String::from("echo \"Error: no test specified\" && exit 1")),
        ];

        // get_scripts関数で取得したscriptsの一覧
        let actual_scripts = get_scripts(file_path);

        // 期待値と実際の値が一致するかを検証
        assert_eq!(expected_scripts, actual_scripts);
    }
    #[test]
    fn test_get_scripts_2() {
        // テスト用のpackage.jsonファイルのパス
        let file_path = String::from("./test_files/bbb.json");

        // テスト用のscripts
        let expected_scripts = vec![
            (String::from("start"), String::from("node server.js")),
            (String::from("start2"), String::from("node server.js")),
            (String::from("start3"), String::from("node server.js")),
        ];

        // get_scripts関数で取得したscriptsの一覧
        let actual_scripts = get_scripts(file_path);

        // 期待値と実際の値が一致するかを検証
        assert_eq!(expected_scripts, actual_scripts);
    }
}
