fn main() {
    /*
        SCENARIO 2:

        1. Create an array with the temperatures for each day of the week
        2. Use a for loop to calculate sum of temperatures
        3. Find and print the average
     */

    let temperatures = [70.1, 80.2, 75.3, 68.0, 71.8, 77.7, 65.5];
    let mut weekly_sum = 0.0;
    for temp in temperatures {
        weekly_sum += temp;
    }
    let average_temp = weekly_sum / temperatures.len() as f64;  /* Casting */ /* I changed f32 to f64 */

    println!("The weekly sum is {}", weekly_sum);
    println!("The average weekly temperature is {}", average_temp);
}