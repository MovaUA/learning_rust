use std::collections::HashMap;

pub fn get_mode(v: &[i32]) -> Option<i32> {
    let mut map = HashMap::new();

    for &number in v {
        *map.entry(number).or_insert(0) += 1;
    }

    map.into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
}

#[cfg(test)]
mod test {
    use crate::chapter_08::mode::get_mode;

    #[test]
    fn get_mode_non_empty_slice() {
        // arrange
        let input = vec![1, 2, 3, 2, 3, 3];

        // act
        let actual = get_mode(&input);

        // assert
        let expected = Some(3);
        assert_eq!(actual, expected);
    }

    #[test]
    fn get_mode_empty_slice() {
        // arrange
        let input = vec![];

        // act
        let actual = get_mode(&input);

        // assert
        let expected = None;
        assert_eq!(actual, expected);
    }
}
