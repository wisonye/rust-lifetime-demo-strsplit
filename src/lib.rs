#![allow(warnings)]
use std::iter::*;

// The `'a` lifetime is saying that the actually string content is valid for that long,
// as this struct got the pointer to it!!!
struct StrSplit<'a> {
    reminder: &'a str,
    delimiter: &'a str,
}

impl<'a> StrSplit<'a> {
    pub fn new(content: &'a str, delimiter: &'a str) -> Self {
        StrSplit {
            reminder: content,
            delimiter,
        }
    }
}

impl<'a> Iterator for StrSplit<'a> {
    type Item = &'a str;

    /// Be able to keep calling `next` to return `Some` until it returns `None`
    fn next(&mut self) -> Option<Self::Item> {
        // Find the next delimiter
        if let Some(next_delimiter) = self.reminder.find(self.delimiter) {
            let before_next_delimiter = &self.reminder[..next_delimiter];
            self.reminder = &self.reminder[next_delimiter + 1..];

            println!(
                "before_next_delimiter {}, reminder: <{}>",
                before_next_delimiter, self.reminder
            );
            Some(before_next_delimiter)
        } else if self.reminder.is_empty() {
            None
        } else {
            let rest = self.reminder;

            // What happen here is that we assign a `&'static` str (empty string) to
            // `self.reminder`. That's ok, as the `'static` lifetime means the value
            // will be valid for the entire program lifetime!!!
            self.reminder = "";
            Some(rest)
        }
    }
}

// The `'a` lifetime is saying that the actually string content is valid for that long,
// as this struct got the pointer to it!!!
struct StrSplit2<'a> {
    reminder: Option<&'a str>,
    delimiter: Option<&'a str>,
}

impl<'a> StrSplit2<'a> {
    pub fn new(content: Option<&'a str>, delimiter: Option<&'a str>) -> Self {
        StrSplit2 {
            reminder: content,
            delimiter,
        }
    }
}

impl<'a> Iterator for StrSplit2<'a> {
    type Item = &'a str;

    /// Be able to keep calling `next` to return `Some` until it returns `None`
    fn next(&mut self) -> Option<Self::Item> {
        if self.reminder.is_none() {
            return None;
        } else if self.delimiter.is_none() {
            return Some(self.reminder.unwrap());
        }

        // As we need to use the `value` inside the `option` but we don't want to
        // move that value out!!! We just need a mutable reference to it then
        // we can do something on that mutable reference.
        //
        // After we call `self.reminder::as_mut().unwrap`, we will get back the
        // value with type of `&mut &'a str`: A mutable reference to the option
        // value which is `&'a str`!!!
        let reminder: &mut &'a str = self.reminder.as_mut().unwrap();
        let delimiter = self.delimiter.as_ref().unwrap();

        if reminder.is_empty() || delimiter.is_empty() {
            return None;
        }

        if let Some(next_delimiter) = reminder.find(delimiter) {
            let before_next_delimiter = &reminder[..next_delimiter];

            // As `reminder` type is `&mut &'a str`, and we want to re-assign
            // a value to it. Then we have to dereference it first before we
            // assign to it. That's why the `*` comes in :)
            *reminder = &reminder[next_delimiter + 1..];

            println!(
                "StrSplit2 -> before_next_delimiter {}, reminder: <{:?}>",
                before_next_delimiter, self.reminder
            );

            Some(before_next_delimiter)
        } else {
            // If we can't find the delimiter, then just return the
            // rest part of reminder and leave the `None` into `self.reminder`, 
            // that's what `Option::take()` does.
            self.reminder.take()
        }

        // match (self.reminder, self.delimiter) {
        // (Some(ref mut writable_reminder), Some(ref delimiter)) => {}
        // _ => None,
        // }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn iterator_works() {
        let test_content = "a b c d e f";
        let mut letters = StrSplit2::new(Some(&test_content), Some(" "));

        // assert_eq!(letters.next(), Some("a"));
        // assert_eq!(letters.next(), Some("b"));
        // assert_eq!(letters.next(), Some("c"));
        // assert_eq!(letters.next(), Some("d"));
        // assert_eq!(letters.next(), Some("e"));
        // assert_eq!(letters.next(), Some("f"));
        // assert_eq!(letters.next(), None);

        assert!(letters.eq(vec!["a", "b", "c", "d", "e", "f"].into_iter()));

        // `For` loop just a syntax sugar for the `Iterator`
        // for letter in letters {
        // println!("letter {}", letter);
        // }
    }

    #[test]
    fn empty_value_is_ok() {
        let test_content = "";
        let mut letters = StrSplit2::new(Some(&test_content), Some(" "));

        assert_eq!(letters.next(), None);
    }

    #[test]
    fn empty_content_is_ok() {
        let test_content = "";
        let mut letters = StrSplit2::new(None, Some(" "));

        assert_eq!(letters.next(), None);
    }

    #[test]
    fn empty_delimiter_is_ok() {
        let test_content = "a b c";
        let mut letters = StrSplit2::new(Some(&test_content), None);

        assert_eq!(letters.next(), Some("a b c"));
    }

    #[test]
    fn end_with_delimiter() {
        let test_content = "a b c ";
        let mut letters = StrSplit2::new(Some(&test_content), Some(" "));

        assert_eq!(letters.next(), Some("a"));
        assert_eq!(letters.next(), Some("b"));
        assert_eq!(letters.next(), Some("c"));
        assert_eq!(letters.next(), None);
    }
}
