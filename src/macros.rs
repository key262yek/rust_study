
#[allow(unused_imports)]
use crate::prelude::*;

#[macro_export]
macro_rules! parse_line {
    ($($t: ty),+) => ({
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.split_whitespace();
        ($(iter.next().unwrap().parse::<$t>().unwrap()),+)
    })
}


#[macro_export]
macro_rules! parse_list {
  ($t: ty) => ({
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let list: Vec<$t> = line.split_whitespace()
                            .map(|w| w.parse::<$t>().unwrap()).collect();
    list
  })
}

#[macro_export]
macro_rules! read_single_line{
    () => {{
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();

        line
    }}
}

#[macro_export]
macro_rules! unwrap_with_msg_result{
    ($t : expr, $err_msg : literal, $label : tt) => {
        match $t {
            Ok(v) => v,
            Err(_) => {
                println!($err_msg);
                continue $label;
            }
        }
    }
}

#[macro_export]
macro_rules! unwrap_with_msg_option{
    ($t : expr, $err_msg : literal, $label : tt) => {
        match $t {
            Some(v) => v,
            None => {
                println!($err_msg);
                continue $label;
            }
        }
    }
}

#[macro_export]
macro_rules! parse_line_err {
    ($($t: ty),+, $err_msg : literal) => ({
        #[allow(unused_parens)]
        let res : ($($t), +);

        'outer: loop{
            let mut line = String::new();
            unwrap_with_msg_result!(std::io::stdin().read_line(&mut line), $err_msg, 'outer);

            let mut iter = line.split_whitespace();
            res = (
                $(
                unwrap_with_msg_result!(
                    unwrap_with_msg_option!(iter.next(), $err_msg, 'outer).parse::<$t>(), $err_msg, 'outer
                    )
                ),+
            );

            break;
        }

        res
    })
}

#[macro_export]
macro_rules! parse_list_err {
  ($t: ty, $err_msg : literal) => ({
    let list : Vec<$t>;

    'outer: loop{
        let mut line = String::new();
        unwrap_with_msg_result!(std::io::stdin().read_line(&mut line), $err_msg, 'outer);
        let parsed : Vec<Option<$t>> = line.split_whitespace()
                .map(|w| w.parse::<$t>().ok()).collect();
        let mut test = Vec::<$t>::new();
        for p in parsed{
            test.push(unwrap_with_msg_option!(p, $err_msg, 'outer));
        }

        list = test.clone();
        break;
    }

    list
  })
}
