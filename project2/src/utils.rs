use serde::{Deserialize, Serialize};
use stop_words::LANGUAGE;
use tabled::{Style, Table, Tabled};
use whatlang::Lang;

#[derive(Debug, Deserialize, Serialize)]
pub struct Tweet {
    pub id: u64,
    pub text: String,
}

pub fn convert(lang: Lang) -> Option<LANGUAGE> {
    let language = match lang {
        Lang::Ara => LANGUAGE::Arabic,
        Lang::Dan => LANGUAGE::Danish,
        Lang::Nld => LANGUAGE::Dutch,
        Lang::Eng => LANGUAGE::English,
        Lang::Fin => LANGUAGE::Finnish,
        Lang::Fra => LANGUAGE::French,
        Lang::Deu => LANGUAGE::German,
        Lang::Ell => LANGUAGE::Greek,
        Lang::Hun => LANGUAGE::Hungarian,
        Lang::Ind => LANGUAGE::Indonesian,
        Lang::Ita => LANGUAGE::Italian,
        Lang::Por => LANGUAGE::Portuguese,
        Lang::Ron => LANGUAGE::Romanian,
        Lang::Rus => LANGUAGE::Russian,
        Lang::Slv => LANGUAGE::Slovenian,
        Lang::Spa => LANGUAGE::Spanish,
        Lang::Swe => LANGUAGE::Swedish,
        Lang::Tur => LANGUAGE::Turkish,
        Lang::Afr => LANGUAGE::Afrikaans,
        Lang::Ben => LANGUAGE::Bengali,
        Lang::Bul => LANGUAGE::Bulgarian,
        Lang::Ces => LANGUAGE::Czech,
        Lang::Cmn => LANGUAGE::Chinese,
        Lang::Est => LANGUAGE::Estonian,
        Lang::Pes => LANGUAGE::Persian,
        Lang::Heb => LANGUAGE::Hebrew,
        Lang::Hin => LANGUAGE::Hindi,
        Lang::Jpn => LANGUAGE::Japanese,
        Lang::Epo => LANGUAGE::Esperanto,
        Lang::Ukr => LANGUAGE::Ukrainian,
        Lang::Pol => LANGUAGE::Polish,
        Lang::Kor => LANGUAGE::Korean,
        Lang::Mar => LANGUAGE::Marathi,
        Lang::Hrv => LANGUAGE::Croatian,
        Lang::Lit => LANGUAGE::Lithuanian,
        Lang::Lav => LANGUAGE::Latvian,
        Lang::Vie => LANGUAGE::Vietnamese,
        Lang::Urd => LANGUAGE::Urdu,
        Lang::Tha => LANGUAGE::Thai,
        Lang::Guj => LANGUAGE::Gujarati,
        Lang::Zul => LANGUAGE::Zulu,
        Lang::Lat => LANGUAGE::Latin,
        Lang::Slk => LANGUAGE::Slovak,
        Lang::Cat => LANGUAGE::Catalan,
        Lang::Tgl => LANGUAGE::Tagalog,
        Lang::Hye => LANGUAGE::Armenian,
        _ => return None,
    };
    Some(language)
}

pub trait FindMore {
    /// Simpler way to find a `&str` word reference in a `Vec<String>`
    fn contains_from(&self, word: &str) -> bool;
}

pub trait AppendContents {
    fn append_contents(&mut self, contents: Vec<String>);
}

impl FindMore for Vec<&str> {
    fn contains_from(&self, word: &str) -> bool {
        for item in self {
            if item.to_lowercase() == word.to_lowercase() {
                return true;
            }
        }
        false
    }
}

impl FindMore for Vec<String> {
    /// Simpler way to find a `&str` word reference in a `Vec<String>`
    fn contains_from(&self, word: &str) -> bool {
        for item in self {
            if item.to_lowercase() == word.to_lowercase() {
                return true;
            }
        }
        false
    }
}

impl AppendContents for Vec<String> {
    /// A simple way to append before knowing about `&mut`
    fn append_contents(&mut self, mut contents: Vec<String>) {
        self.append(&mut contents);
    }
}

pub trait AsTable {
    fn print_table(&self);
}

impl<T: Tabled> AsTable for Vec<T>
where
    T: Tabled,
{
    fn print_table(&self) {
        println!("{}", Table::new(self).with(Style::modern()));
    }
}

pub trait EasyTake {
    type Output;
    fn prefix(self, count: usize) -> Vec<Self::Output>;
}

impl<T: Clone> EasyTake for Vec<T> {
    type Output = T;
    /// Get the first `count` items from the Vec
    fn prefix(self, count: usize) -> Vec<Self::Output> {
        self.into_iter().take(count).collect()
    }
}
