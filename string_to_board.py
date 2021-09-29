## 
 #  Author:     Simon Bennett
 #  Date:       2021-03-23
 #  string.py:  Take a string and convert it to an array representing a sudoku
##


def from_string(s):
    """
    This function takes a string and converts it to an array representing a sudoku.
    The string must be 27 characters 0-9..
    Note 0 indicates an empty space
    Form: row ... row
    """
    board = []
    row = []
    n = 0
    for e in s:
        row.append(int(e))
        n += 1
        if n == 9:
            board.append(row)
            n = 0
            row = []

    return board


def board_to_string(board):
    """
    This is a definition
    """
    for r in board:
        print(r)


def is_valid_board(board):
    """
    This function takes an array and determines if the provided array is in the 
    correct form. 
    That is a 9x9 2d array of the form [[row],...,[row]], where all entries are 
    between 0-9 and 0 or null represents an empty entry.
    """
    return True