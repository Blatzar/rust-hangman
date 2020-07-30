use std::io;

fn main() {
	println
	!("Enter word for hangman: ");
	let word = input();
	let mut guessed = Vec::<String>::new();
	guessed.push(" ".to_string());
	clear();
	let mut lives = 10;
	print_letters(&guessed, &word);
	println!("Enter a letter");
	while lives > 0 {
		if lives <= 3 {
			println!("Lives left: {}", lives);
		}
		let guess = input();
		if !word.contains(&guess.to_string()) {
			lives -= 1;
		}
		guessed.push(guess);
		if print_letters(&guessed, &word) {
			println!("Completed!");
			break;
		}
	}
	println!("The word was \"{}\"",&word)
}

fn input() -> std::string::String {
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Failed to get console input");
    return x.trim().to_string();
}

fn print_letters(guessed: &Vec<String>, word: &String) -> bool {
	//println!("correct {:?}", word.contains(last));
	let mut completed = true;
    for i in word.chars() {
        let mut out = "";
        for j in guessed {
            if &i.to_string() == j {
                out = j;
            }

	        if out == "" {
	            out = "_";
	        }
        }
    if out == "_" {completed = false;}
    print!("{} ", out);
    }
    println!("");
    return completed;
}

fn clear(){
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}
