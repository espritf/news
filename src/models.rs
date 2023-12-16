use chrono::NaiveDateTime;

pub struct Channel {
    title: String,
    link: String,
    language: String,
    last_build_date: NaiveDateTime,
    items: Vec<Item>,
}

impl Channel {
    pub fn new(title: &str, link: &str, language: &str, last_build_date: NaiveDateTime, items: Vec<Item>) -> Self {
        Self { title: title.to_owned(), link: link.to_owned(), language: language.to_owned(), last_build_date, items }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn link(&self) -> &str {
        &self.link
    }

    pub fn language(&self) -> &str {
        &self.language
    }

    pub fn last_build_date(&self) -> NaiveDateTime {
        self.last_build_date
    }

    pub fn items(&self) -> &[Item] {
        &self.items
    }
}

pub struct Item {
    guid: String,
    title: String,
    link: String,
    description: String,
    pub_date: NaiveDateTime,
}

impl Item {
    pub fn new(guid: &str, title: &str, link: &str, description: &str, pub_date: NaiveDateTime) -> Self {
        Self {
            guid: guid.to_owned(),
            title: title.to_owned(),
            link: link.to_owned(),
            description: description.to_owned(),
            pub_date,
        }
    }

    pub fn guid(&self) -> &str {
        &self.guid
    }
    
    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn link(&self) -> &str {
        &self.link
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn pub_date(&self) -> NaiveDateTime {
        self.pub_date
    }
}
