
def main () {
  val values = (1 to 100).toList map { i => 
    i match {
      case _ if i % 3 == 0 && i % 5 == 0 => "FizzBuzz"
      case _ if i % 3 == 0 => "Fizz"
      case _ if i % 5 == 0 => "Buzz"
      case _ => Nil
    }
  } 

  val fizzbuzz = values filter { i => i == "FizzBuzz" }
  val fizz = values filter { i => i == "Fizz" }
  val buzz = values filter { i => i == "Buzz" }
  println(s"found FizzBuzz = ${fizzbuzz.size} Fizz = ${fizz.size} Buzz = ${buzz.size}")
}

main()