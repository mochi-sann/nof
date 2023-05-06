// Vec<(String, String)>をフィルタリングして、結果をVec<(String, String)>で返す関数
//

use std::{borrow::Cow, sync::Arc};

use ansi_term::{Colour, Style};

use skim::{
    prelude::{unbounded, SkimOptionsBuilder},
    AnsiString, ItemPreview, PreviewContext, Skim, SkimItem, SkimItemReceiver, SkimItemSender,
};

use crate::debug;

struct ScriptItems {
    scripts: String,
    scripts_value: String,
}

impl SkimItem for ScriptItems {
    fn text(&self) -> Cow<str> {
        Cow::Borrowed(&self.scripts)
    }

    fn preview(&self, _context: PreviewContext) -> ItemPreview {
        if !self.scripts_value.is_empty() {
            ItemPreview::Text(self.scripts_value.to_string())
        } else {
            ItemPreview::Text(self.scripts.to_string())
        }
    }
    fn display<'a>(&'a self, _context: skim::DisplayContext<'a>) -> AnsiString<'a> {
        AnsiString::parse(&format!(
            "{} {} {}",
            Style::new().bold().paint(&self.scripts),
            Style::new().fg(Colour::Fixed(007_u8)).paint("-"),
            Style::new()
                .fg(Colour::Fixed(007_u8))
                .paint(&self.scripts_value)
        ))
    }
}

pub fn fzf_scipts(scripts: Vec<(String, String)>) -> Vec<String> {
    let options = SkimOptionsBuilder::default()
        .multi(true)
        .reverse(true)
        .build()
        .unwrap();

    let (tx_item, rx_item): (SkimItemSender, SkimItemReceiver) = unbounded();

    debug!(scripts.clone());
    for i in scripts {
        let _ = tx_item.send(Arc::new(ScriptItems {
            scripts: i.0,
            scripts_value: i.1,
        }));
    }

    drop(tx_item);
    let selected_items = Skim::run_with(&options, Some(rx_item))
        .map(|out| out.selected_items)
        .unwrap_or_else(Vec::new);

    let mut return_value: Vec<String> = Vec::new();
    for item in selected_items.iter() {
        return_value.push(Cow::Borrowed(&item.output()).to_string())
    }
    return_value
}
