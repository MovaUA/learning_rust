pub fn get_median(v: &[i32]) -> Option<f64> {
    match v.is_empty() {
        true => None,
        false => {
            let mut v = v.to_vec();
            v.sort();
            let length_is_odd = v.len() % 2 == 1;
            match length_is_odd {
                true => Some(v[v.len() / 2].into()),
                false => Some(f64::from(v[v.len() / 2 - 1] + v[v.len() / 2]) / 2.0),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::chapter_08::median::get_median;

    #[test]
    fn get_median_len_is_odd() {
        // arrange
        let input = vec![2, 1, 3];

        // act
        let actual = get_median(&input);

        // assert
        let expected = Some(2.0);
        assert_eq!(actual, expected);
    }

    #[test]
    fn get_median_len_is_even() {
        // arrange
        let input = vec![2, 1, 3, 4];

        // act
        let actual = get_median(&input);

        // assert
        let expected = Some(2.5);
        assert_eq!(actual, expected);
    }

    #[test]
    fn median_len_is_zero() {
        // arrange
        let input = vec![];

        // act
        let actual = get_median(&input);

        // assert
        let expected = None;
        assert_eq!(actual, expected);
    }
}
