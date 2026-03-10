//! Handling multi-line Strings

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
            Location::Position(pos) => {
                let mut chars = self.chars();
                for _ in 0..*pos {
                    chars.next();
                }
                chars.as_str().lines().count()
            }
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
}

pub(crate) struct NumberedLines<LineIterator> {
    lines: LineIterator,
    line_number: usize,
}

impl<'a, LineIterator> Iterator for NumberedLines<LineIterator>
where
    LineIterator: Iterator<Item = &'a str>,
{
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        self.line_number += 1;
        self.lines.next()
    }
}

impl<'a, LineIterator> NumberedLines<LineIterator>
where
    LineIterator: Iterator<Item = &'a str>,
{
    pub(crate) fn lines_to(self, location: &Location) -> NumberedLines<impl Iterator<Item = &'a str>> {
        let Location::Line(end_line) = location else {
            todo!("Cannot lines_to a position")
        };
        NumberedLines {
            lines: self.lines.take(end_line - self.line_number + 1),
            line_number: self.line_number,
        }
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
        assert_eq!(2, numbered_lines.line_number);
        assert_eq!("line 2", numbered_lines.next().unwrap());
        assert_eq!(3, numbered_lines.line_number);
    }

    #[test]
    fn lines_to() {
        let mut text = String::new();
        text.push_line(0, ["line 1"]);
        text.push_line(0, ["line 2"]);
        text.push_line(0, ["line 3"]);
        text.push_line(0, ["line 4"]);
        let line2 = Location::Line(2);
        let line3 = Location::Line(3);
        let text = text.as_str();
        let mut numbered_lines = text.lines_from(&line2).lines_to(&line3);
        assert_eq!(2, numbered_lines.line_number);
        assert_eq!("line 2", numbered_lines.next().unwrap());
        assert_eq!(3, numbered_lines.line_number);
        assert_eq!("line 3", numbered_lines.next().unwrap());
        assert!(numbered_lines.next().is_none());
    }
}
