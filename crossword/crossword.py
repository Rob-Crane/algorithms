#!/usr/bin/python3
import re

BOARD_SIZE = 10


class Position:
    def __init__(self, row, col):
        self.row = row
        self.col = col


def remaining_positions(start_pos):
    """
    Yield positions from start_pos to end of board.
    """
    i = start_pos.row * BOARD_SIZE + start_pos.col
    while i < BOARD_SIZE * BOARD_SIZE:
        i += 1
        row = i % BOARD_SIZE
        yield Position(row, i - row*BOARD_SIZE)

def Crossword:

    def __init__(self, board):
        self.board = board

    def __getitem__(self, pos):
        return self.board[pos.row][pos.col]

    def __setitem__(self, pos, c):
        return self.board[pos.row][pos.col] = c


def add_word(start_pos, board, words):

    crossword = Crossword(board)
    for pos in remaining_positions(start_pos):
        if crossword[pos] == '-':


    return True
    

# Complete the crosswordPuzzle function below.
def fill_crossword(board, words):
    start_pos = Position(0, 0)
    if !add_word(start_pos, board, words):
        raise RuntimeError('Unable to fill crossword.')

if __name__ == '__main__':
    board = []
    for i in range(BOARD_SIZE):
        board_row = input()
        if len(board_row) != BOARD_SIZE:
            msg = 'Puzzle rows must be {} characters'.format(BOARD_SIZE)
            raise RuntimeError(msg)
        if re.search('[^+-]', board_row):
            raise RuntimeError('Row {} contains invalid character.'.format(i))

        board.append(board_row)

    words = input().split(';')
    if not words:
        raise RuntimeError('No input words.')
    for word in words:
        if re.search('[^A-Z]', word):
            msg = 'Error on word: {}.  Words must be A-Z.'.format(word)
            raise RuntimeError(msg)

    fill_crossword(board, words)
    print('\n'.join(board))


