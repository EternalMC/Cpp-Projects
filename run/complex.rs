use std::io;

//from a java program I made 

	pub fn main() {
		println!("New Program.");
		
			println!("Welcome to a game.");
		
			print("Please enter your name: ");
				let mut name = String::new();
					io::stdin::().read_line(&mut name).expect(“error: unable to read user input”);
				
				println!("Name: {}", name);

		numgen(); //more errors 
	}
	pub fn numgen() {
		  //Random rand = new Random(); 
			int mut hp = 10;
			//String name;
	      // Generate random integers in range 0 to 50 
	      int rand_int1 = rand.nextInt(51); 
	      int rand_int2 = rand.nextInt(51); 
	      println!("Starting HP: {}", hp);
	      try {
			this.wait(2);
		} catch (InterruptedException e) {
			// TODO Auto-generated catch block
			e.printStackTrace();
		}
	      // Print random integers 
	      if rand_int1 < 20  {
	    	  println!("Dice Roll: "+ rand_int1);
	    	  hp -= 1;
	    	  println!("HP: " + hp);
	      }
	      else {
	    	  println!("Dice Roll: "+ rand_int1);
	    	  //println(name + " lives a another day.");
	    	  println!(hp);
	      }
	      
	      System.out.println("Random Integers: "+ rand_int2); 
	   
	      //do not place anything after this
}

/*
use std::io;
use std::io::*;
fn main(){
let mut input = String::new();
io::stdin::().read_line(&mut input).expect(“error: unable to read user input”);
println!("{}",input);
}
*/
