use std::fs::read_to_string;

#[derive(Debug)]
enum ArgumentsStage {
   StartOfFirst,
   First(String),
   StartOfSecond(String),
   Second(String, String)
}

#[derive(Debug)]
enum State {
   Mul (String),
   Arguments (ArgumentsStage),
   Done (i32, i32)
}

const MUL_KEYWORD: &str = "mul(";

impl State {
   fn consume(self, v: char) -> Self {
      
      // Based on the current state...
      match self {
        
        // Start again
        State::Done(_first, _second) => panic!("Invalid parser state"),

        State::Mul (so_far) => {
         
         let next = format!("{}{}", so_far, v);
         
         // Found our keyword?
         if MUL_KEYWORD == next {
            return State::Arguments(ArgumentsStage::StartOfFirst)
         }

         // If this is still the start of our keyword, continue searching for the next character
         if MUL_KEYWORD.starts_with(next.as_str()) {
            return State::Mul(next)
         }

         // Otherwise, reset
         return State::Mul(String::new())
         
        },

        State::Arguments (stage) => {

         // If the character is numeric, add it to the appropriate argument
         match v {
            '0'..='9' => match stage {

               // Start either of the arguments
               ArgumentsStage::StartOfFirst =>
                  State::Arguments(ArgumentsStage::First(String::from(v))),
               ArgumentsStage::StartOfSecond(first) =>
                  State::Arguments(ArgumentsStage::Second(first, String::from(v))),

               // Extend either of the arguments
               ArgumentsStage::First(first) => 
                  State::Arguments(ArgumentsStage::First(format!("{}{}", first, v))),
               ArgumentsStage::Second(first, second) => 
                  State::Arguments(ArgumentsStage::Second(first, format!("{}{}", second, v))),

            },
            ',' => match stage {
               // Switch argument if we're working on the first
               ArgumentsStage::First(first) => State::Arguments(ArgumentsStage::StartOfSecond(first)),

               // Otherwise, reset
               _ => State::Mul(String::new())
            },

            ')' => match stage {
               // Must have some second argument content
               ArgumentsStage::Second(first, second) => State::Done(
                  first.parse().unwrap(),
                  second.parse().unwrap()
               ),

               // Otherwise, reset
               _ => State::Mul(String::new())
            },

            // Otherwise, reset
            _ => State::Mul(String::new())
         }

        }

      }
   }
}


fn main() {

   let mut state = State::Mul(String::new());
   let mut total: i32 = 0;
   
   // Read each character of our input data
   for (index, c) in read_to_string("input.txt").unwrap().chars().enumerate() {
      state = state.consume(c);

      // If we have a complete phrase, add to the total and continue
      match state {
         State::Done(first, second) => {
            let result = first * second;
            total += result;
            println!("Found mul @ index {}, larg={}, rarg={}, result={}, total={}", index, first, second, result, total);
            state = State::Mul(String::new());
         },
         _ => ()
      }
      
   }

   println!("{:?}", state);


}
