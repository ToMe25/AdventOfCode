import os

def recuresiveGame(player1cards, player2cards):
    states = []
    while len(player1cards) > 0 and len(player2cards) > 0:
        state = [player1cards.copy(), player2cards.copy()]
        if state in states:
            return 1, player1cards, player2cards

        states.append(state)

        winner = 0
        if len(player1cards) > player1cards[0] and len(player2cards) > player2cards[0]:
            winner, _, _ = recuresiveGame(player1cards[1:player1cards[0] + 1].copy(), player2cards[1:player2cards[0] + 1].copy())
        elif (player1cards[0] > player2cards[0]):
            winner = 1
        else:
            winner = 2

        if winner == 1:
            player1cards.append(player1cards[0])
            player1cards.append(player2cards[0])
        else:
            player2cards.append(player2cards[0])
            player2cards.append(player1cards[0])

        player1cards = player1cards[1:]
        player2cards = player2cards[1:]

    if len(player1cards) > 0:
        return 1, player1cards, player2cards
    else:
        return 2, player1cards, player2cards

def main():
    input = os.path.dirname(os.getcwd())
    input = os.path.join(input, "input")
    if not os.path.exists(input):
        print(input + " does not exist!")
        input = os.path.join(os.path.dirname(input), "input.txt")
        print(f"trying {input} instead.")
        if not os.path.exists(input):
            print(input + " does not exist!")
            return;

    origPlayer1cards = []
    origPlayer2cards = []
    player2 = False
    for line in open(input, 'r').readlines():
        if (len(line[:-1]) == 0):
            continue

        if line.startswith("Player"):
            if line[-3] == '2':
                player2 = True

            continue

        if player2:
            origPlayer2cards.append(int(line[:-1]))
        else:
            origPlayer1cards.append(int(line[:-1]))

    player1cards = origPlayer1cards.copy()
    player2cards = origPlayer2cards.copy()
    while len(player1cards) > 0 and len(player2cards) > 0:
        if (player1cards[0] > player2cards[0]):
            player1cards.append(player1cards[0])
            player1cards.append(player2cards[0])
        else:
            player2cards.append(player2cards[0])
            player2cards.append(player1cards[0])

        player1cards = player1cards[1:]
        player2cards = player2cards[1:]

    winner = 0
    winningScore = 0
    if len(player1cards) > 0:
        winner = 1
        for i in range(1, len(player1cards) + 1):
            winningScore += player1cards[len(player1cards) - i] * i
    else:
        winner = 2
        for i in range(1, len(player2cards) + 1):
            winningScore += player2cards[len(player2cards) - i] * i

    print(f"Player {winner} won part 1 with a score of {winningScore}.")

    winner, player1cards, player2cards = recuresiveGame(origPlayer1cards.copy(), origPlayer2cards.copy())
    winningScore = 0
    if winner == 1:
        for i in range(1, len(player1cards) + 1):
            winningScore += player1cards[len(player1cards) - i] * i
    else:
        for i in range(1, len(player2cards) + 1):
            winningScore += player2cards[len(player2cards) - i] * i

    print(f"Player {winner} won part 2 with a score of {winningScore}.")

if __name__ == '__main__':
    main()
