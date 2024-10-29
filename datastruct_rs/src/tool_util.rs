use crate::stack_study::Stack;

pub(crate) fn par_check(par: &str) -> bool {
    let mut char_list = Vec::new();

    for c in par.chars() {
        char_list.push(c);
    }

    let mut index = 0;

    let mut balance = true;

    let mut stack = Stack::new();

    while index < char_list.len() && balance {
        let c = char_list[index];

        if '(' == c {
            stack.push(c);
        } else {
            if stack.is_empty() {
                balance = false;
                break;
            } else {
                let _r = stack.pop();
            }
        }
        index += 1;
    }
    balance && stack.is_empty()
}