def main ():
  fizzbuzz = 0
  fizz = 0
  buzz = 0
  for i in range(1, 101):
    if i % 3 == 0 and i % 5 == 0:
      fizzbuzz += 1
      print("FizzBuzz")
    elif i % 3 == 0:
      fizz += 1
      print("Fizz")
    elif i % 5 == 0:
      buzz += 1
      print("Buzz")

  print("found FizzBuzz = {} Fizz = {} Buzz = {}".format(fizzbuzz, fizz, buzz))

main()