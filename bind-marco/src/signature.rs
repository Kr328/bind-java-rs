use crate::types::Type;

pub fn method_signature(return_type: &Type, argument_types: impl Iterator<Item = Type>) -> String {
    let mut result = String::new();

    result.push('(');
    for t in argument_types {
        result.push_str(&t.to_signature());
    }
    result.push(')');

    result.push_str(&return_type.to_signature());

    result
}
