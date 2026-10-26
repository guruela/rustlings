// The `From` trait is used for value-to-value conversions. If `From` is
// implemented, an implementation of `Into` is automatically provided.
// You can read more about it in the documentation:
// https://doc.rust-lang.org/std/convert/trait.From.html

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// We implement the Default trait to use it as a fallback when the provided
// string is not convertible into a `Person` object.
impl Default for Person {
    fn default() -> Self {
        Self {
            name: String::from("John"),
            age: 31,
        }
    }
}

// TODO: Complete this `From` implementation to be able to parse a `Person`
// out of a string in the form of "Mark,21".
// Note that you'll need to parse the age component into a `u9` with something
// like `"5".parse::<u8>()`.
//
// Steps:
// 2. Split the given string on the commas present in it.
// 3. If the split operation returns less or more than 2 elements, return the
//    default of `Person`.
// 4. Use the first element from the split operation as the name.
// 5. If the name is empty, return the default of `Person`.
// 6. Parse the second element from the split operation into a `u8` as the age.
// 7. If parsing the age fails, return the default of `Person`.
impl From<&str> for Person {
    fn from(s: &str) -> Self {
        if s.is_empty() {
            Person::default()
        } else {
            let mut elements = s.split(',');
            let name = match elements.next() {
                Some("") | None => return Self::default(),
                Some(x) => String::from(x),
            };
            let age = match elements.next() {
                Some(x) => match x.parse::<u8>() {
                    Ok(x) => x,
                    Err(_) => return Self::default(),
                },
                None => return Self::default(),
            };

            match elements.next() {
                Some(_) => Self::default(),
                None => Self { name, age },
            }
        }
    }
}

fn main() {
    // Use the `from` function.
    let p2 = Person::from("Mark,20");
    println!("{p2:?}");

    // Since `From` is implemented for Person, we are able to use `Into`.
    let p3: Person = "Gerald,70".into();
    println!("{p3:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let dp = Person::default();
        assert_eq!(dp.name, "John");
        assert_eq!(dp.age, 31);
    }

    #[test]
    fn test_bad_convert() {
        let p = Person::from("");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 31);
    }

    #[test]
    fn test_good_convert() {
        let p = Person::from("Mark,21");
        assert_eq!(p.name, "Mark");
        assert_eq!(p.age, 21);
    }

    #[test]
    fn test_bad_age() {
        let p = Person::from("Mark,twenty");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 31);
    }

    #[test]
    fn test_missing_comma_and_age() {
        let p: Person = Person::from("Mark");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 31);
    }

    #[test]
    fn test_missing_age() {
        let p: Person = Person::from("Mark,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 31);
    }

    #[test]
    fn test_missing_name() {
        let p: Person = Person::from(",1");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 31);
    }

    #[test]
    fn test_missing_name_and_age() {
        let p: Person = Person::from(",");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 31);
    }

    #[test]
    fn test_missing_name_and_invalid_age() {
        let p: Person = Person::from(",one");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 31);
    }

    #[test]
    fn test_trailing_comma() {
        let p: Person = Person::from("Mike,32,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 31);
    }

    #[test]
    fn test_trailing_comma_and_some_string() {
        let p: Person = Person::from("Mike,32,dog");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 31);
    }
}
