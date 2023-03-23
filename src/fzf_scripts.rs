// Vec<(String, String)>をフィルタリングして、結果をVec<(String, String)>で返す関数
//

use std::{borrow::Cow, sync::Arc};

use skim::{
    prelude::{unbounded, SkimOptionsBuilder},
    ItemPreview, PreviewContext, Skim, SkimItem, SkimItemReceiver, SkimItemSender,
};

struct ScriptItems {
    inner: String,
    outer: String,
}

impl SkimItem for ScriptItems {
    fn text(&self) -> Cow<str> {
        Cow::Borrowed(&self.inner)
    }
    fn preview(&self, _context: PreviewContext) -> ItemPreview {
        if self.outer.len() > 0 {
            ItemPreview::Text(format!("{}", self.outer))
        } else {
            ItemPreview::Text(format!("{}", self.inner))
        }
    }
}

pub fn fzf_scipts(scripts: Vec<(String, String)>) -> Vec<String> {
    let options = SkimOptionsBuilder::default()
        .height(Some("30%"))
        .multi(true)
        .layout("reverse")
        .preview(Some("")) // preview should be specified to enable preview window
        .build()
        .unwrap();

    let (tx_item, rx_item): (SkimItemSender, SkimItemReceiver) = unbounded();

    for i in scripts {
        let _ = tx_item.send(Arc::new(ScriptItems {
            inner: i.0,
            outer: i.1,
        }));
    }

    drop(tx_item);
    let selected_items = Skim::run_with(&options, Some(rx_item))
        .map(|out| out.selected_items)
        .unwrap_or_else(Vec::new);

    let mut return_value: Vec<String> = Vec::new();
    print!("\x1B[?25h");
    for item in selected_items.iter() {
        return_value.push(Cow::Borrowed(&item.output()).to_string())
    }
    return return_value;
}
