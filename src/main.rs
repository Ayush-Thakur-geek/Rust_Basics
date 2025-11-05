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
}
//i8, u8, i16, u32, i32, i128, u128, isize, usize
//f32, f64