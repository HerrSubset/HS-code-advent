################################################################################
# Functions
################################################################################
def transform(number_string)
  res = ""
  index = 1
  count = 1
  cur_number = number_string[0]

  while index < number_string.length
    if number_string[index] == cur_number
      #current number equals the previous one
      count += 1

    else
      #handle previous sequence
      res += count.to_s
      res += cur_number.to_s

      #start new sequence
      count = 1
      cur_number = number_string[index]
    end

    index += 1
  end


  #handle last sequence
  res += count.to_s
  res += cur_number.to_s

  return res
end

################################################################################
# Script
################################################################################

puzzle = "1"

20.times do |n|
  puzzle = transform(puzzle)
  puts "step #{n}: #{puzzle}"
end

puts "The output is #{puzzle}"
