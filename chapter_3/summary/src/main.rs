fn main() {
    let n = 2;
    let temp_in_c = f_to_c(n as f64);
    println!("{n} degrees F is {temp_in_c} degrees C");

    let fib_result = nth_fib(n);
    println!("the {n}th Fib number is {fib_result}");

    n_days_of_christmas(n);
}

fn f_to_c(temp_in_f: f64) -> f64 {
    let temp_in_c = (temp_in_f - 32.0) * (5.0 / 9.0);

    temp_in_c
}

fn nth_fib(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    return nth_fib(n - 1) + nth_fib(n - 2);
}

fn n_days_of_christmas(n: u32) {
    println!(
        "On the {n}{} day of christmas my true love gave to me",
        if n == 1 {
            "st"
        } else if n == 2 {
            "nd"
        } else if n == 3 {
            "rd"
        } else {
            "th"
        }
    );
    for day in (1..n + 1).rev() {
        if day == 12 {
            println!("12 drummers drumming");
        }
        if day == 11 {
            println!("11 pipers piping");
        }
        if day == 10 {
            println!("10 lords a-leaping");
        }
        if day == 9 {
            println!("9 ladies dancing");
        }
        if day == 8 {
            println!("8 maids a-milking");
        }
        if day == 7 {
            println!("7 swans a-swaying");
        }
        if day == 6 {
            println!("6 geese a-laying");
        }
        if day == 5 {
            println!("5 golden rings");
        }
        if day == 4 {
            println!("4 calling birds");
        }
        if day == 3 {
            println!("3 french hens");
        }
        if day == 2 {
            println!("2 turtle doves");
        }
        if day == 1 {
            println!(
                "{}a partridge in a pear tree",
                if n > 1 { "and " } else { "" }
            );
        }
    }
}
