use crate::languages::Language;
use crate::translate_url;
use crate::error::TranslateError;

/// Build Translations more verbosely.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct TranslationBuilder {
    pub url: String,
    pub source: Language,
    pub target: Language,
    pub input: String,
}

impl TranslationBuilder {
    pub fn new() -> Self {
        Self {
            url: String::from("https://libretranslate.com/"),
            source: Language::Detect,
            target: Language::default(),
            input: String::new(),
        }
    }

    pub fn url<T: AsRef<str>>(mut self, url: T) -> Self {
        self.url = url.as_ref().to_string();
        self
    }
    
    pub fn from_lang(mut self, lang: Language) -> Self {
        self.source = lang;
        self
    }

    pub fn to_lang(mut self, lang: Language) -> Self {
        self.target = lang;
        self
    }

    pub fn text<T: AsRef<str>>(mut self, text: T) -> Self {
        self.input = text.as_ref().to_string();
        self
    }

    pub async fn translate(mut self) -> Result<String, TranslateError> {
        if self.input == "" {
            return Ok(String::new());
        };

        let data = translate_url(self.source, self.target, self.input, self.url).await?;

        self.source = data.source;
        self.target = data.target;

        return Ok(data.output);
    }
}