use crate::stack_study::Stack;
use crate::tool_util;

#[test]
fn test_stack_base() {
    let mut stack = Stack::new();
    stack.push(1);

    stack.push(2);

    stack.push(3);

    println!("size:{},{:?}", stack.len(), stack);
}


#[test]
fn test_stack_operation(){

    let mut stack = Stack::new();
    stack.push(1);

    stack.push(2);

    stack.push(3);

    println!("{:?}",stack);

    let mut_item = stack.peek_mut();
    if let Some(top) = mut_item {
        *top += 1;
    }

    println!("{:?}",stack);

}

#[test]
fn test_stack_iter(){

    let mut stack = Stack::new();
    stack.push(1);

    stack.push(2);

    stack.push(3);

    let sum = stack.iter().sum::<i32>();

    let mut addend = 0;
    for item in stack.iter_mut(){
        *item += 1;
        addend += 1;
    }

    let sum2 = stack.iter().sum::<i32>();
    println!("{}",sum2);

}


#[test]
fn test_stack_par(){
    let partten = "(())";
    assert_eq!(tool_util::par_check(partten),true);
}