//! Handling multi-line Strings

use std::{iter::Skip, path::Iter, str::Lines};

/// For incrementally generating Strings.
///
/// - Initialise with `let mut str_buf = String::(new);`
/// - Reset (contents, not capacity) with `str_buf.clear();`
/// - Extend with `push_...` functions
pub(crate) trait MultilineMut {
    /// `indent` with spaces, concatenate `contents`, end with `\n`
    fn push_line<'strs>(&mut self, indent: usize, contents: impl IntoIterator<Item = &'strs str>);
    /// `indent` with 4 x n spaces, concatenate `contents`, end with `\n`
    fn push_python_line<'strs>(
        &mut self,
        indent: usize,
        contents: impl IntoIterator<Item = &'strs str>,
    );
    fn push_newline(&mut self);
}
pub(crate) trait Multiline {
    /// Iterator over lines including the one containing location
    fn lines_from(&self, location: &Location) -> NumberedLines<impl Iterator<Item = &str>>;
    /// Iterator over lines including the one containing location
    fn lines_to(&self, location: &Location) -> Lines;
    /// Get the line number of a given location
    fn line_no(&self, location: &Location) -> usize;
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Location {
    Position(usize),
    Line(usize),
}

impl MultilineMut for String {
    fn push_newline(&mut self) {
        self.push('\n');
    }

    /// `indent` with spaces, concatenate `contents`, end with `\n`
    fn push_line<'strs>(&mut self, indent: usize, contents: impl IntoIterator<Item = &'strs str>) {
        for _ in 0..indent {
            self.push(' ');
        }
        for text in contents {
            self.push_str(text);
        }
        self.push_newline();
    }

    /// `indent` with 4 x n spaces, concatenate `contents`, end with `\n`
    fn push_python_line<'strs>(
        &mut self,
        indent: usize,
        contents: impl IntoIterator<Item = &'strs str>,
    ) {
        self.push_line(4 * indent, contents);
    }
}

impl Multiline for &str {
    fn line_no(&self, location: &Location) -> usize {
        match location {
            Location::Position(pos) => self[0..*pos].lines().count(),
            Location::Line(line) => *line,
        }
    }
    fn lines_from(&self, location: &Location) -> NumberedLines<impl Iterator<Item = &str>> {
        let lineno = self.line_no(location);
        NumberedLines {
            lines: self.lines().skip(lineno - 1),
            line_number: lineno,
        }
    }
    fn lines_to(&self, location: &Location) -> Lines {
        todo!()
    }
}

pub(crate) struct NumberedLines<LineIterator> {
    lines: LineIterator,
    line_number: usize,
}

impl<'a, LineIterator> Iterator for NumberedLines<LineIterator> 
where LineIterator: Iterator<Item = &'a str> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        self.lines.next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lines_from() {
        let mut text = String::new();
        text.push_line(0, ["line 1"]);
        text.push_line(0, ["line 2"]);
        text.push_line(0, ["line 3"]);
        let line2 = Location::Line(2);
        let text = text.as_str();
        let mut numbered_lines = text.lines_from(&line2);
        assert_eq!("line 2", numbered_lines.next().unwrap())
    }
}
