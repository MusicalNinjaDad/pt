//! Handling multi-line Strings

use std::str::Lines;

/// For incrementally generating Strings.
///
/// - Initialise with `let mut str_buf = String::(new);`
/// - Reset (contents, not capacity) with `str_buf.clear();`
/// - Extend with `push_...` functions
#[allow(unused_variables)]
pub(crate) trait Multiline {
    /// `indent` with spaces, concatenate `contents`, end with `\n`
    fn push_line<'strs>(&mut self, indent: usize, contents: impl IntoIterator<Item = &'strs str>) {
        unimplemented!()
    }
    /// `indent` with 4 x n spaces, concatenate `contents`, end with `\n`
    fn push_python_line<'strs>(
        &mut self,
        indent: usize,
        contents: impl IntoIterator<Item = &'strs str>,
    ) {
        unimplemented!()
    }
    fn push_newline(&mut self) {
        unimplemented!()
    }
    // Iterator over lines including the one containing location
    fn lines_from(&self, location: &Location) -> Lines {
        unimplemented!()
    }
    /// Iterator over lines including the one containing location
    fn lines_to(&self, location: &Location) -> Lines {
        unimplemented!()
    }
    /// Get the line number of a given location
    fn line_no(&self, location: &Location) -> usize {
        unimplemented!()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Location {
    Position(usize),
    Line(usize),
}

impl Multiline for String {
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
}
