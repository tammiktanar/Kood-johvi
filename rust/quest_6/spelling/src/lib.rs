
pub fn spell(n: u64) -> String {
    if n == 1000000 {
        return "one million".to_string();
    }

    if n == 0 {
        return "zero".to_string();
    }

    let nums =  vec!["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten"];
    let number = n.to_string();
    let mut count = 0;
    
    let mut res = "".to_string();
    for char in number.chars().rev() {
        if count == 0 {
            res.push_str(nums[char.to_digit(10).unwrap() as usize]);
        }

        if count == 1 {
            let mut temp = nums[char.to_digit(10).unwrap() as usize].to_owned();
            temp.push_str("ty-");
            temp.push_str(&res);
            res = temp;
        }

        if count == 2 {
            let mut temp = nums[char.to_digit(10).unwrap() as usize].to_owned();
            temp.push_str(" hundred ");
            temp.push_str(&res);
            res = temp;
        }

        if count == 3 {
            let mut temp = nums[char.to_digit(10).unwrap() as usize].to_owned();
            temp.push_str(" thousand ");
            temp.push_str(&res);
            res = temp;
        }

        if count == 4 {
            let mut temp = nums[char.to_digit(10).unwrap() as usize].to_owned();
            temp.push_str("ty-");
            temp.push_str(&res);
            res = temp;
        }

        if count == 5 {
            let mut temp = nums[char.to_digit(10).unwrap() as usize].to_owned();
            temp.push_str(" hundred ");
            temp.push_str(&res);
            res = temp;
        }

        count +=1;
    }


    
    res = res.replace("onety-one", "eleven");
    res = res.replace("onety-two", "twelve");
    res = res.replace("onety-three", "thirteen");
    res = res.replace("onety-four", "fourteen");
    res = res.replace("onety-five", "fifteen");
    res = res.replace("onety-six", "sixteen");
    res = res.replace("onety-seven", "seventeen");
    res = res.replace("onety-eight", "eighteen");
    res = res.replace("onety-nine", "ninteen");
    res = res.replace("onety", "ten");
    res = res.replace("twoty", "twenty");
    res = res.replace("threety", "thirty");
    res = res.replace("fourty", "forty");
    res = res.replace("fivety", "fifty");
    res = res.replace("zeroty-", "");
    res = res.replace("-zero", "");
    res = res.replace("zero", "");
    res = res.replace("  hundred", " ");
    res = res.replace("  ", " ");
    res = res.trim_end().to_string();



    return res;
}