#[derive(Debug)]
pub enum Language {
   English,
   Arabic,
   Chinese,
   French,
   German,
   Italian,
   Portuguese,
   Russain,
   Spanish,
}

pub struct Translate {
    input_text: String,
    input_lang: Language,
    output_text: String,
    output_lang: Language,
}

impl Translate {
    pub fn new(input_lang_type: Language, output_lang_type: Language, input: &str) -> Self {
        let input_lang: String = match input_lang_type {
            Language::English => "en".to_string(),
            Language::Arabic => "ar".to_string(),
            Language::Chinese => "zh".to_string(),
            Language::French => "fr".to_string(),
            Language::German => "de".to_string(),
            Language::Italian => "it".to_string(),
            Language::Portuguese => "pt".to_string(),
            Language::Russain => "rs".to_string(),
            Language::Spanish => "es".to_string(),
        };

        let output_lang: String = match output_lang_type {
            Language::English => "en".to_string(),
            Language::Arabic => "ar".to_string(),
            Language::Chinese => "zh".to_string(),
            Language::French => "fr".to_string(),
            Language::German => "de".to_string(),
            Language::Italian => "it".to_string(),
            Language::Portuguese => "pt".to_string(),
            Language::Russain => "rs".to_string(),
            Language::Spanish => "es".to_string(),
        };

        let input_text = String::from(input);

        let mut output_text: String = ureq::post("http://libretranslate.com/translate")
            .send_json(ureq::json!({
                "q": input_text,
                "source": input_lang,
                "target": output_lang,
            }))?
            .into_json()?;

        return Translate {
            input_text,
            input_lang: input_lang_type,
            output_text,
            output_lang: output_lang_type,
        };
    }
}
