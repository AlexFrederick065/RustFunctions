use std::io;
use max_subarray_sum::interface::Elements;
fn main() {
    is_panlindrome();
    find_index();
    shortest_word();
    check_prime();
    arr_median();
    longest_prefix();
    smallest_element();
    tree_depth();
    reverse_string();
    merge_arr();
    subarray_sum();


    
}

fn is_panlindrome() {
    println!("Please Enter a Word");

    // Create a mutable variable
    let mut str_check = String::new(); 

    // Get the Input and Assign it to the variable
    io::stdin().read_line(&mut str_check).expect("Failed to read line");

    // Remove newline characters
    let input = str_check.trim().to_lowercase();
    // Reverse the String
    let reverse_str: String = input.chars().rev().collect();
    
    if input == reverse_str {
        println!("Entered String is Palindrome");
    }else {
        println!("Entered String is not a Palindrome");
    }
}

fn find_index(){
    let arr: [i32; 6] = [1, 2, 3, 4, 5, 5];

    let index = arr.iter().position(|&x| x == 5).unwrap();
    println!("{}", index);
}

fn shortest_word(){
    println!("Please Enter a String");

    // Create a mutable variable
    let mut str = String::new(); 

    // Get the Input and Assign it to the variable
    io::stdin().read_line(&mut str).expect("Failed to read line");

    let mut shortest_word = String::new();

    let words: Vec<&str> = str.split_whitespace().collect();

    for item in words.iter() {
        if item.chars().count() < shortest_word.chars().count() || shortest_word.chars().count() == 0{
            shortest_word = item.to_string();
        }
    }
    println!("Shorts Word in the string : {shortest_word}");
}

fn check_prime(){
    // This is for both point number 4 and 10
    println!("Please Enter a Number");

    // Create a mutable variable
    let mut str = String::new();

    // Get the Input and Assign it to the variable
    io::stdin().read_line(&mut str).expect("Failed to read line");

    // Convert the String to integer
    let num: u32 = str.trim().parse().expect("This is not a interger");
    let mut i = 2;
    let mut flag = 0;

    while i <= num/2 {
        if num % i == 0 {
            flag = 1;
            break;
        }
        i +=1;
    }

    if flag == 0 {
        println!("Is a prime number");
    }else{
        println!("Not a prime number");
    }
}

fn arr_median(){
    let arr: [i32;6] = [1, 2, 3, 4, 5, 5];
    let arr_len = arr.len();
    // Check if array is of even or odd type
    if arr_len % 2 == 0 {
        let index = arr_len/2;
        let median = (arr[index] + arr[index-1]) as f64/2.0;
        println!("Median is {}", median);
    }else {
        let index = (arr_len + 1)/2;
        println!("Median is {}", arr[index - 1]);
    }
}

fn longest_prefix(){
    let mut arr = ["mint","minerals","minimum"];

    // Sort the Array
    arr.sort();

    let mut prefix = String::new();
    
    // Loop Through each character of the first element 
    // and compare it with the character of the last elements
    for (i, c) in arr[0].chars().enumerate() {
        if arr[0].chars().nth(i).unwrap() == arr[arr.len()-1].chars().nth(i).unwrap(){
            prefix.push(arr[0].chars().nth(i).unwrap());
        }else{
            break;
        }
    }
    println!("Longest Prefix is :{}", prefix);
}

fn smallest_element(){
    let mut arr: [i32;7] = [4, 2, 7, 7, 10, 9, 3];
    let k = 3;
    arr.sort();
    println!("Smallest Kth number : {}",arr[k-1]);
}



fn reverse_string(){
    println!("Please Enter a String");
    // Create a mutable variable
    let mut str = String::new();

    // Get the Input and Assign it to the variable
    io::stdin().read_line(&mut str).expect("Failed to read line");
    let reverse_str = str.chars().rev().collect::<String>();

    println!("Reversed String : {reverse_str}");
}

fn merge_arr(){
    let mut arr1 = vec![4, 5, 6, 7, 1, 10];
    arr1.sort();
    let mut arr2 = vec![3, 11, 14, 15, 16, 2, 12, 13];
    arr2.sort();

    let mut merge = [arr1, arr2].concat();
    merge.sort();
    println!("Array :{:?}", merge);
}

fn subarray_sum(){
    // By Using Crate
    let mut arr = [-3, -4, 5, 6, -2, 3, 5, -4];
    let mut elements = Elements::new(&mut arr);
    let max_sum = elements.find_max_sum().result();
    println!("Max Subarray Sum by crate : {}", max_sum);

    // Manually
    let manual_arr = [-3, -4, 5, 6, -2, 3, 5, -4];
    let mut max_sum = 0;
    let mut curr_sum = 0;

    for (index, item) in manual_arr.iter().enumerate() {
        curr_sum = curr_sum + arr[index];

        if index == 0 || max_sum < curr_sum {
            max_sum = curr_sum;
        }

        if curr_sum < 0 {
            curr_sum = 0;
        }

        println!(": {}", max_sum);
    }	
    println!("Max Subarray Sum manually :{max_sum}");

}


// Maximum Depth of a Binary Tree 
#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode { val, left: None, right: None }
    }
}
fn tree_depth(){
    let root = Some(Box::new(TreeNode {
        val: 1,
        left: Some(Box::new(TreeNode::new(2))),
        right: Some(Box::new(TreeNode {
            val: 3,
            left: Some(Box::new(TreeNode::new(4))),
            right: Some(Box::new(TreeNode::new(5))),
        })),
    }));

    // Calculating and printing the maximum depth
    let depth = max_depth(&root);
    println!("Maximum Depth: {}", depth);
}
fn max_depth(root: &Option<Box<TreeNode>>) -> i32{
    match root {
        None => 0,
        Some(node) => {
            let left_depth = max_depth(&node.left);
            let right_depth = max_depth(&node.right);
            1+left_depth.max(right_depth)
        }
    }
}
