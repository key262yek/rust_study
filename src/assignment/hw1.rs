// 1. 정수 1부터 100까지 더하여 화면에 출력하는 프로그램을 작성하세요.
// 2. 터미널에서 문자열을 입력 받아서 그 문자열을 역순으로 출력하세요.
//     예를 들어 터미널에서 "abbd" 를 입력 받았으면 "dbba"를 출력하세요.
// 3. 임의의 숫자를 입력 받고(만약 문자열을 입력하면 에러메시지를 내고 다시 입력 받음) 그 숫자를 20 자리의 xxx,xxx,xxx 형태로 출력하세요.
//     만약 출력 문자열의 자릿수가 20 자리가 안되면 앞에 '0' 을 붙여주세요.
//     예를 들어 1234567 을 터미널에서 입력했으면 000000000001,234,567 와 같이 출력하세요.


#[allow(unused_imports)]
use crate::prelude::*;


#[allow(dead_code)]
pub fn prob1(){
	for i in 1..101{
		println!("{:?}", i);
	}
}

#[allow(dead_code)]
pub fn prob2(){

    let line = read_single_line!();
    let reversed = line.trim().chars().rev().collect::<String>();
    println!("{:?}", reversed);
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FormattedNumber(i128);

impl FromStr for FormattedNumber{
	type Err = std::num::ParseIntError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let num = s.parse::<i128>()?;
		return Ok(Self(num));
	}
}

impl Display for FormattedNumber{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result{
        let mut res : Vec<String> = Vec::new();
        let mut num = self.0;

        while num > 999 {
        	let three_digit = num % 1000;
        	num = num / 1000;

        	res.push(format!("{:03}", three_digit));
        }
        res.push(format!("{}", num));

        let formatted = res.into_iter().rev().collect::<Vec<String>>().join(",");
        // let length = formatted.len();

        // if length < 20{
        // 	let zeros = "0".repeat(20 - length).to_owned();
        // 	formatted = zeros + &formatted;
        // }

        write!(f, "{:0>20}", formatted)
    }
}

#[allow(dead_code)]
pub fn prob3(){
   	let x = parse_line_err!(FormattedNumber, "You should give integer");
    println!("{}", x);
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_fromstr(){
    	let test = "13";
    	let result : FormattedNumber = test.parse().unwrap();
    	assert_eq!(result, FormattedNumber(13));


    	let test = "0";
    	let result : FormattedNumber = test.parse().unwrap();
    	assert_eq!(result, FormattedNumber(0));

    	let test = "-12312";
    	let result : FormattedNumber = test.parse().unwrap();
    	assert_eq!(result, FormattedNumber(-12312));
    }

    #[test]
    #[should_panic]
    fn test_fromstr_panic_non_numeric(){
    	let test = "Not a number";
    	let _result : FormattedNumber = test.parse().unwrap();
    }

    #[test]
    #[should_panic]
    fn test_fromstr_panic_float(){
    	let test = "21314.23123";
    	let _result : FormattedNumber = test.parse().unwrap();
    }

    #[test]
    fn test_display(){
    	let test = FormattedNumber(12314120);
    	let result = format!("{}", test);
    	assert_eq!(result, "000000000012,314,120");

    	let test = FormattedNumber(314120314120314120314120);
    	let result = format!("{}", test);
    	assert_eq!(result, "314,120,314,120,314,120,314,120");
    }
}

