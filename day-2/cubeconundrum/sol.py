import re
import sys
from functools import reduce
input_file = open('input').read().strip()

LIMIT ={
	"red": 12,
	"green": 13,
	"blue": 14
}

SUCCESS_GAMEIDS_SUM = 0
SUM_OF_PRODUCT = 0

for idx, line in enumerate(input_file.split('\n')):
	game_id = idx + 1
	minimum_balls = {
		"red" : 1,
		"green" : 1,
		"blue" : 1
	}

	games = line.split(':')[-1]
	pattern = re.compile(r'(\d+) (\w+)')
	games_list = games.split(';')
	success = True
	for game in games_list:
		matches = pattern.findall(game)
		result_dict = {color: int(number) for number, color in matches}
		for key, value in result_dict.items():

			# Part 1
			if value > LIMIT[key]:
				success = False
			# Part 2
			if (value > minimum_balls[key]):
				minimum_balls[key] = value

	print(f"End of Game {game_id}")
	mult = reduce(lambda x, y: x*y, minimum_balls.values())
	SUM_OF_PRODUCT += mult
	if success:
		SUCCESS_GAMEIDS_SUM += game_id

# PART 1
print(SUCCESS_GAMEIDS_SUM)

# PART 2
print(SUM_OF_PRODUCT)

"""
2156
66909
"""

