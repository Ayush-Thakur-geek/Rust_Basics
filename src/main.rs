// mod constants;

static mut COUNTER: u32 = 0;
fn main() {
    let value = 128_u32;
    let y: i8 = value as i8;
    println!("Y: {}", y);
    #[allow(unused_variables)]//to suppress the warnings for unused variables
    let (mut _a, mut _b, mut _c) = (10_u8, 32_i8, 12_u32);
    let code = b'+';
    println!("{}", code as char);
    let infinity_symbol = '\u{221E}';
    println!("symbol = {}, usv = {}", infinity_symbol, infinity_symbol as u32);

    let usv_of_inf = 0x221e_u32;
    if let Some(inf_symbol) = char::from_u32(usv_of_inf) {
        println!("symbol = {}", inf_symbol);
    } else {
        println!("Not a valid Unicode scalar value");
    }

    let _arr : [i32; 5] = [1, 2, 3, 4, 5];
    //or
    let arr2 = [1, 2, 3, 4, 5];

    let _arr3 = [1, 2, 3_u8, 4, 5];

    let arr4: [i32; 10] = [0; 10];
    let arr5 = [5_u8; 4];
    let arr6 = [0; 10]; //default is i32

    println!("arr2 = {:?}", arr2);

    unsafe {
        let counter_ptr = std::ptr::addr_of_mut!(COUNTER);
        *counter_ptr += 1;
        println!("COUNTER {}", *counter_ptr);
    }

    let mut value = 42;
    let ref_of_value = &mut value;
    println!("ref_of_value: {}", *ref_of_value + 1); //Manual derefrencing
    println!("ref_of_value: {:p}", ref_of_value); //Automatic deferencing

    let mut num1 = 50; //mutable referent
    // let ref_of_num1 = &num1; //immutable borrow
    let mut ref_of_num2 = &mut num1;
    let mut ref_of_num3 = &mut ref_of_num2;
    ref_of_num3 = &mut &mut 10;
    // println!("{}", ref_of_num3);
    // println!("{}", ref_of_num2);

    let mut arr = [1, 2, 3, 4];
    let s1 = &mut arr[1..=3]; //add '=' for making it inclusive
    s1[1] = 21;
    println!("{:?}", arr);

    let arr2 = [10, 23, 34, 564, 221];
    let slice = &arr2[2..];
    let mut sum = 0;

    for i in slice {
        sum += *i;
    }

    println!("sum: {}", sum);
}
//i8, u8, i16, u32, i32, i128, u128, isize, usize
//f32, f64