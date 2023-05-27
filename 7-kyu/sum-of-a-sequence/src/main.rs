fn main() {
    let mut _x = sum_of_a_sequence(2,2,2);
    _x = sum_of_a_sequence(2,6,2);
    _x = sum_of_a_sequence(1,5,1);
    _x = sum_of_a_sequence(1,5,3);
}

fn sum_of_a_sequence(begin: u16, end: u16, step: u16) -> u16 {
    let mut sum = 0;
    let mut string = String::new();
    let mut tmp_string = String::new();
    let range = (f64::from(end)/f64::from(step)).ceil() as u16;

    string.push_str(&(begin.to_string()));
    string.push_str(",");
    string.push_str(&(end.to_string()));
    string.push_str(",");
    string.push_str(&(step.to_string()));
    string.push_str(" --> ");

    if begin > end {
        string.push_str(&(sum.to_string()));
        println!("{string}");
        return sum;
    } else if begin == end {
        sum = begin;
        string.push_str(&(sum.to_string()));
        println!("{string}");
        return begin;
    } else {
        sum = begin;
        tmp_string.push_str(" (");
        tmp_string.push_str(&(begin.to_string()));
        tmp_string.push_str(" + ");
        for number in 1..range+1 {
            if (step*number+begin) > end {
                break;
            } else {
                tmp_string.push_str(&((step*number+begin).to_string()));
                sum = sum + (step*number+begin);
            }
            if number < range-1 {
                tmp_string.push_str(" + ");
            }
        }
        tmp_string.push_str(")");
        string.push_str(&(sum.to_string()));
        string.push_str(&(tmp_string));
        println!("{string}");
        return sum;
    }
}
