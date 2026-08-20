/// Very niche trait to access the row and column value of a `Vec<String>`.
/// This _should_ save a lot of time in accessing coordinates that may or
/// may not be in bounds.
pub trait AocVecStringAccess {
    /// Get the character at a `row` and `col` value which are signed.
    /// # Examples
    /// ```rust
    /// use aoc_2025::solution::utils::AocVecStringAccess;
    ///
    /// let lines = vec![String::from("ABCD"), String::from("1234")];
    /// assert_eq!(Some('C'), lines.get_signed(0, 2));
    /// assert_eq!(Some('4'), lines.get_signed(1, 3));
    /// assert_eq!(None, lines.get_signed(0, 10));
    /// assert_eq!(None, lines.get_signed(-1, 0));
    /// ```
    fn get_signed(&self, row: isize, col: isize) -> Option<char>;

    /// Iterate over the rows and columns getting each entry.
    /// # Examples
    /// ```rust
    /// use aoc_2025::solution::utils::AocVecStringAccess;
    ///
    /// let lines = vec![String::from("12"), String::from("AB")];
    /// let mut iter = lines.row_col_iter();
    ///
    /// assert_eq!(iter.next(), Some((0, 0, '1')));
    /// assert_eq!(iter.next(), Some((0, 1, '2')));
    /// assert_eq!(iter.next(), Some((1, 0, 'A')));
    /// assert_eq!(iter.next(), Some((1, 1, 'B')));
    /// assert_eq!(iter.next(), None);
    /// ```
    fn row_col_iter(&self) -> impl Iterator<Item = (usize, usize, char)>;

    /// Get the max for row and col for a list of `String`
    /// Helpful for grids.
    /// # Examples
    /// ```rust
    /// use aoc_2025::solution::utils::AocVecStringAccess;
    ///
    /// let grid = vec![String::from("123"), String::from("ABC")];
    /// let not_grid = vec![String::from("123"), String::from("ABCD")];
    ///
    /// assert_eq!(grid.max_row_col(), Some((2, 3)));
    /// assert_eq!(not_grid.max_row_col(), Some((2, 4)));
    /// assert_eq!(vec![].max_row_col(), None);
    ///```
    fn max_row_col(&self) -> Option<(usize, usize)>;
}

impl AocVecStringAccess for Vec<String> {
    fn get_signed(&self, row: isize, col: isize) -> Option<char> {
        Some(
            *self
                .as_slice()
                .get_signed(row)?
                .as_bytes()
                .get_signed(col)? as char,
        )
    }

    fn row_col_iter(&self) -> impl Iterator<Item = (usize, usize, char)> {
        self.iter()
            .enumerate()
            .map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(col, ch)| (row, col, ch))
            })
            .flatten()
    }

    fn max_row_col(&self) -> Option<(usize, usize)> {
        Some((self.len(), self.iter().map(String::len).max()?))
    }
}

pub fn gcd<T: std::ops::Rem<Output = T> + std::cmp::PartialEq<i32> + Copy>(a: T, b: T) -> T {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

trait SignedAccess<T> {
    fn get_signed(&self, idx: isize) -> Option<&T>;
}

impl<T> SignedAccess<T> for [T] {
    fn get_signed(&self, idx: isize) -> Option<&T> {
        if idx < 0 {
            None
        } else {
            self.get(idx.unsigned_abs() as usize)
        }
    }
}
