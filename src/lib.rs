//!
//! Rust name_of! Macro
//!
//! MIT License
//! Copyright (c) 2018 SilentByte <https://silentbyte.com/>
//!

#![crate_name = "nameof"]

/// Takes a binding, type, or function as an argument and returns its
/// unqualified string representation. If the identifier does not exist
/// in the current context, the macro will cause a compilation error.
/// This macro is mainly intended for debugging purposes and to improve
/// the refactoring experience compared to `stringify!()`.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate nameof;
/// # fn main() {
/// // Bindings
/// let text = "Hello, World!";
/// println!("Variable `{}` holds `{}`.", name_of!(text), text);
///
/// // Functions
/// fn greet() { }
/// println!("Function is called `{}`.", name_of!(greet));
///
/// // Types & Fields
/// struct TestStruct { test_field: i32 }
/// println!("Struct is called `{}`.", name_of!(type TestStruct));
/// println!("Standard Types: `{}`.", name_of!(type i32));
/// println!("Field is called `{}`.", name_of!(test_field for TestStruct));
///
/// # }
/// ```
#[macro_export]
macro_rules! name_of {
    // Covers Bindings
    ($n: ident) => {{
        || {
            &$n;
        };
        stringify!($n)
    }};

    // Covers Types
    (type $t: ty) => {{
        name_of_type!($t)
    }};

    // Covers Struct Fields
    ($n: ident for $t: ty) => {{
        |f: $t| {
            let _ = &f.$n;
        };
        stringify!($n)
    }};
}

/// Alternative for the `name_of!(type T)` macro specifically for types.
///
/// # Examples
///
/// ```
/// #[macro_use] extern crate nameof;
/// # fn main() {
/// // Alternative for Types
/// struct TestStruct { test_field: i32 }
/// println!("Struct is called `{}`.", name_of_type!(TestStruct));
/// println!("Struct is called `{}`.", name_of_type!(i32));
///
/// # }
/// ```
#[macro_export]
macro_rules! name_of_type {
    // Covers Types
    ($t: ty) => {{
        || {
            let _: $t;
        };
        stringify!($t)
    }};
}

#[cfg(test)]
mod tests {
    fn test_fn() {
        //
    }

    struct TestStruct {
        test_field: i32,
    }

    struct TestGenericStruct<T> {
        test_field: T,
    }

    struct TestGenericStructMultiType<T, U> {
        test_field_t: T,
        test_field_u: U,
    }

    #[test]
    fn name_of_binding() {
        let test_variable = 123;
        assert_eq!(name_of!(test_variable), "test_variable");
    }

    #[test]
    fn name_of_fn() {
        assert_eq!(name_of!(test_fn), "test_fn");
    }

    #[test]
    fn name_of_type() {
        assert_eq!(name_of!(type i32), "i32");
        assert_eq!(name_of_type!(i32), "i32");
    }

    #[test]
    fn name_of_struct() {
        assert_eq!(name_of!(type TestStruct), "TestStruct");
        assert_eq!(name_of_type!(TestStruct), "TestStruct");
    }

    #[test]
    fn name_of_generic_struct() {
        assert_eq!(
            name_of!(type TestGenericStruct<i32>),
            "TestGenericStruct<i32>"
        );

        assert_eq!(
            name_of_type!(TestGenericStruct<i32>),
            "TestGenericStruct<i32>"
        );
    }

    #[test]
    fn name_of_generic_multi_type_struct() {
        assert_eq!(
            name_of!(type TestGenericStructMultiType<i32, TestGenericStruct<String>>),
            "TestGenericStructMultiType<i32, TestGenericStruct<String>>"
        );

        assert_eq!(
            name_of_type!( TestGenericStructMultiType<i32, TestGenericStruct<String>>),
            "TestGenericStructMultiType<i32, TestGenericStruct<String>>"
        );
    }

    #[test]
    fn name_of_struct_field() {
        assert_eq!(name_of!(test_field for TestStruct), "test_field");
    }

    #[test]
    fn name_of_generic_struct_field() {
        assert_eq!(
            name_of!(test_field for TestGenericStruct<i32>),
            "test_field"
        );
    }

    #[test]
    fn name_of_generic_multi_type_struct_field() {
        assert_eq!(
            name_of!(test_field_t for TestGenericStructMultiType<i32, TestGenericStruct<String>>),
            "test_field_t"
        );

        assert_eq!(
            name_of!(test_field_u for TestGenericStructMultiType<i32, TestGenericStruct<String>>),
            "test_field_u"
        );
    }
}
