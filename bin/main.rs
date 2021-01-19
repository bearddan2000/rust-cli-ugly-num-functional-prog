/*This function divides a by greatest
 divisible power of b*/
 fn max_divide(a :i32, b :i32) -> i32 {
     if a % b != 0 {
        return a;
     }
     let c = a/b;
     return max_divide(c, b);
 }

 /* Function to check if a number
 is ugly or not */
 fn is_ugly(n :usize, args :&[i32], no :i32) -> bool {
   if n >= args.len() {
    return no == 1;
    }
   let c = max_divide(no, args[n]);
   return is_ugly(n+1, args, c);
 }
fn find_ugly(n :usize, i :i32, count :usize, args :&[i32]) -> i32 {
  if n < count {
    return i-1;
  }
  if is_ugly(0, args, i) {
    return find_ugly(n, i+1, count+1, args);
  }
  else {
    return find_ugly(n, i+1, count, args);
  }
}

fn main() {
  let input = 10;
  let args = [2,3,5];
  println!( "[INPUT] {}", input);
  let output = find_ugly(input as usize, 1, 1 as usize, &args);
  println!( "[OUTPUT] {}", output);
}
