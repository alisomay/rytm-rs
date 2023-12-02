use rytm_rs_macro::parameter_range;

// Mock error types and structures for testing
#[derive(Debug, PartialEq)]
enum ParameterError {
    Range {
        value: String,
        parameter_name: String,
    },
}

#[derive(Debug, PartialEq)]
enum RytmError {
    Parameter(ParameterError),
}

// Test function with inclusive range
#[parameter_range(range = "length:0..=127")]
fn set_length_inclusive(length: usize) -> Result<(), RytmError> {
    Ok(())
}

// Test function with exclusive range
#[parameter_range(range = "age:18..60")]
fn set_age_exclusive(age: usize) -> Result<(), RytmError> {
    Ok(())
}

#[test]
fn test_within_inclusive_range() {
    assert_eq!(set_length_inclusive(0), Ok(())); // Test lower bound
    assert_eq!(set_length_inclusive(127), Ok(())); // Test upper bound
    assert_eq!(set_length_inclusive(50), Ok(())); // Test within range
}

#[test]
fn test_outside_inclusive_range() {
    assert_eq!(
        set_length_inclusive(128),
        Err(RytmError::Parameter(ParameterError::Range {
            value: 128.to_string(),
            parameter_name: "length".to_string()
        }))
    );
    assert_eq!(
        set_length_inclusive(usize::MAX),
        Err(RytmError::Parameter(ParameterError::Range {
            value: usize::MAX.to_string(),
            parameter_name: "length".to_string()
        }))
    );
}

#[test]
fn test_within_exclusive_range() {
    assert_eq!(set_age_exclusive(18), Ok(())); // Test lower bound (inclusive)
    assert_eq!(set_age_exclusive(59), Ok(())); // Test within range
}

#[test]
fn test_outside_exclusive_range() {
    assert_eq!(
        set_age_exclusive(60),
        Err(RytmError::Parameter(ParameterError::Range {
            value: 60.to_string(),
            parameter_name: "age".to_string()
        }))
    ); // Test upper bound (exclusive)
    assert_eq!(
        set_age_exclusive(17),
        Err(RytmError::Parameter(ParameterError::Range {
            value: 17.to_string(),
            parameter_name: "age".to_string()
        }))
    ); // Test below range
}

// Test for invalid range specification
#[test]
#[should_panic(expected = "Invalid range start")]
fn test_invalid_range_specification() {
    #[parameter_range(range = "number:abc..def")]
    fn set_invalid_range(number: usize) -> Result<(), RytmError> {
        Ok(())
    }

    set_invalid_range(10).unwrap();
}
