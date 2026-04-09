// predictability, unlike garbage collecting
#![allow(unused)]
fn main() {
    type Document = Vec<String>; // type alias
    // we normally should use struct though

    // alternative to String::from()?
    let words = vec!["hello".to_string()];

    let d = new_document(words);

    // .to_vec() convers &[String] to Vec<String>
    // by cloning each string
    let words_copy = get_words(&d).to_vec();
    let mut d2 = new_document(words_copy);
    add_word(&mut d2, "world".to_string());

    // modifying 'd2' does not affect 'd', unlike Python
    assert!(!get_words(&d).contains(&"world".into()));

    fn new_document(words: Vec<String>) -> Document {
        // consumes ownership of the input vector "words"
        // Document OWNS the word vector
        // word vector will be predictably deallocated
        // when its owning Document goes out of scope
        words
    }

    fn add_word(this: &mut Document, word: String) {
        // requires mutable reference to mutate a doc
        // consumes ownership of the input "word"
        // meaning no one else can mutate the individual
        // words of the document
        this.push(word); // push with W permission
    }

    fn get_words(this: &Document) -> &[String] {
        // returns explicit immutable reference to
        // strings within the document
        this.as_slice()
    }
}
