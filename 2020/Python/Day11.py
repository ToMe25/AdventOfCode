import os
import sys


def main():
    inf = os.path.dirname(os.getcwd())
    inf = os.path.join(inf, "input")
    if not os.path.exists(inf) or os.path.isdir(inf):
        print(inf + " does not exist!")
        inf += ".txt"
        print(f"trying {inf} instead.")

    if not os.path.exists(inf) or os.path.isdir(inf):
        print(inf + " does not exist!")
        inf = os.path.join(os.path.dirname(inf), "input")
        inf = os.path.join(inf, "Day11.txt")
        print(f"trying {inf} instead.")

    if not os.path.exists(inf) or os.path.isdir(inf):
        print("None of the expected inputs exist!", file=sys.stderr)
        return
    else:
        print(f"Using input file {inf}.")

    inputFile = open(inf, 'r')
    board = [[c for c in line[:-1]] for line in inputFile.readlines()]
    inputFile.close()

    lastOccupied = -1
    occupied = 0
    while lastOccupied != occupied:
        lastOccupied = occupied
        oldBoard = [row.copy() for row in board]
        for y in range(len(board)):
            for x in range(len(board[y])):
                if board[y][x] != '.':
                    occupiedNeighbors = 0
                    for i in range(-1, 2):
                        for j in range(-1, 2):
                            if y + i >= 0 and y + i < len(board) and x + j >= 0 and x + j < len(board[y + i]):
                                if oldBoard[y + i][x + j] == '#' and not(i == 0 and j == 0):
                                    occupiedNeighbors += 1

                    if occupiedNeighbors == 0:
                        board[y][x] = '#'
                    elif occupiedNeighbors >= 4:
                        board[y][x] = 'L'

        occupied = sum(row.count('#') for row in board)

    print(f"Found {occupied} occupied seats for part 1.")

    inputFile = open(inf, 'r')
    board = [[c for c in line[:-1]] for line in inputFile.readlines()]
    inputFile.close()

    lastOccupied = -1
    occupied = 0
    while lastOccupied != occupied:
        lastOccupied = occupied
        oldBoard = [row.copy() for row in board]
        for y in range(len(board)):
            for x in range(len(board[y])):
                if board[y][x] != '.':
                    occupiedNeighbors = 0
                    for i in range(-1, 2):
                        for j in range(-1, 2):
                            k = 1
                            while y + i * k >= 0 and y + i * k < len(board) and x + j * k >= 0 and x + j * k < len(board[y + i * k]) and not(i == 0 and j == 0) and board[y + i * k][x + j * k] == '.':
                                k += 1

                            if y + i * k >= 0 and y + i * k < len(board) and x + j * k >= 0 and x + j * k < len(board[y + i * k]) and not(i == 0 and j == 0):
                                if oldBoard[y + i * k][x + j * k] == '#':
                                    occupiedNeighbors += 1

                    if occupiedNeighbors == 0:
                        board[y][x] = '#'
                    elif occupiedNeighbors >= 5:
                        board[y][x] = 'L'

        occupied = sum(row.count('#') for row in board)

    print(f"Found {occupied} occupied seats for part 2.")


if __name__ == '__main__':
    main()
