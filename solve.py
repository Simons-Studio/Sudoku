## 
 #  Author:     Simon Bennett
 #  Date:       2021-03-23
 #  solve.py:   Take a sudoku as an array and determine a solution if it exists.
##

import numpy as np
from string_to_board import *


def solve(board):
    """
    This function takes a sudoku board array and returns the solution if it exists.
    If there is no unique solution the function will return a null value.

    A sudoku board array should be of the form:
    [[row],...,[row]], where all entries are between 0-9 and 0 or null represents 
    an empty entry.
    """
    # TODO: Create an array that store possible solutions for each element 
    solutions = []
    for e in range(81):
        solutions.append([1,2,3,4,5,6,7,8,9])

    # Initial 
    for r in range(9):
        for c in range(9):
            value = board[r][c]
            if value != 0:
                propagate(r, c, value, solutions)

    b = True
    while(b):
        index = minimal_solution(solutions)
        current = solutions[index]

        if len(current) == 1:
            r = index // 9
            c = index % 9
            board[r][c] = current[0]
            propagate(r, c, current[0], solutions)
        if len(current) == 2:
            pairs = propergate_pairs(solutions, index)
            print("first pair: ", pairs, " values: ", solutions[pairs[0][0]], ", ", solutions[pairs[0][1]])
            b = False
            print("Failure: Too many solutions")

            
        else:
            b = False

    return board


def minimal_solution(solutions):
    """
    Returns the index of the minimal element of "solutions".
    """
    min_s = 9
    minimum = 0
    for i in range(len(solutions)):
        n_sols = len(solutions[i])  # Number of solutions for an element of the sudoku board
        if (0 < n_sols and n_sols < min_s): # Attempting to find element with minimal solutions
            if n_sols == 1:
                return i
            else:
                minimum = i
                min_s = n_sols
    return minimum


def remove(element, arr):
    """
    Removed the first matching element from the list.
    """
    for i in range(len(arr)):
        if arr[i] == element:
            return arr[:i] + arr[i+1:]
    return arr


def propagate(r, c, element, solutions):
    """
    This function propagates a given solution to the solutions list.
    Note: This function does not update the board
    """
    # Clear solutions from solved element
    solutions[9*r+c] = []

    for i in range(9):
        # Row
        solutions[9*r+i] = remove(element, solutions[9*r+i])
        # Column
        solutions[9*i+c] = remove(element, solutions[9*i+c])

    # Box
    box_r_start = (r // 3) * 3
    box_c_start = (c // 3) * 3
    for box_r in range(box_r_start, box_r_start + 3):
        for box_c in range(box_c_start, box_c_start + 3):
            solutions[9 * box_r + box_c] = remove(element, solutions[9 * box_r + box_c])

    return solutions


# Additional Heuristics
def hidden_singles(solutions):
    """
    Find indexes that hold a unique solution in their column/row/box
    """

    return 0


def propergate_pairs (solutions, index=0):
    """
    Find a pair of indexes that have a similar pair of solutions, 
    then propergate the removal of those solutions
    """
    pairs = []

    for i in range(index, len(solutions)):
        if len(solutions[i]) == 2:
            row = i // 9
            col = i % 9

            # check the elements in the row after the current element
            row_end = ((i // 9) + 1) * 9
            for j in range(i + 1, row_end):
                if solutions[i] == solutions[j]:
                    pairs.append([i,j])

            # Check the elements in the column after the current element
            col_start = (row + 1) * 9 + col
            for j in range(col_start, 81, 9):
                if solutions[i] == solutions[j]:
                    pairs.append([i,j])

            # Check the elements in the Box after the current element
            box_r_start = (row // 3) * 3
            box_c_start = (col // 3) * 3
            for box_r in range(row, box_r_start + 3):
                for box_c in range(col + 1, box_c_start + 3):
                    if solutions[i] == solutions[9 * box_r + box_c]:
                        pairs.append([i, 9 * box_r + box_c])

    

    return pairs


def solve_all():
    """
    Takes the problems from 'problems.txt' and iterates solve over the list
    """
    problem_file = open('problems.txt', 'r')
    problems = problem_file.readlines()
    count = 0
    for p in problems:
        count += 1
        b = from_string(p.strip())
        solved = solve(b)
        print("{} :{}".format(count, solved))


def solve_one(s):

    print(board_to_string(solve(from_string(s))))


solve_one("000100702030950000001002003590000301020000070703000098800200100000085060605009000")