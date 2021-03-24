use crate::{translate, Language, TranslateError};

pub struct Query<'a> {
    pub text: &'a str,
    pub source: Option<Language>,
    pub target: Language,
}

impl<'a> Query<'a> {
    pub fn to_lang(mut self, language: Language) -> Query<'a> {
        self.target = language;
        self
    }

    pub fn from_lang(mut self, language: Language) -> Query<'a> {
        self.source = Some(language);
        self
    }

    pub fn translate(self) -> Result<String, TranslateError> {
        let res = translate(self.source, self.target, self.text)?;
        Ok(res.output)
    }
}

pub trait Translate {
    fn to_lang(&self, language: Language) -> Query;
    fn from_lang(&self, language: Language) -> Query;
}

impl<T> Translate for T
where
    T: AsRef<str>,
{
    fn to_lang(&self, language: Language) -> Query {
        Query {
            text: self.as_ref(),
            source: None,
            target: language,
        }
    }

    fn from_lang(&self, language: Language) -> Query {
        Query {
            text: self.as_ref(),
            source: Some(language),
            target: Language::default(),
        }
    }
}
