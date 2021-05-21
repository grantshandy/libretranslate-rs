use crate::{TranslateError, Language};

/// A struct created by a [`Translate`](crate::traits::Translate) that can be translated using the translate method.
pub struct Query<'a> {
    pub url: &'a str,
    pub text: &'a str,
    pub source: Language,
    pub target: Language,
}

impl<'a> Query<'a> {
    pub fn to_lang(mut self, language: Language) -> Query<'a> {
        self.target = language;
        self
    }

    pub fn from_lang(mut self, language: Language) -> Query<'a> {
        self.source = language;
        self
    }

    pub fn url(mut self, url: &'a str) -> Query {
        self.url = url.as_ref();
        self
    }

    pub async fn translate(self) -> Result<String, TranslateError> {
        let res = crate::translate_url(self.source, self.target, self.text, self.url).await?;
        Ok(res.output)
    }
}

/// Translate text from a [`String`](std::string::String) or [`str`](std::str) (anything that implements [`AsRef<str>`](std::convert::AsRef)).
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
            url: "https://libretranslate.com/",
            text: self.as_ref(),
            source: Language::Detect,
            target: language,
        }
    }

    fn from_lang(&self, language: Language) -> Query {
        Query {
            url: "https://libretranslate.com/",
            text: self.as_ref(),
            source: language,
            target: Language::default(),
        }
    }
}
