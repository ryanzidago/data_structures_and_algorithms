use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct City {
    name: String,
    routes: HashMap<Rc<RefCell<City>>, u32>,
}

impl City {
    pub fn new(name: String) -> Self {
        City {
            name,
            routes: vec![],
        }
    }

    pub fn add_route(&mut self, city: City, price: u32) {
        self.routes.push()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let city = City::new("Paris".to_string());

        assert_eq!(
            city,
            City {
                name: "Paris".to_string(),
                routes: HashMap::new(),
            }
        );
    }
}
