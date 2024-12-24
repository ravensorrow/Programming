use compute_pi::compute_pi_str;

fn main() {
    /* Specify the number of digits you want PI to compute to */
    let digits = 10000;

    /* Time to do the heavy lifting */
    let pi = compute_pi_str(digits);

    /* Print the result */
    println!("π to {} decimal places: {}", digits, pi);
}
