//! Handling multi-line Strings

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Location {
    Char(usize),
    Line(usize),
    Offset(usize),
}

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

/// For reading multiline Strings
pub(crate) trait Multiline {
    /// Iterator over lines including the one containing location
    fn lines_from(&self, location: &Location) -> NumberedLines<impl Iterator<Item = &str>>;
    /// Get the line number of a given location
    fn line_no(&self, location: &Location) -> usize;
}
impl Multiline for &str {
    fn line_no(&self, location: &Location) -> usize {
        match location {
            Location::Char(pos) => {
                self.chars().take(*pos - 1).fold(1, |mut linecount, char| {
                    if char == '\n' {
                        linecount += 1;
                    }
                    linecount
                })
            }
            Location::Offset(pos) => {
                // Not safe to index to (*pos + 1) as this may not be a valid UTF-8 char boundary
                self[..*pos].lines().count() + 1
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

/// Iterator adaptor that keeps track of current line number
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
    /// Iterator over lines up to and *excluding* 'location'
    pub(crate) fn lines_to(
        self,
        location: &Location,
    ) -> NumberedLines<impl Iterator<Item = &'a str>> {
        let Location::Line(end_line) = location else {
            todo!("Cannot lines_to a position")
        };
        NumberedLines {
            lines: self.lines.take(end_line - self.line_number),
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
        let line4 = Location::Line(4);
        let text = text.as_str();
        let mut numbered_lines = text.lines_from(&line2).lines_to(&line4);
        assert_eq!(2, numbered_lines.line_number);
        assert_eq!("line 2", numbered_lines.next().unwrap());
        assert_eq!(3, numbered_lines.line_number);
        assert_eq!("line 3", numbered_lines.next().unwrap());
        assert!(numbered_lines.next().is_none());
    }
    #[test]
    fn line_no() {
        let mut text = String::new();
        text.push_line(0, ["line 1"]);
        text.push_line(0, ["line 2"]);
        text.push_line(0, ["line 3"]);
        let end_of_line_1 = Location::Char(7);
        assert_eq!(1, text.as_str().line_no(&end_of_line_1));
        let start_of_line_2 = Location::Char(8);
        assert_eq!(2, text.as_str().line_no(&start_of_line_2));
    }
}
