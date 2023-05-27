fn main() {
    let mut x = sum_of_a_sequence(2,2,2);
    println!("2,2,2 --> {x}");
    x = sum_of_a_sequence(2,6,2);
    println!("2,6,2 --> {x} (2 + 4 + 6)");
    x = sum_of_a_sequence(1,5,1);
    println!("1,5,1 --> {x} (1 + 2 + 3 + 4 + 5)");
    x = sum_of_a_sequence(1,5,3);
    println!("1,5,3 --> {x} (1 + 4)");
}

fn sum_of_a_sequence(begin: u16, end: u16, step: u16) -> u16 {
    let mut sum = 0;
    let range = (f64::from(end)/f64::from(step)).ceil() as u16;

    if begin > end {
        return sum;
    } else if begin == end {
        return begin;
    } else {
        sum = begin;
        for number in 1..range {
            if (step*number+begin) > end {
                break;
            } else {
                sum = sum + (step*number+begin);
            }
        }
        return sum;
    }
}
