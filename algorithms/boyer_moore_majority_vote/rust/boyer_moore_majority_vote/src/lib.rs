fn get_majority_element(elements: &[i32]) -> Option<i32> {
    if elements.is_empty() {
        return None;
    }
    let mut candidate = elements[0];
    let mut count = 0;

    for element in elements {
        if count == 0 {
            candidate = *element;
        }

        if candidate == *element {
            count += 1;
        } else {
            count -= 1;
        }
    }

    Some(candidate)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_majority_element_test() {
        let result = get_majority_element(&[1, 1, 1, 1, 1, 1, 1, 2, 3, 4, 5, 1, 1, 1]);
        assert_eq!(1, result.unwrap());

        let result = get_majority_element(&[1]);
        assert_eq!(1, result.unwrap());

        let result = get_majority_element(&[]);
        assert!(result.is_none());
    }
}
