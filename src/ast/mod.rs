#[derive(Debug)]
struct EvalError {
    msg: String,
}

// Hmm, dunno if this is good...
fn cant_apply_err<T: Value + ?Sized, U: Value + ?Sized>(operator: &str, left: &T, right: &U) -> EvalError {
    EvalError {
        msg: format!("Can't apply {} {} {}", left.name(), operator, right.name())
    }
}

#[derive(PartialEq, Debug)]
enum TypeMatcher<'a> {
    Integer(&'a u32),
    String(&'a String),
    Void,
}

trait Value {
    fn type_matcher(&self) -> TypeMatcher;
    fn name(&self) -> &str {
        match self.type_matcher() {
            TypeMatcher::Integer(_) => "Integer",
            TypeMatcher::String(_) => "String",
            TypeMatcher::Void => "Void",
        }
    }
}

trait Operable: Value {
    fn apply_plus<T: Value>(&self, other: &T) -> Result<Box<dyn Value>, EvalError> {
        // By default, return can't apply error
        Err(cant_apply_err("+", self, other))
    }
}

// VOID

struct VoidValue;

impl Value for VoidValue {
    fn type_matcher(&self) -> TypeMatcher {
        TypeMatcher::Void
    }
}

impl Operable for VoidValue {}

// INTEGER

struct IntegerValue {
    value: u32,
}

impl Value for IntegerValue {
    fn type_matcher(&self) -> TypeMatcher {
        TypeMatcher::Integer(&self.value)
    }
}

impl Operable for IntegerValue {
    fn apply_plus<T: Value>(&self, other: &T) -> Result<Box<dyn Value>, EvalError> {
        match other.type_matcher() {
            TypeMatcher::Integer(other_val) =>
                Ok(Box::new(IntegerValue { value: self.value + other_val })),
            TypeMatcher::String(other_val) => {
                let mut new_string = String::new();
                new_string.push_str(self.value.to_string().as_str());
                new_string.push_str(other_val.as_str());
                Ok(Box::new(StringValue { value: new_string }))
            },
            _ => Err(cant_apply_err("+", self, other))
        }
    }
}


// STRING

struct StringValue {
    value: String,
}

impl Value for StringValue {
    fn type_matcher(&self) -> TypeMatcher {
        TypeMatcher::String(&self.value)
    }
}

impl Operable for StringValue {
    fn apply_plus<T: Value>(&self, other: &T) -> Result<Box<dyn Value>, EvalError> {
        match other.type_matcher() {
            TypeMatcher::String(other_val) => {
                let mut new_string = String::new();
                new_string.push_str(self.value.as_str());
                new_string.push_str(other_val.as_str());
                Ok(Box::new(StringValue { value: new_string }))
            },
            TypeMatcher::Integer(other_val) => {
                let mut new_string = String::new();
                new_string.push_str(self.value.as_str());
                new_string.push_str(other_val.to_string().as_str());
                Ok(Box::new(StringValue { value: new_string }))
            },
            _ => Err(cant_apply_err("+", self, other))
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_void_plus_something() {
        let val_void = VoidValue {};
        let val_int = IntegerValue { value: 123 };
        let val_string = StringValue { value: "foo".to_string() };

        eval_fails(val_void.apply_plus(&val_int), "Can't apply Void + Integer");
        eval_fails(val_int.apply_plus(&val_void), "Can't apply Integer + Void");

        eval_fails(val_string.apply_plus(&val_void), "Can't apply String + Void");
        eval_fails(val_void.apply_plus(&val_string), "Can't apply Void + String");
    }

    #[test]
    fn test_integer_plus_integer() {
        let val1 = IntegerValue { value: 123 };
        let val2 = IntegerValue { value: 1 };

        evals_to(val1.apply_plus(&val2),
                 TypeMatcher::Integer(&124));
        evals_to(val2.apply_plus(&val1),
                 TypeMatcher::Integer(&124));
    }

    #[test]
    fn test_string_plus_string() {
        let val1 = StringValue { value: "foo".to_string() };
        let val2 = StringValue { value: "bar".to_string() };

        evals_to(val1.apply_plus(&val2),
                 TypeMatcher::String(&"foobar".to_string()));
        evals_to(val2.apply_plus(&val1),
                 TypeMatcher::String(&"barfoo".to_string()));
    }

    #[test]
    fn test_integer_plus_string() {
        let val1 = IntegerValue { value: 123 };
        let val2 = StringValue { value: "bar".to_string() };

        evals_to(val1.apply_plus(&val2),
                 TypeMatcher::String(&"123bar".to_string()));
        evals_to(val2.apply_plus(&val1),
                 TypeMatcher::String(&"bar123".to_string()));
    }

    fn evals_to(result: Result<Box<dyn Value>, EvalError>, expected: TypeMatcher) {
        match result {
            Ok(result) => {
                assert_eq!(result.type_matcher(), expected)
            },
            Err(err) => panic!("Expecting result, but gor EvalError: {}", err.msg)
        }
    }

    fn eval_fails(result: Result<Box<dyn Value>, EvalError>, msg: &str) {
        match result {
            Ok(result) =>
                panic!("Expected to eval to fail, but got: {:?}", result.type_matcher()),
            Err(err) =>
                assert_eq!(msg, err.msg)
        }
    }
}