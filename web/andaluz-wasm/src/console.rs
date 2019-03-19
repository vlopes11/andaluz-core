use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::document;
use stdweb::web::html_element::TextAreaElement;

#[derive(Debug, Clone)]
pub struct Console {
    selector: TextAreaElement,
    value: String,
}

impl Console {
    pub fn new() -> Self {
        let selector: TextAreaElement = document()
            .query_selector("#andaluz-console")
            .unwrap()
            .unwrap()
            .try_into()
            .unwrap();

        Console {
            selector,
            value: String::from(""),
        }
    }

    pub fn println(&mut self, txt: String) {
        self.value = format!("{}\n{}", txt, self.value);
        self.selector.set_value(self.value.as_str());
    }

    pub fn clear(&mut self) {
        self.value.clear();
        self.selector.set_value(self.value.as_str());
    }
}
