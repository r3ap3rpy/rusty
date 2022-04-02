/// First line
///
/// The next line presents detailed docs blalabla
///
/// ``` 
/// let result = doccomments::add(2,3);
/// assert_eq!(result,5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
/// jaksdjkasjdkajsdkasjd
///
/// aksdhkjahjkalksjdkasjdk
///
/// #Examples
///
/// ```
/// let result = doccomments::div(10,2);
/// assert_eq!(result,5);
/// ```
///
/// The function panisc if the second argument is zero.
/// ```rust,should_panic
/// doccomments::div(10,0);
/// ```
pub fn div(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Zero division is not allowed!");
    }
    a / b
}
