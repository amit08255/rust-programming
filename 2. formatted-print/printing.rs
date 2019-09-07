fn main(){

	/*below statement stringify the given parameter value (31) and prints the formatted string - 31 days
	  remember that it only works when single parameter value is passed.
	 */
	println!("{} days", 31);

	/*below statement formats string with given parameter values and prints formatted string - Alice in the Wonderland
	 */
	println!("{0} in the {1}", "Alice", "Wonderland");

	/*To make your code more readable you can also name parameters passed to format string.
	  Below statement prints formatted string - Amit is going to Delhi
	 */
	println!("{name} is going to {place}", name="Amit", place="Delhi");

	/* In below statement, we used two types of formatting. First one we used before, second one is called special formatting.
	   {:b} - This type of formatting is used to convert parameter value into a specific type. {:b} converts value - 2 into binary
	   which is - 10. Every special formatting starts with ":"
	 */

	println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

	/* Below statement implements :? for formatting, ":?" is called debug trait and is used to print value for debugging*/

	println!("{:?}", 23);

	/* Below statement uses .2 which specifies that print only 2 digits after decimal in given floating point value*/
	println!("{:.2}", 9.564366);

	/*{:.*} syntax, which sets the number of decimal places in floating-point types.
	  If this syntax is used, then the number of characters to print precedes the actual value to be formatted
	  Below staement will print - 7.388
	 */
	println!("{:.*}", 3, 7.3876578);

	println!("Lowercase hexadecimal value of 15 = {:x}", 15);

	println!("Uppercase hexadecimal value of 12 = {:X}", 12);

	let x = 23;

	println!("Memory location of variable x = {:p}", &x);

	println!("Binary value of 123 = {:b}", 123);

	println!("Uppercase hexadecimal value of 12 with 0X = {:#X}", 12);

	println!("Binary value of 123 with 0b = {:#b}", 123);

}	