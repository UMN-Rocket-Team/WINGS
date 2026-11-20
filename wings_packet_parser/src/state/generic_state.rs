pub fn result_to_string<ReturnType, ErrorType: std::fmt::Display>(
    use_struct_result: Result<ReturnType, ErrorType>,
) -> Result<ReturnType, String> {
    match use_struct_result {
        Ok(ok) => Ok(ok),
        Err(err) => Err(err.to_string()),
    }
}
