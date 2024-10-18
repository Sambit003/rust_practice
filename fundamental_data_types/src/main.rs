// This program is spanned through number of functions to demonstrate the fundamental data types in Rust.
// By default, all the fundamenta data types are immutable in Rust, making it a safe lower level language to work with. In this file, we will mostly work with the immutable
// data types and later we will see how to make them mutable.

/*
    Undertanding Type Literal:
    1. What is it? - A type literal is a sequence of characters that represents a type.
    2. Why do we need it? - Type literals are used to specify the type of a variable or a function parameter.
    3. How to use it? - Type literals are used in Rust by appending a colon and the type literal to the variable or function parameter or appending the value with the type.
    4. Example: let x: i32 = 5; // Here, i32 is the type literal.
       Another example is let x = 5; // Here, Rust will infer the type of x as i32.
       Another advanced example is let x = 5i32; // Here, Rust will infer the type of x as i32 and value is 5.
*/

struct ArrayDemo;
struct TupleDemo;
struct TypeLiteralDemo;

impl TypeLiteralDemo {
    pub fn type_literal_demo() {
        // Type Literal
        let x: i32 = 5; // Here, i32 is the type literal.
        let y = 5; // Here, Rust will infer the type of x as i32.
        let z = 5i32; // Here, Rust will infer the type of x as i32 and value is 5.
        println!("x: {}", x);
        println!("y: {}", y);
        println!("z: {}", z);

        pub fn add(x: i32, y: i32) -> i32 {
            x + y
        } // return type literal
        add(x, y);
    }
}

impl ArrayDemo {
    pub fn array_demo() {
        // Simplest way of decalaring the array
        let array: [u32; 6] = [1, 2, 3, 4, 5, 6]; // Array of maximum 6 elements of type unsigned integer 32 bits (u32)
        let explicit_string_array: [&str; 3] = ["Hello", "World", "!"]; // Array of maximum 3 elements of type string slice with explicit type declaration
        let implicit_string_array = ["Hello", "World"]; // Array of maximum 2 elements of type string slice with implicit type declaration [rust analyzer will show you the type and element count]
        assert_eq!(array[0], 1);
        assert_eq!(array.len(), 6);
        assert_eq!(explicit_string_array.len(), 3);
        assert_eq!(implicit_string_array.len(), 2);
        println!("Array: {:?}", array);
        println!("String Array: {:?}", explicit_string_array);
        println!("Normal String Array: {:?}", implicit_string_array);

        /* For the common case of a long array filled with some value, you can write
        [V; N], where V is the value each element should have, and N is the
        length. For example, [true; 10000] is an array of 10,000 bool
        elements, all set to true: */
        let mut sieve = [true; 10000];
        for i in 2..100 {
            if sieve[i] {
                let mut j = i * i;
                while j < 10000 {
                    sieve[j] = false;
                    j += i;
                }
            }
        }
        assert!(sieve[211]);
        assert!(!sieve[9876]);

        // Array of 100 elements with default value 0
        let mut array_100 = [0; 100]; // Array of 100 elements with default value 0 (mutable array)
                                      /* Why Mutable Array?
                                       * 1. You can change the value of the array elements
                                       * 2. You can change the length of the array
                                       */
        println!("Array_100 before changing each values: {:?}", array_100);
        // Changing the value of the array elements
        for i in 0..100 {
            array_100[i] = i as i32;
        }
        println!("Array_100 after changing each values: {:?}", array_100);

        //Let's have a fixed size buffer array
        let buffer1  = [0u8; 1024]; // Method 1
        let buffer2: [u8; 1024]  = [0; 1024]; // Method 2
        println!("Buffer1: {:?}", buffer1);
        println!("Buffer2: {:?}", buffer2);
    }
}

impl TupleDemo {
    pub fn tuple_demo() {
        /*  A tuple is a pair, or triple, quadruple, quintuple, etc. (hence, n-tuple, or
        tuple), of values of assorted types. You can write a tuple as a sequence of
        elements, separated by commas and surrounded by parentheses. For
        example, ("Brazil", 1985) is a tuple whose first element is a
        statically allocated string, and whose second is an integer; its type is
        (&str, i32). Given a tuple value t, you can access its elements as
        t.0, t.1, and so on. */
        // Basic Tuple
        let tuple = (1, 2, 3, 4, 5);
        println!("Tuple: {:?}", tuple);
        println!("Tuple.0: {}", tuple.0);
        println!("Tuple.1: {}", tuple.1);
        println!("Tuple.2: {}", tuple.2);
        println!("Tuple.3: {}", tuple.3);
        println!("Tuple.4: {}", tuple.4);

        // Tuple demo with string destructuring
        let text = "I see the eigenvalue in thine eye";
        let (head, tail) = text.split_at(21);
        assert_eq!(head, "I see the eigenvalue ");
        assert_eq!(tail, "in thine eye");

        // Tuple allocation in Heap
        let t = (12, "eggs");
        let b = Box::new(t);
        println!("b: {:?}", b);
        // memory Address of the heap allocated tuple
        let address = &b as *const _ as usize;
        println!("Address of b: {:x}", address);
    }
}


fn main() {
    print!("Type Literal Demo: ");
    TypeLiteralDemo::type_literal_demo();

    print!("Array Demo: ");
    ArrayDemo::array_demo();

    print!("Tuple Demo: ");
    TupleDemo::tuple_demo();
}
