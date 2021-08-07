import InputReader


def main():
    lines = InputReader.readInputFileLines(11)
    board = [[c for c in line] for line in lines]

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

    board = [[c for c in line] for line in lines]

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
