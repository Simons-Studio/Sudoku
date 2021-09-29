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
        print(solutions)

        if len(current) == 2:
            print("Failure: Too many solutions")
        if len(current) == 1:
            r = index // 9
            c = index % 9
            board[r][c] = current[0]
            propagate(r, c, current[0], solutions)
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
        if n_sols == 0:
            continue
        if n_sols < min_s:          # Attempting to find element with minimal solutions
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


def solve_all():
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