require "digest/md5"


################################################################################
# Functions
################################################################################
def five_leading_zeroes?(s, six_zeroes = false)
  res = true

  if s[0] != "0" then res = false end
  if s[1] != "0" then res = false end
  if s[2] != "0" then res = false end
  if s[3] != "0" then res = false end
  if s[4] != "0" then res = false end
  if six_zeroes
    if s[5] != "0" then res = false end
  end

  return res
end

################################################################################
# Script
################################################################################
puts "Enter the puzzle input:"
secret_key = gets.chomp
i = 0

loop do
  md5_input = secret_key + i.to_s
  md5 = Digest::MD5.hexdigest(md5_input)

  if five_leading_zeroes?(md5, true)
    break

  else
    i += 1
  end
end

puts "Your number is #{i}"
