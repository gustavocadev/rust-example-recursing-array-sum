fn get_arr_sum(books: Vec<i32>) -> i32 {
  if books.len() == 0 {
    return 0;
  }

  // capture the last value of each invokation
  let last_value = books[books.len() - 1];

  // remove the last element of the array in each invokation
  let dynamic_arr = books[..books.len() - 1].to_vec();

  return get_arr_sum(dynamic_arr) + last_value;
}

fn get_arr_sum2(books: Vec<i32>) -> i32 {
  if books.len() == 0 {
    return 0;
  }

  // capture the first value of each invocation
  let first_value_arr = books[0];

  let dynamic_arr = books[books.len() - books.len() + 1..].to_vec();

  return get_arr_sum2(dynamic_arr) + first_value_arr;
}

fn get_arr_sum3(books: Vec<i32>) -> i32 {
  if books.len() == 1 {
    return books[0];
  }

  // capture the first value of each invocation
  let first_value_arr = books[0];

  let dynamic_arr = books[1..].to_vec();

  return get_arr_sum3(dynamic_arr) + first_value_arr;
}

fn main() {
  // image they're book prices!
  let numbers = vec![50, 100, 150];

  println!("{}", get_arr_sum3(numbers));
}
