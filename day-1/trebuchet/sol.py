import sys
input_file = open('input').read().strip()
sum1 = 0
sum2 = 0
for line in input_file.split('\n'):
  part1_sum = []
  part2_sum = []
  for idx,char in enumerate(line):
    if char.isdigit():
      part1_sum.append(char)
      part2_sum.append(char)
    for idx2,val in enumerate(
      [
        'one',
        'two',
        'three',
        'four',
        'five',
        'six',
        'seven',
        'eight',
        'nine'
      ]):
      if line[idx:].startswith(val):
        part2_sum.append(str(idx2+1))
      
  sum1 += int(part1_sum[0]+part1_sum[-1])
  sum2 += int(part2_sum[0]+part2_sum[-1])

print(sum1, sum2)