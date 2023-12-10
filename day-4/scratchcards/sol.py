import re
all_cards = open("sample").read().splitlines()

def fetch_nums(value):
	return [int(i) for i in re.compile(r'(\d+)').findall(value)]

total = 0
card_number_count = [0] * len(all_cards)

for idx, card in enumerate(all_cards):
	card_no_str = card.split(":")
	card_number_count[idx] +=1
	winning_no, my_no = card_no_str[-1].split('|')
	winning_numbers = set(fetch_nums(winning_no)).intersection(
		set(fetch_nums(my_no)))
	if winning_numbers:
		total_winning_numbers = len(winning_numbers)

		# Part 1
		print(total_winning_numbers)
		total+= (2 ** (total_winning_numbers -1))

		# Part 2
		for idy in range(total_winning_numbers):
				card_number_count[idx + idy + 1] += card_number_count[idx]

print(total)
print(sum(card_number_count))

"""
13
30

26346
8467762
"""
