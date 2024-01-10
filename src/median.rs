use std::collections::HashMap;

pub fn median(v: &[i32]) -> Option<f64> {
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

pub fn mode(v: &[i32]) -> Option<i32> {
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
    use crate::median;

    #[test]
    fn median_len_is_odd() {
        // arrange
        let input = vec![2, 1, 3];

        // act
        let actual = median::median(&input);

        // assert
        let expected = Some(2.0);
        assert_eq!(actual, expected);
    }

    #[test]
    fn median_len_is_even() {
        // arrange
        let input = vec![2, 1, 3, 4];

        // act
        let actual = median::median(&input);

        // assert
        let expected = Some(2.5);
        assert_eq!(actual, expected);
    }

    #[test]
    fn median_len_is_zero() {
        // arrange
        let input = vec![];

        // act
        let actual = median::median(&input);

        // assert
        let expected = None;
        assert_eq!(actual, expected);
    }


    #[test]
    fn mode() {
        // arrange
        let input = vec![1, 2, 3, 2, 3, 3];

        // act
        let actual = median::mode(&input);

        // assert
        let expected = Some(3);
        assert_eq!(actual, expected);
    }

    #[test]
    fn mode_len_is_empty() {
        // arrange
        let input = vec![];

        // act
        let actual = median::mode(&input);

        // assert
        let expected = None;
        assert_eq!(actual, expected);
    }
}
