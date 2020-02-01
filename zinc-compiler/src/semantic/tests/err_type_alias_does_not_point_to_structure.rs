//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;
use crate::semantic::Error as SemanticError;
use crate::semantic::Path;
use crate::syntax::MemberString;
use crate::Error;

#[test]
fn test() {
    let input = r#"
type X = field;

fn main() {
    let data = struct X;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::TypeAliasDoesNotPointToStructure(
            Location::new(5, 23),
            Path::new(
                Location::new(5, 23),
                MemberString::new(Location::new(5, 23), "field".to_owned()),
            )
            .to_string(),
        ),
    ));

    let result = super::get_binary_result(input);

    assert_eq!(expected, result);
}