mod mean;
mod pig_latin;

fn main() {
    println!("PIG LATIN");
    println!("hello: {}", pig_latin::translate("hello"));
    println!("my: {}", pig_latin::translate("my"));
    println!("name: {}", pig_latin::translate("name"));
    println!("is: {}", pig_latin::translate("is"));
    println!("komal: {}", pig_latin::translate("komal"));
    println!("aloha: {}", pig_latin::translate("aloha"));
    println!("eiffel: {}", pig_latin::translate("eiffel"));
    println!("ice: {}", pig_latin::translate("ice"));
    println!("oscar: {}", pig_latin::translate("oscar"));
    println!("under: {}", pig_latin::translate("under"));

    println!();
    println!("MATH");
    let mut numbers = [1, 2, 3, 4, 5, 2, 1, 5, 3, 5];

    println!("mean: {}", mean::calculate_mean(&numbers));
    println!("median: {}", mean::calculate_median(&mut numbers));
    println!("mode: {}", mean::calculate_mode(&numbers));
}
