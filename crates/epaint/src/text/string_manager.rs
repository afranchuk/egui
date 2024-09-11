use super::StringId;

pub trait StringManager {
    type CharIterator<'a>: Iterator<Item = char> + 'a
        where Self: 'a;

    fn chars<'a>(&'a self, id: StringId) -> Self::CharIterator<'a>;
}

#[derive(Default)]
pub struct OwningStringManager {
    strings: Vec<String>,
}

impl OwningStringManager {
    pub fn string(&mut self, v: &(impl ToString + ?Sized)) -> StringId {
        let id = self.strings.len();
        self.strings.push(v.to_string());
        id
    }

    pub fn clear(&mut self) {
        self.strings.clear();
    }
}

impl StringManager for OwningStringManager {
    type CharIterator<'a> = std::str::Chars<'a>;

    fn chars<'a>(&'a self, id: StringId) -> Self::CharIterator<'a> {
        self.strings[id].chars()
    }
}
