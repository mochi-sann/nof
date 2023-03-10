use skim::prelude::*;

// Vec<(String, String)>をフィルタリングして、結果をVec<(String, String)>で返す関数
//

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

pub fn fzf_scipts(scripts: Vec<(String, String)>) -> (String, String) {
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

    print!("\x1B[?25h");
    return (
        selected_items[0].output().to_string(),
        selected_items[1].output().to_string(),
    );
}
