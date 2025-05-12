fn main() {
    let gifts = [
        "a Partridge in a Pear Tree",
        "Two turtle-doves", 
        "Three French hens", 
        "Four calling birds", 
        "Five golden rings", 
        "Six geese a-laying", 
        "Seven swans a-swimming", 
        "Eight maids a-milking", 
        "Nine ladies dancing", 
        "Ten lords a-leaping", 
        "Eleven pipers piping", 
        "Twelve drummers drumming"
    ];

    let ordinal_days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh",
        "eighth", "ninth", "tenth", "eleventh", "twelfth",
    ];

    for i in 0..12 {
        println!("On the {} day of Christmas", ordinal_days[i]);
        println!("My true love sent to me");

        for j in (0..=i).rev() {
            if i > 0 && j == 0 {
                println!("And a partridge in a pear tree!!!");
            } else {
                println!("{}", gifts[j]);
            }
        }
        println!();
    }
}
