function main () {
  const values = Array(100).fill(0).map((_, u) => {
    const i = u + 1
    if (i % 3 === 0 && i % 5 === 0) {
      return 'FizzBuzz'
    } else if (i % 3 === 0) {
      return 'Fizz'
    } else if (i % 5 === 0) {
      return 'Buzz'
    }
  })

  const fizzbuzz = values.filter(v => v === 'FizzBuzz')
  const fizz = values.filter(v => v === 'Fizz')
  const buzz = values.filter(v => v === 'Buzz')

  console.log(`found FizzBuzz = ${fizzbuzz.length} Fizz = ${fizz.length} Buzz = ${buzz.length}`)
}

main()
